#[cfg(test)]
mod tests {
    extern crate std;

    use embedded_nano_mesh::{ms, ExactAddressType, Node, NodeConfig, NodeString};
    use proto_lab::{NetworkSimulator, WirelessModemFake};
    use std::time::Instant;

    const NODE_1_UPDATE_PERIOD: ms = 5;
    const NODE_2_UPDATE_PERIOD: ms = 10;

    const SIMULATION_TIMEOUT: ms = 200;

    #[test]

    fn test_send_to_self_ignore_duplication() {
        let network_simulator = NetworkSimulator::new(0);
        network_simulator.create_ether("1");

        let mut modem_1 = WirelessModemFake::new("1");
        let mut modem_2 = WirelessModemFake::new("2");

        {
            let mut ether = network_simulator.get_ether("1").expect("Can not get ether");

            ether.register_driver(modem_1.clone());
            ether.register_driver(modem_2.clone());
        }

        let mut node_1 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(1).unwrap(),
            listen_period: NODE_1_UPDATE_PERIOD,
        });

        let mut node_2 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(2).unwrap(),
            listen_period: NODE_2_UPDATE_PERIOD,
        });

        let _ = node_1.send_to_exact(
            NodeString::from_iter("This is the message from self".chars()).into_bytes(),
            ExactAddressType::try_from(1).unwrap(),
            2.into(),
            true,
        );

        let start_time = Instant::now();

        loop {
            let current_time = Instant::now().duration_since(start_time).as_millis() as ms;

            network_simulator.start_tick();
            network_simulator.simulate();
            network_simulator.end_tick();

            let _ = node_1.update(&mut modem_1, current_time);
            let _ = node_2.update(&mut modem_2, current_time);

            if let Some(packet) = node_1.receive() {
                let expected = NodeString::from_iter("This is the message from self".chars());
                let got = NodeString::from_iter(packet.data.iter().map(|c| *c as char));

                assert!(got.starts_with(expected.as_str()));
                break;
            }

            if current_time >= SIMULATION_TIMEOUT as ms {
                panic!("Simulation timeout");
            }
        }
    }
}
