//! This simple OPC UA client will do the following:
//!
//! 1. Read a configuration file (either default or the one specified using --config)
//! 2. Connect & create a session on one of those endpoints that match with its config (you can override which using --endpoint-id arg)
//! 3. Subscribe to values and loop forever printing out their values
use std::sync::{Arc, RwLock};
#[macro_use]
extern crate log;
use crate::log::*;
use clap::{App, Arg};

use opcua_client::prelude::*;

fn main() {
    // Read command line arguments
    let m = App::new("Simple OPC UA Client")
        .arg(Arg::with_name("url")
            .long("url")
            .help("Specify the OPC UA endpoint to connect to")
            .takes_value(true)
            .default_value("opc.tcp://188.0.0.101:4840")
//            .default_value("opc.tcp://127.0.0.1:4855")
            .required(false))
        .get_matches();
    let url = m.value_of("url").unwrap().to_string();

    // Optional - enable OPC UA logging
    opcua_console_logging::init();

    // Make the client configuration
    let mut client = ClientBuilder::new()
        .application_name("Simple Client")
        .application_uri("urn:SimpleClient")
        .trust_server_certs(true)
        .create_sample_keypair(true)
        .session_retry_limit(3)
        .client().unwrap();

    match client.connect_to_endpoint((url.as_ref(),
                                      SecurityPolicy::None.to_str(),
                                      MessageSecurityMode::None, UserTokenPolicy::anonymous()),
                                     IdentityToken::Anonymous) {
        Ok(session) => {
            match subscribe_to_variables(session.clone()) {
                Err(result) => {
                    println!("ERROR: Got an error while subscribing to variables - {}", result);
                }
                Ok(..) => { //这种写法清晰么? err return 不更好么
                    // Loops forever. The publish thread will call the callback with changes on the variables
                    /*
                    测试读写
                    */
                    let cs1state = NodeId::new(4, "|var|Chongqing-ARM-Linux.Application.SystemGVL.CS_1_State");
                    match session.write().unwrap().read(&[cs1state.clone().into()]){
                        Err(result)=>{
                            error!("error read {}",cs1state);
                        }
                        Ok(result)=>{
                            info!("read {} value={:?}",cs1state,result);
                        }
                    }
                    let am601=NodeId::new(4,"|var|Chongqing-ARM-Linux.Application.TempGVL.AM10601");
                    let am601w=    WriteValue {
                        node_id: am601.clone(),
                        attribute_id:AttributeId::Value as u32,
                        index_range: UAString::null(),
                        value:DataValue::new(300.0 as f32),
                    };
                    match session.write().unwrap().write(&[am601w ]){
                        Err(result)=>{
                            error!("error read {}",am601);
                        }
                        Ok(result)=>{
                            info!("read {} value={:?}",am601,result);
                        }
                    }
                    match session.write().unwrap().read(&[am601.clone().into() ]){
                        Err(result)=>{
                            error!("error read {}",am601);
                        }
                        Ok(result)=>{
                            info!("read {} value={:?}",am601,result);
                        }
                    }
                    let _ = Session::run(session);
                }
            }
        }
        Err(s) => {
            println!("connect error {}",s)
        }
    }
}

fn subscribe_to_variables(session: Arc<RwLock<Session>>) -> Result<(), StatusCode> {
    let mut session = session.write().unwrap();
    // Creates a subscription with a data change callback 间隔是两秒
    let subscription_id = session.create_subscription(2000.0, 10, 30, 0, 0, true, DataChangeCallback::new(|changed_monitored_items| {
        println!("Data change from server:");
        changed_monitored_items.iter().for_each(|item| print_value(item));
    }))?;
    info!("Created a subscription with id = {}", subscription_id);

    // Create some monitored items
    let items_to_create: Vec<MonitoredItemCreateRequest> = ["v1", "v2", "v3", "v4"].iter()
        .map(|v|{
         let n=   NodeId::new(2, *v);
            info!("nodeid={}",n);
         n.into()
        }).collect();
    let _ = session.create_monitored_items(subscription_id, TimestampsToReturn::Both, &items_to_create)?;

    Ok(())
}

fn print_value(item: &MonitoredItem) {
    let node_id = &item.item_to_monitor().node_id;
    let data_value = item.value();
    if let Some(ref value) = data_value.value {
        info!("Item \"{}\", Value = {:?}", node_id, value);
    } else {
        info!("Item \"{}\", Value not found, error: {}", node_id, data_value.status.as_ref().unwrap());
    }
}