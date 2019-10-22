use opcua_server::{
    prelude::*,
};
/*
演示如何修改点变量的值来控制服务器的退出
*/
pub fn add_control_switches(server: &mut Server) {
    // The address space is guarded so obtain a lock to change it
    let abort_node_id = NodeId::new(2u16, "abort");

    let address_space = server.address_space();
    let server_state = server.server_state();

    {
        let mut address_space = address_space.write().unwrap();
        let folder_id = address_space
            .add_folder("Control", "Control", &NodeId::objects_folder_id())
            .unwrap();

        VariableBuilder::new(&abort_node_id, "Abort", "Abort")
            .value(false)
            .writable()
            .organized_by(&folder_id)
            .insert(&mut address_space);
    }
    /*
    每1秒检查一次,看看标志是否设置,如果设置为true,则server标记为即将退出
    */
    server.add_polling_action(1000, move || {
        let address_space = address_space.read().unwrap();
        // Test for abort flag
        let abort = if let Ok(v) = address_space.get_variable_value(abort_node_id.clone()) {
            match v.value {
                Some(Variant::Boolean(v)) => v,
                _ => {
                    panic!("Abort value should be true or false");
                }
            }
        } else {
            panic!("Abort value should be in address space");
        };
        // Check if abort has been set to true, in which case abort
        if abort {
            let mut server_state = server_state.write().unwrap();
            server_state.abort();
        }
    });
}
