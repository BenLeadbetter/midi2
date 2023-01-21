fn main() {
    let (stop_sender, stop): (std::sync::mpsc::Sender<()>, std::sync::mpsc::Receiver<()>) =
        std::sync::mpsc::channel();
    let (start_sender, start): (std::sync::mpsc::Sender<()>, std::sync::mpsc::Receiver<()>) =
        std::sync::mpsc::channel();

    let destination_thread = std::thread::spawn(move || {
        let client = coremidi::Client::new("Destination Client").unwrap();
        let _destination = client.virtual_destination_with_protocol(
            "CoreMIDI Destination",
            coremidi::Protocol::Midi10,
            |event_list| println!("{:?}", event_list),
        );

        start_sender.send(()).expect("signal start sending events");

        wait_on_signal(stop);
    });

    wait_on_signal(start);

    for (i, dst) in coremidi::Destinations.into_iter().enumerate() {
        if let Some(display_name) = dst.display_name() {
            println!("[{}] {}", i, display_name)
        }
    }

    let destination = coremidi::Destination::from_index(0).unwrap();
    println!(
        "Destination display name: {}",
        destination.display_name().unwrap()
    );

    let client = coremidi::Client::new("Example Client").unwrap();
    let output_port = client.output_port("Example Port").unwrap();

    let note_on =
        coremidi::EventBuffer::new(coremidi::Protocol::Midi10).with_packet(0, &[0x2090407f]);
    let note_off =
        coremidi::EventBuffer::new(coremidi::Protocol::Midi10).with_packet(0, &[0x2080407f]);

    for i in 0..10 {
        println!("[{}] Sending note ...", i);

        output_port.send(&destination, &note_on).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1000));

        output_port.send(&destination, &note_off).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    stop_sender
        .send(())
        .expect("signal destination thread to stop");
    destination_thread
        .join()
        .expect("destination thread finished");
}

fn wait_on_signal(receiver: std::sync::mpsc::Receiver<()>) {
    loop {
        if let Ok(()) = receiver.recv() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
}
