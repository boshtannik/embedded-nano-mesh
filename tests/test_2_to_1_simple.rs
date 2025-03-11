#[cfg(test)]
mod tests {
    extern crate std;

    use embedded_nano_mesh::{ms, ExactAddressType, Node, NodeConfig, NodeString};
    use proto_lab::{NetworkSimulator, WirelessModemFake};
    use std::time::Instant;

    #[test]
    fn test_send_2_to_1_simple() {
        let network_simulator = NetworkSimulator::new(0);
        network_simulator.create_ether("1");

        let mut modem_1 = WirelessModemFake::new("1");
        let mut modem_2 = WirelessModemFake::new("2");
        let mut modem_3 = WirelessModemFake::new("3");

        {
            let mut ether = network_simulator.get_ether("1").expect("Can not get ether");

            ether.register_driver(modem_1.clone());
            ether.register_driver(modem_2.clone());
            ether.register_driver(modem_3.clone());
        }

        let mut node_1 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(1).unwrap(),
            listen_period: 150 as ms,
        });

        let mut node_2 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(2).unwrap(),
            listen_period: 160 as ms,
        });

        let mut node_3 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(3).unwrap(),
            listen_period: 170 as ms,
        });

        let _ = node_1.send_to_exact(
            NodeString::from_iter("This is the message from node 1".chars()).into_bytes(),
            ExactAddressType::try_from(2).unwrap(),
            1.into(),
            false,
        );

        let _ = node_3.send_to_exact(
            NodeString::from_iter("This is the message from node 3".chars()).into_bytes(),
            ExactAddressType::try_from(2).unwrap(),
            1.into(),
            false,
        );

        let start_time = Instant::now();

        let mut expected_messages_count = 2;

        loop {
            let current_time = Instant::now().duration_since(start_time).as_millis() as ms;

            network_simulator.start_tick();
            network_simulator.simulate();
            network_simulator.end_tick();

            let _ = node_1.update(&mut modem_1, current_time);
            let _ = node_2.update(&mut modem_2, current_time);
            let _ = node_3.update(&mut modem_3, current_time);

            if let Some(message) = node_2.receive() {
                let expected_1 = NodeString::from_iter("This is the message from node 1".chars());
                let expected_2 = NodeString::from_iter("This is the message from node 3".chars());

                let got = NodeString::from_iter(message.data.iter().map(|c| *c as char));

                if got.starts_with(expected_1.as_str()) {
                    expected_messages_count -= 1;
                }

                if got.starts_with(expected_2.as_str()) {
                    expected_messages_count -= 1;
                }
            }

            if expected_messages_count == 0 {
                break;
            }

            if current_time >= 200 as ms {
                panic!("Simulation timeout");
            }
        }
    }
}
