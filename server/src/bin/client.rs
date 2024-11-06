// src/bin/client.rs
use compute::prelude::*;
use s2n_quic::{client::Connect, Client};
use server::util::extract;
use server::util::prepare;
use std::{error::Error, net::SocketAddr, path::Path};
use tracing::debug;
use tracing::info;

#[encrypted(compile)]
fn average(a: u128, b: u128, c: u128) -> u128 {
    let res = a + b + c;
    res / 3
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt::init();

    let client = Client::builder()
        .with_tls(Path::new("server/certs/cert.pem"))?
        .with_io("0.0.0.0:0")?
        .start()?;

    let addr: SocketAddr = "127.0.0.1:4433".parse()?;
    let connect = Connect::new(addr).with_server_name("localhost");
    let mut connection = client.connect(connect).await?;

    // Ensure the connection doesn't time out with inactivity
    connection.keep_alive(true)?;

    // Open a new stream and split the receiving and sending sides
    let mut stream = connection.open_bidirectional_stream().await?;
    //let (mut receive_stream, mut send_stream) = stream.split();

    // Initialize garbler with sample data
    let a = 10_u128;
    let b = 20_u128;
    let c = 30_u128;

    let (circuit, input_garbler) = average(a, b, c);
    info!("Circuit: {:?}", hex::encode(circuit.blake3_hash()));

    let (mut garbler, mut msg_for_evaluator) = GatewayGarbler::start(&circuit, &input_garbler)?;

    // Send initial message from garbler to evaluator
    stream
        .send(prepare(msg_for_evaluator.clone()).into())
        .await?;
    //stream.flush().await?;

    info!(
        "Message for evaluator, length: {:?}",
        msg_for_evaluator.len()
    );

    while let Some(first_chunk) = stream.receive().await? {
        let (size, mut data) = extract(&first_chunk)?;
        info!("size from header: {:?}", size);

        // Continue receiving until the full message is obtained
        while data.len() < size as usize {
            let chunk = stream.receive().await?.expect("no data received");
            data.extend_from_slice(&chunk);
        }

        info!("Received message, length: {:?}", data.len());
        debug!("Received data: {:?}", hex::encode(&data));

        // Pass evaluator response to garbler "next" function
        let (next_garbler, next_message) = garbler.next(&data)?;
        info!("Steps remaining: {}", next_garbler.steps());
        garbler = next_garbler;

        info!("Sending message, length: {:?}", next_message.len());
        // Send the next message back to evaluator
        stream.send(prepare(next_message.clone()).into()).await?;
        //stream.flush().await?;

        debug!(
            "Sent message for garbler: {:?}",
            hex::encode(msg_for_evaluator.clone())
        );

        msg_for_evaluator = next_message;

        if garbler.is_complete() {
            // Receive the final output from the evaluator
            let final_output = stream.receive().await?.expect("no data received");
            info!("Final output received: {:?}", hex::encode(&final_output));
            break;
        }
    }

    debug!("last message: {:?}", msg_for_evaluator);
    println!("Garbler has completed the interaction.");

    Ok(())
}
