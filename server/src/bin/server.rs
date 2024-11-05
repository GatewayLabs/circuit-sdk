use compute::prelude::*;
use s2n_quic::Server;
use server::util::extract;
use server::util::prepare;
use std::{error::Error, path::Path};
use tracing::{debug, error, info, instrument};

#[encrypted(compile)]
fn multi_arithmetic(a: u8, b: u8, c: u8, d: u8) -> u8 {
    let res = a * b;
    let res = res + c;
    res - d
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt::init();

    // Setup server
    let mut server = Server::builder()
        .with_tls((
            Path::new("server/certs/cert.pem"),
            Path::new("server/certs/key.pem"),
        ))?
        .with_io("127.0.0.1:4433")?
        .start()?;

    info!("Server started and listening on 127.0.0.1:4433");

    while let Some(mut connection) = server.accept().await {
        info!("Accepted new connection");

        // Spawn a new task for the connection
        tokio::spawn(async move {
            while let Ok(Some(stream)) = connection.accept_bidirectional_stream().await {
                info!("Accepted bidirectional stream");

                // Initialize the evaluator instance with circuit and dummy input
                let (circuit, _) = multi_arithmetic(0_u8, 0_u8, 0_u8, 0_u8);

                info!("Circuit: {:?}", hex::encode(circuit.blake3_hash()));

                let evaluator =
                    GatewayEvaluator::new(&circuit, &[]).expect("Evaluator initialization failed");

                if let Err(e) = handle_evaluator_connection(evaluator, stream).await {
                    error!("Error handling evaluator connection: {:?}", e);
                }
            }
        });
    }

    Ok(())
}

#[instrument(skip_all, fields(evaluator, stream))]
async fn handle_evaluator_connection(
    mut evaluator: GatewayEvaluator,
    mut stream: s2n_quic::stream::BidirectionalStream,
) -> Result<(), Box<dyn Error>> {
    info!("Starting evaluator connection handler");

    while let Some(first_chunk) = stream.receive().await? {
        let (size, mut data) = extract(&first_chunk)?;
        info!("size from header: {:?}", size);

        // Continue receiving until the full message is obtained
        while data.len() < size as usize {
            let chunk = stream.receive().await?.expect("no data received");
            data.extend_from_slice(&chunk);
        }

        info!("Received data, length: {:?}", data.len());
        debug!("Received data: {:?}", hex::encode(&data));

        // Check if the evaluator has completed all steps
        if evaluator.is_complete() {
            info!("Evaluator processing complete, preparing output");
            let output_message = evaluator.output(&data)?;
            let mut output_data = Vec::new();
            for bit in output_message {
                output_data.push(if bit { 1 } else { 0 });
            }
            stream.send(prepare(output_data).into()).await?;
            info!("Final output sent to client, closing connection");
            break;
        }

        // Process the evaluator "next" step with received data
        let (next_evaluator, response) = evaluator.next(&data)?;
        info!("Steps remaining: {}", next_evaluator.steps());
        evaluator = next_evaluator;

        // Send back the response
        info!("Sending data, length: {:?}", response.len());
        debug!("Sending data: {:?}", hex::encode(&response));
        stream.send(prepare(response).into()).await?;
        info!("Sent response back to client");
    }

    debug!("Evaluator connection handler complete");
    Ok(())
}
