use hermes_mavlink::examples::*;
use mavlink_core::{connect, MavHeader};
use std::{
    thread,
    time::{Duration, SystemTime},
};

/// This example sets up a basic MAVLink communication system using two separate threads:
/// 1. The "Ground Station (GS)" thread connects to a MAVLink router and continuously
///    sends heartbeat messages on a TCP connection without termination.
/// 2. The "Router" thread listens for incoming MAVLink messages and logs them to the console.
///
/// This program will run indefinitely until manually terminated.
fn main() {
    let gs_thread = thread::spawn(move || {
        let router = connect::<MavMessage>("tcpout:127.0.0.1:5760")
            .expect("Failed to connect to MAVLink GS");

        println!("[GS] Connected to MAVLink router");

        let mut sequence: u8 = 1;
        loop {
            let now = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap();
            let heartbeat = MavMessage::HEARTBEAT(HEARTBEAT_DATA {
                system_type: MavSystemType::ROCKET,
                uptime: now.as_millis() as u64,
            });
            let header = MavHeader {
                system_id: 200,  // The sending system (GS uses high values)
                component_id: 1, // The sending component (non-standard)
                sequence,        // The component should increment this once per sent message
            };
            sequence = sequence.wrapping_add(1);
            match router.send(&header, &heartbeat) {
                Ok(_) => println!("Heartbeat sent."),
                Err(e) => println!("Error sending heartbeat: {e:?}"),
            }
            thread::sleep(Duration::from_millis(100));
        }
    });

    let router_thread = thread::spawn(move || {
        let gs = connect::<MavMessage>("tcpin:127.0.0.1:5760")
            .expect("Failed to connect to MAVLink router");

        println!("[Router] New node connected");
        loop {
            match gs.recv() {
                Ok((header, msg)) => {
                    println!("Received message: header: {header:?}, msg: {msg:?}");
                }
                Err(e) => {
                    println!("Error receiving message: {e:?}");
                }
            }
        }
    });

    gs_thread.join().expect("GS thread panicked");
    router_thread.join().expect("Router thread panicked");
}
