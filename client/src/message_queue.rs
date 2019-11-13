use std::collections::{HashSet, HashMap};

use futures::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use opcua_types::*;
/**
这个单纯就是一个管理消息发送与接收的辅助结构
记录发送,
收到以后交到这里,如果是没有对应的发送消息,会被丢弃
对于有对应发送消息的存下来,等待使用的人来取,
消息分为两大类: 异步消息和非异步消息
*/
pub(crate) struct MessageQueue {
    /// The requests that are in-flight, defined by their request handle and an async flag. Basically,
    /// the sent requests reside here until the response returns at which point the entry is removed.
    /// If a response is received for which there is no entry, the response will be discarded.
    inflight_requests: HashSet<(u32, bool)>, //记录还没有处理的消息,正在处理的,这是一个set,其中u32是消息id,bool是表示异步还是同步
    /// A map of incoming responses waiting to be processed
    responses: HashMap<u32, (SupportedMessage, bool)>,
    /// This is the queue that messages will be sent onto the transport for sending
    sender: Option<UnboundedSender<Message>>, //真正存放发送消息的地方
}

pub enum Message {
    Quit,
    SupportedMessage(SupportedMessage),
}

impl MessageQueue {
    pub fn new() -> MessageQueue {
        MessageQueue {
            inflight_requests: HashSet::new(),
            responses: HashMap::new(),
            sender: None,
        }
    }

    pub(crate) fn clear(&mut self) {
        self.inflight_requests.clear();
        self.responses.clear();
    }

    // Creates the transmission queue that outgoing requests will be sent over
    //todo 这个api 感觉设计的并不合理,可能会反复创建sender,receiver,是没必要的,应该是初始化的时候就构造好
    /// 自己保留了一份sender用于发送正常消息,返回的则是用于其他人发送退出信号
    pub(crate) fn make_request_channel(&mut self) -> (UnboundedSender<Message>, UnboundedReceiver<Message>) {
        let (tx, rx) = mpsc::unbounded::<Message>();
        self.sender = Some(tx.clone());
        (tx, rx)
    }

    pub(crate) fn request_was_processed(&mut self, request_handle: u32) {
        debug!("Request {} was processed by the server", request_handle);
    }

    fn send_message(&mut self, message: Message) {
        if let Err(err) = self.sender.as_ref().unwrap().unbounded_send(message) {
            debug!("Cannot sent message to message receiver, error = {:?}", err);
        }
        trace!("send_message unbounded_send complete");
    }

    /// Called by the session to add a request to be sent
    pub(crate) fn add_request(&mut self, request: SupportedMessage, is_async: bool) {
        let request_handle = request.request_handle();
        trace!("Sending request {:?} to be sent", request);
        self.inflight_requests.insert((request_handle, is_async));
        self.send_message(Message::SupportedMessage(request));
    }

    pub(crate) fn quit(&mut self) {
        debug!("Sending a quit to the message receiver");
        self.send_message(Message::Quit);
    }

    /// Called when a session's request times out. This call allows the session state to remove
    /// the request as pending and ignore any response that arrives for it.
    pub(crate) fn request_has_timed_out(&mut self, request_handle: u32) {
        info!("Request {} has timed out and any response will be ignored", request_handle);
        let _ = self.inflight_requests.remove(&(request_handle, false));
        let _ = self.inflight_requests.remove(&(request_handle, true));
    }

    /// Called by the connection to store a response for the consumption of the session.
    pub(crate) fn store_response(&mut self, response: SupportedMessage) {
        // Remove corresponding request handle from inflight queue, add to responses
        let request_handle = response.request_handle();
        debug!("Response to Request {} has been stored", request_handle);
        // Remove the inflight request
        // This true / false is slightly clunky.
        if let Some(request) = self.inflight_requests.take(&(request_handle, true)) {
            self.responses.insert(request_handle, (response, request.1));
        } else if let Some(request) = self.inflight_requests.take(&(request_handle, false)) {
            self.responses.insert(request_handle, (response, request.1));
        } else {
            error!("A response with request handle {} doesn't belong to any request and will be ignored, inflight requests = {:?}", request_handle, self.inflight_requests);
        }
    }

    /// Takes all pending asynchronous responses into a vector sorted oldest to latest and
    /// returns them to the caller.
    pub(crate) fn async_responses(&mut self) -> Vec<SupportedMessage> {
        // Gather up all request handles
        let mut async_handles = self.responses.iter()
            .filter(|(_, v)| v.1)
            .map(|(k, _)| *k)
            .collect::<Vec<_>>();

        // Order them from oldest to latest (except if handles wrap)
        async_handles.sort();

        // Remove each item from the map and return to caller
        async_handles.iter()
            .map(|k| self.responses.remove(k).unwrap().0)
            .collect()
    }

    /// Called by the session to take the identified response if one exists, otherwise None
    pub(crate) fn take_response(&mut self, request_handle: u32) -> Option<SupportedMessage> {
        self.responses.remove(&request_handle).map(|v| v.0)
    }
}