#[cfg(enm_address_type_u8)]
#[cfg(test)]
mod tests {
    extern crate std;

    use embedded_nano_mesh::{ms, ExactAddressType, LifeTimeType, Node, NodeConfig, NodeString};
    use embedded_nano_mesh_v2::{
        ms as ms_v2,
        ExactAddressType as ExactAddressTypeV2,
        LifeTimeType as LifeTimeTypeV2,
        Node as NodeV2,
        NodeConfig as NodeConfigV2,
        NodeString as NodeStringV2,
    };
    use proto_lab::{NetworkSimulator, WirelessModemFake};
    use std::{
        sync::{Arc, Mutex},
        time::Instant,
    };

    fn v3_to_v2_text() -> embedded_nano_mesh::PacketDataBytes {
        NodeString::from_iter("Hello from v3 node".chars()).into_bytes()
    }

    fn v2_to_v3_text() -> embedded_nano_mesh_v2::PacketDataBytes {
        NodeStringV2::from_iter("Hello from v2 node".chars()).into_bytes()
    }

    fn run_v3_to_v2_send_to_exact() -> bool {
        let mut network_simulator = NetworkSimulator::new(1);
        network_simulator.create_ether("1");
        let mut ether = network_simulator.get_ether("1").expect("Can not get ether");

        let mut modem_1 = WirelessModemFake::new("1");
        let mut modem_2 = WirelessModemFake::new("2");

        ether.register_driver(modem_1.clone());
        ether.register_driver(modem_2.clone());

        let mut node_v3 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(1).unwrap(),
            listen_period: 10 as ms,
        });

        let mut node_v2 = NodeV2::new(NodeConfigV2 {
            device_address: ExactAddressTypeV2::try_from(2).unwrap(),
            listen_period: 20 as ms_v2,
        });

        let _ = node_v3.send_to_exact(
            v3_to_v2_text(),
            ExactAddressType::try_from(2).unwrap(),
            LifeTimeType::try_from(1).unwrap(),
            true,
        );

        network_simulator.start_simulation_thread();

        let start_time = Instant::now();

        loop {
            let current_time = Instant::now().duration_since(start_time).as_millis() as ms;

            let _ = node_v3.update(&mut modem_1, current_time);
            let _ = node_v2.update(&mut modem_2, current_time as ms_v2);

            if let Some(packet) = node_v2.receive() {
                let expected = NodeStringV2::from_iter("Hello from v3 node".chars());
                let got = NodeStringV2::from_iter(packet.data.iter().map(|c| *c as char));
                let result = got.starts_with(expected.as_str());
                network_simulator.stop_simulation_thread();
                return result;
            }

            if current_time >= 300 as ms {
                network_simulator.stop_simulation_thread();
                return false;
            }
        }
    }

    fn run_v2_to_v3_send_to_exact() -> bool {
        let mut network_simulator = NetworkSimulator::new(1);
        network_simulator.create_ether("1");
        let mut ether = network_simulator.get_ether("1").expect("Can not get ether");

        let mut modem_1 = WirelessModemFake::new("1");
        let mut modem_2 = WirelessModemFake::new("2");

        ether.register_driver(modem_1.clone());
        ether.register_driver(modem_2.clone());

        let mut node_v2 = NodeV2::new(NodeConfigV2 {
            device_address: ExactAddressTypeV2::try_from(1).unwrap(),
            listen_period: 10 as ms_v2,
        });

        let mut node_v3 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(2).unwrap(),
            listen_period: 20 as ms,
        });

        let _ = node_v2.send_to_exact(
            v2_to_v3_text(),
            ExactAddressTypeV2::try_from(2).unwrap(),
            LifeTimeTypeV2::try_from(1).unwrap(),
            true,
        );

        network_simulator.start_simulation_thread();

        let start_time = Instant::now();

        loop {
            let current_time = Instant::now().duration_since(start_time).as_millis() as ms;

            let _ = node_v2.update(&mut modem_1, current_time as ms_v2);
            let _ = node_v3.update(&mut modem_2, current_time);

            if let Some(packet) = node_v3.receive() {
                let expected = NodeString::from_iter("Hello from v2 node".chars());
                let got = NodeString::from_iter(packet.data.iter().map(|c| *c as char));
                let result = got.starts_with(expected.as_str());
                network_simulator.stop_simulation_thread();
                return result;
            }

            if current_time >= 300 as ms {
                network_simulator.stop_simulation_thread();
                return false;
            }
        }
    }

    fn run_v3_to_v2_broadcast() -> bool {
        let mut network_simulator = NetworkSimulator::new(1);
        network_simulator.create_ether("1");
        let mut ether = network_simulator.get_ether("1").expect("Can not get ether");

        let mut modem_1 = WirelessModemFake::new("1");
        let mut modem_2 = WirelessModemFake::new("2");

        ether.register_driver(modem_1.clone());
        ether.register_driver(modem_2.clone());

        let mut node_v3 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(1).unwrap(),
            listen_period: 10 as ms,
        });

        let mut node_v2 = NodeV2::new(NodeConfigV2 {
            device_address: ExactAddressTypeV2::try_from(2).unwrap(),
            listen_period: 20 as ms_v2,
        });

        let _ = node_v3.broadcast(v3_to_v2_text(), LifeTimeType::try_from(2).unwrap());

        network_simulator.start_simulation_thread();

        let start_time = Instant::now();

        loop {
            let current_time = Instant::now().duration_since(start_time).as_millis() as ms;

            let _ = node_v3.update(&mut modem_1, current_time);
            let _ = node_v2.update(&mut modem_2, current_time as ms_v2);

            if let Some(packet) = node_v2.receive() {
                let expected = NodeStringV2::from_iter("Hello from v3 node".chars());
                let got = NodeStringV2::from_iter(packet.data.iter().map(|c| *c as char));
                let result = got.starts_with(expected.as_str());
                network_simulator.stop_simulation_thread();
                return result;
            }

            if current_time >= 300 as ms {
                network_simulator.stop_simulation_thread();
                return false;
            }
        }
    }

    fn run_v2_to_v3_broadcast() -> bool {
        let mut network_simulator = NetworkSimulator::new(1);
        network_simulator.create_ether("1");
        let mut ether = network_simulator.get_ether("1").expect("Can not get ether");

        let mut modem_1 = WirelessModemFake::new("1");
        let mut modem_2 = WirelessModemFake::new("2");

        ether.register_driver(modem_1.clone());
        ether.register_driver(modem_2.clone());

        let mut node_v2 = NodeV2::new(NodeConfigV2 {
            device_address: ExactAddressTypeV2::try_from(1).unwrap(),
            listen_period: 10 as ms_v2,
        });

        let mut node_v3 = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(2).unwrap(),
            listen_period: 20 as ms,
        });

        let _ = node_v2.broadcast(v2_to_v3_text(), LifeTimeTypeV2::try_from(2).unwrap());

        network_simulator.start_simulation_thread();

        let start_time = Instant::now();

        loop {
            let current_time = Instant::now().duration_since(start_time).as_millis() as ms;

            let _ = node_v2.update(&mut modem_1, current_time as ms_v2);
            let _ = node_v3.update(&mut modem_2, current_time);

            if let Some(packet) = node_v3.receive() {
                let expected = NodeString::from_iter("Hello from v2 node".chars());
                let got = NodeString::from_iter(packet.data.iter().map(|c| *c as char));
                let result = got.starts_with(expected.as_str());
                network_simulator.stop_simulation_thread();
                return result;
            }

            if current_time >= 300 as ms {
                network_simulator.stop_simulation_thread();
                return false;
            }
        }
    }

    fn run_v3_transit_through_v2_relay() -> bool {
        let mut network_simulator = NetworkSimulator::new(1);
        network_simulator.create_ether("1");
        let mut ether = network_simulator.get_ether("1").expect("Can not get ether");

        let mut modem_1 = WirelessModemFake::new("1");
        let mut modem_2 = WirelessModemFake::new("2");
        let mut modem_3 = WirelessModemFake::new("3");

        ether.register_driver(modem_1.clone());
        ether.register_driver(modem_2.clone());
        ether.register_driver(modem_3.clone());

        let mut node_v3_src = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(1).unwrap(),
            listen_period: 10 as ms,
        });

        let mut node_v2_relay = NodeV2::new(NodeConfigV2 {
            device_address: ExactAddressTypeV2::try_from(2).unwrap(),
            listen_period: 20 as ms_v2,
        });

        let mut node_v3_dst = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(3).unwrap(),
            listen_period: 30 as ms,
        });

        let _ = node_v3_src.send_to_exact(
            v3_to_v2_text(),
            ExactAddressType::try_from(3).unwrap(),
            LifeTimeType::try_from(3).unwrap(),
            true,
        );

        network_simulator.start_simulation_thread();

        let start_time = Instant::now();

        loop {
            let current_time = Instant::now().duration_since(start_time).as_millis() as ms;

            let _ = node_v3_src.update(&mut modem_1, current_time);
            let _ = node_v2_relay.update(&mut modem_2, current_time as ms_v2);
            let _ = node_v3_dst.update(&mut modem_3, current_time);

            if let Some(packet) = node_v3_dst.receive() {
                let expected = NodeString::from_iter("Hello from v3 node".chars());
                let got = NodeString::from_iter(packet.data.iter().map(|c| *c as char));
                let result = got.starts_with(expected.as_str());
                network_simulator.stop_simulation_thread();
                return result;
            }

            if current_time >= 400 as ms {
                network_simulator.stop_simulation_thread();
                return false;
            }
        }
    }

    fn run_v2_ping_pong_through_v3_relay() -> bool {
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

        let mut node_v2_src = NodeV2::new(NodeConfigV2 {
            device_address: ExactAddressTypeV2::try_from(1).unwrap(),
            listen_period: 100 as ms_v2,
        });

        let mut node_v3_relay = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(2).unwrap(),
            listen_period: 133 as ms,
        });

        let mut node_v2_dst = NodeV2::new(NodeConfigV2 {
            device_address: ExactAddressTypeV2::try_from(3).unwrap(),
            listen_period: 172 as ms_v2,
        });

        let start_time = Instant::now();

        let update_loop_breaker: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
        let update_loop_breaker_clone = Arc::clone(&update_loop_breaker);

        let pinger_thread = std::thread::spawn(move || {
            let result = match node_v2_src.send_ping_pong(
                NodeStringV2::from_iter("ping through v3 relay".chars()).into_bytes(),
                ExactAddressTypeV2::try_from(3).unwrap(),
3u8.into(),
                600 as ms_v2,
                || Instant::now().duration_since(start_time).as_millis() as ms_v2,
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

        loop {
            let current_time = Instant::now().duration_since(start_time).as_millis() as ms;

            network_simulator.start_tick();
            network_simulator.simulate();
            network_simulator.end_tick();

            let _ = node_v3_relay.update(&mut modem_2, current_time);
            let _ = node_v2_dst.update(&mut modem_3, current_time as ms_v2);

            if *update_loop_breaker
                .lock()
                .expect("Fail to lock update_loop_breaker")
            {
                break;
            }

            if current_time >= 600 as ms {
                break;
            }
        }

        pinger_thread.join().expect("Fail to join pinger thread")
    }

    fn run_v2_transaction_through_v3_relay() -> bool {
        let network_simulator = NetworkSimulator::new(1);

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

        let mut node_v2_src = NodeV2::new(NodeConfigV2 {
            device_address: ExactAddressTypeV2::try_from(1).unwrap(),
            listen_period: 200 as ms_v2,
        });

        let mut node_v3_relay = Node::new(NodeConfig {
            device_address: ExactAddressType::try_from(2).unwrap(),
            listen_period: 102 as ms,
        });

        let mut node_v2_dst = NodeV2::new(NodeConfigV2 {
            device_address: ExactAddressTypeV2::try_from(3).unwrap(),
            listen_period: 153 as ms_v2,
        });

        let start_time = Instant::now();

        let update_loop_breaker: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
        let update_loop_breaker_clone = Arc::clone(&update_loop_breaker);

        let transaction_thread = std::thread::spawn(move || {
            let result = match node_v2_src.send_with_transaction(
                NodeStringV2::from_iter("transaction through v3 relay".chars()).into_bytes(),
                ExactAddressTypeV2::try_from(3).unwrap(),
5u8.into(),
                1200 as ms_v2,
                || Instant::now().duration_since(start_time).as_millis() as ms_v2,
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

        loop {
            let current_time = Instant::now().duration_since(start_time).as_millis() as ms;

            network_simulator.start_tick();
            network_simulator.simulate();
            network_simulator.end_tick();

            let _ = node_v3_relay.update(&mut modem_2, current_time);
            let _ = node_v2_dst.update(&mut modem_3, current_time as ms_v2);

            if *update_loop_breaker
                .lock()
                .expect("Fail to lock update_loop_breaker")
            {
                break;
            }

            if current_time >= 1200 as ms {
                break;
            }
        }

        transaction_thread
            .join()
            .expect("Fail to join transaction thread")
    }

    #[test]
    fn test_version_compatibility_matrix() {
        let scenarios: [(&str, &str, bool); 7] = [
            (
                "send_to_exact",
                "v3 -> v2",
                run_v3_to_v2_send_to_exact(),
            ),
            (
                "send_to_exact",
                "v2 -> v3",
                run_v2_to_v3_send_to_exact(),
            ),
            ("broadcast", "v3 -> v2", run_v3_to_v2_broadcast()),
            ("broadcast", "v2 -> v3", run_v2_to_v3_broadcast()),
            (
                "send_to_exact through relay",
                "v3 -> v2 -> v3",
                run_v3_transit_through_v2_relay(),
            ),
            (
                "ping-pong through relay",
                "v2 -> v3 -> v2",
                run_v2_ping_pong_through_v3_relay(),
            ),
            (
                "transaction through relay",
                "v2 -> v3 -> v2",
                run_v2_transaction_through_v3_relay(),
            ),
        ];

        println!();
        println!("| Scenario | Topology | Status |");
        println!("|----------|----------|:------:|");
        for (scenario, topology, is_ok) in &scenarios {
            println!(
                "| {} | {} | {} |",
                scenario,
                topology,
                if *is_ok { "pass" } else { "fail" }
            );
        }

        let expected: [bool; 7] = [
            true,  // send_to_exact v3 -> v2
            true,  // send_to_exact v2 -> v3
            true,  // broadcast v3 -> v2
            true,  // broadcast v2 -> v3
            true,  // send_to_exact through relay v3 -> v2 -> v3
            true,  // ping-pong through relay v2 -> v3 -> v2
            false, // transaction through relay v2 -> v3 -> v2
        ];

        for ((scenario, topology, is_ok), expected_ok) in scenarios.iter().zip(expected.iter()) {
            assert_eq!(
                is_ok, expected_ok,
                "Compatibility changed for '{}' ({}): update the expected matrix and the README",
                scenario, topology
            );
        }
    }
}
