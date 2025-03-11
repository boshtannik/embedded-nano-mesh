#[cfg(test)]
mod tests {
    extern crate std;

    use embedded_nano_mesh::{ms, ExactAddressType, LifeTimeType, Node, NodeConfig, NodeString};
    use proto_lab::{NetworkSimulator, WirelessModemFake};
    use std::time::Instant;

    #[test]
    fn test_send_1_to_1_ignore_duplication() {
        let mut network_simulator = NetworkSimulator::new(1);
        network_simulator.create_ether("1");
        let mut ether = network_simulator.get_ether("1").expect("Can not get ether");

        let mut modem_1 = WirelessModemFake::new("1");
        let mut modem_2 = WirelessModemFake::new("2");

        ether.register_driver(modem_1.clone());
        ether.register_driver(modem_2.clone());

        let mut node_1 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(1).unwrap(),
            listen_period: 10 as ms,
        });

        let mut node_2 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(2).unwrap(),
            listen_period: 20 as ms,
        });

        let _ = node_1.send_to_exact(
            NodeString::from_iter("This is the message from node 1".chars()).into_bytes(),
            ExactAddressType::try_from(2).unwrap(),
            LifeTimeType::try_from(1).unwrap(),
            true,
        );

        network_simulator.start_simulation_thread();

        let start_time = Instant::now();

        loop {
            let current_time = Instant::now().duration_since(start_time).as_millis() as ms;

            let _ = node_1.update(&mut modem_1, current_time);
            let _ = node_2.update(&mut modem_2, current_time);

            if let Some(message) = node_2.receive() {
                let expected = NodeString::from_iter("This is the message from node 1".chars());
                let got = NodeString::from_iter(message.data.iter().map(|c| *c as char));

                assert!(got.starts_with(expected.as_str()));

                break;
            }

            if current_time >= 200 as ms {
                panic!("Simulation timeout");
            }
        }

        network_simulator.stop_simulation_thread();
    }
}
