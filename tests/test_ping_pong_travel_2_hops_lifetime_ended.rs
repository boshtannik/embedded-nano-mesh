#[cfg(test)]
mod tests {
    extern crate std;

    use embedded_nano_mesh::{ms, ExactAddressType, Node, NodeConfig, NodeString, PacketState};
    use proto_lab::{NetworkSimulator, WirelessModemFake};
    use std::{
        sync::{Arc, Mutex},
        time::Instant,
    };

    const NODE_1_UPDATE_PERIOD: ms = 200;
    const NODE_2_UPDATE_PERIOD: ms = 150;
    const NODE_3_UPDATE_PERIOD: ms = 100;

    const SIMULATION_TIMEOUT: ms = 600;

    #[test]
    fn test_send_ping_pong_travel_2_hops_lifetime_ended() {
        let network_simulator = NetworkSimulator::new(0);

        let mut modem_1 = WirelessModemFake::new("1");
        let mut modem_2 = WirelessModemFake::new("2");
        let mut modem_3 = WirelessModemFake::new("3");

        {
            network_simulator.create_ether("1");
            let mut ether = network_simulator.get_ether("1").expect("Can not get ether");
            ether.register_driver(modem_1.clone());
            ether.register_driver(modem_2.clone());
        }

        {
            network_simulator.create_ether("2");
            let mut ether = network_simulator.get_ether("2").expect("Can not get ether");
            ether.register_driver(modem_2.clone());
            ether.register_driver(modem_3.clone());
        }

        let mut node_1 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(1).unwrap(),
            listen_period: NODE_1_UPDATE_PERIOD as ms,
        });

        let mut node_2 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(2).unwrap(),
            listen_period: NODE_2_UPDATE_PERIOD as ms,
        });

        let mut node_3 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(3).unwrap(),
            listen_period: NODE_3_UPDATE_PERIOD as ms,
        });

        let start_time = Instant::now();

        let update_loop_breaker: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
        let update_loop_breaker_clone = Arc::clone(&update_loop_breaker);

        let pinger_thread = std::thread::spawn(move || {
            let result = match node_1.send_ping_pong(
                NodeString::from_iter("This is the message from node 1".chars()).into_bytes(),
                ExactAddressType::try_from(3).unwrap(),
                2.into(),
                SIMULATION_TIMEOUT as ms,
                || Instant::now().duration_since(start_time).as_millis() as ms,
                &mut modem_1,
            ) {
                Ok(_) => true,
                Err(_) => false,
            };

            *update_loop_breaker_clone
                .lock()
                .expect("Fail to lock update_loop_breaker") = true;

            result
        });

        let start_time = Instant::now();
        let mut is_node_3_pinged = false;

        loop {
            let current_time = Instant::now().duration_since(start_time).as_millis() as ms;

            network_simulator.start_tick();
            network_simulator.simulate();
            network_simulator.end_tick();

            let _ = node_2.update(&mut modem_2, current_time);
            let _ = node_3.update(&mut modem_3, current_time);

            if let Some(message) = node_3.receive() {
                match message.get_spec_state() {
                    PacketState::Ping => is_node_3_pinged = true,
                    _ => (),
                }
            }

            if *update_loop_breaker
                .lock()
                .expect("Fail to lock update_loop_breaker")
            {
                break;
            }

            if current_time >= SIMULATION_TIMEOUT as ms {
                break;
            }
        }

        let is_ping_pong_done = pinger_thread.join().expect("Fail to join pinger thread");

        assert!(is_node_3_pinged);
        assert!(!is_ping_pong_done);
    }
}
