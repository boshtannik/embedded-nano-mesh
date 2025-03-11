#[cfg(test)]
mod tests {
    extern crate std;

    use embedded_nano_mesh::{ms, ExactAddressType, Node, NodeConfig, NodeString};
    use proto_lab::{NetworkSimulator, WirelessModemFake};
    use std::time::Instant;

    const NODE_1_UPDATE_PERIOD: ms = 1;
    const NODE_2_UPDATE_PERIOD: ms = 6;

    const SIMULATION_TIMEOUT: ms = 200;

    #[test]
    fn test_send_ping_pong_simple() {
        let mut network_simulator = NetworkSimulator::new(0);
        network_simulator.create_ether("1");
        let mut ether = network_simulator.get_ether("1").expect("Can not get ether");

        let mut modem_1 = WirelessModemFake::new("1");
        let mut modem_2 = WirelessModemFake::new("2");

        ether.register_driver(modem_1.clone());
        ether.register_driver(modem_2.clone());

        let mut node_1 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(1).unwrap(),
            listen_period: NODE_1_UPDATE_PERIOD as ms,
        });

        let mut node_2 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(2).unwrap(),
            listen_period: NODE_2_UPDATE_PERIOD as ms,
        });

        let start_time = Instant::now();

        let pinger_thread = std::thread::spawn(move || {
            match node_1.send_ping_pong(
                NodeString::from_iter("This is the message from node 1".chars()).into_bytes(),
                ExactAddressType::try_from(2).unwrap(),
                1.into(),
                SIMULATION_TIMEOUT,
                || Instant::now().duration_since(start_time).as_millis() as ms,
                &mut modem_1,
            ) {
                Ok(_) => true,
                Err(_) => false,
            }
        });

        network_simulator.start_simulation_thread();

        let start_time = Instant::now();

        let mut is_node_2_pinged = false;

        let mut break_loop_at = None;

        loop {
            let current_time = Instant::now().duration_since(start_time).as_millis() as ms;

            let _ = node_2.update(&mut modem_2, current_time);

            if let Some(packet) = node_2.receive() {
                is_node_2_pinged = true;

                break_loop_at.replace(current_time + (NODE_2_UPDATE_PERIOD * 6));

                let expected = NodeString::from_iter("This is the message from node 1".chars());
                let got = NodeString::from_iter(packet.data.iter().map(|c| *c as char));

                assert!(got.starts_with(expected.as_str()));
            }

            // As node 2 sill needs some time to respond with pong to node 1
            if let Some(break_loop_at) = break_loop_at {
                if current_time >= break_loop_at {
                    break;
                }
            }

            if current_time >= SIMULATION_TIMEOUT * 3 / 2 as ms {
                panic!("Simulation timeout");
            }
        }

        let is_ping_pong_done = pinger_thread.join().expect("Fail to join pinger thread");

        network_simulator.stop_simulation_thread();

        assert!(is_node_2_pinged);
        assert!(is_ping_pong_done);
    }
}
