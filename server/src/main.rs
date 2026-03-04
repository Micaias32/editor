use editor_core::*;
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener,
};

fn main() -> anyhow::Result<()> {
    let addr = "127.0.0.1:32001";

    let listener = TcpListener::bind(addr)?;

    println!("Server listening on: {addr}");

    let mut state = Buffer::from_str("hi\nhihi\nhihihi", 1);

    for stream in listener.incoming() {
        let stream = stream?;

        handle_client(&mut state, stream)?;
    }

    Ok(())
}

fn handle_client(state: &mut Buffer, mut stream: std::net::TcpStream) -> anyhow::Result<()> {
    let reader = BufReader::new(stream.try_clone()?);

    for line in reader.lines() {
        let line = line?;

        println!("recieved `{}`", line);

        let client_event: ClientEvent = serde_json::from_str(&line)?;

        let update = match client_event {
            ClientEvent::KeyPress(combo) => handle_key_press(state, combo),
            ClientEvent::InitialRequest => Some(ServerUpdate::BufferNew(state.clone())),
        };

        let update = match update {
            Some(u) => u,
            None => continue,
        };

        writeln!(stream, "{}", serde_json::to_string(&update)?)?;
        if let ServerUpdate::CloseBuffer { .. } = update {
            break;
        }
    }
    Ok(())
}

fn handle_key_press(state: &mut Buffer, combo: String) -> Option<ServerUpdate> {
    match combo.as_str() {
        "q" => Some(ServerUpdate::CloseBuffer {
            buffer_id: state.id(),
        }),

        &_ => None,
    }
}
