use std::{io::Write, time::Duration};

use flate2::{write::GzEncoder, Compression};
use futures::{SinkExt, StreamExt};
use tokio::time;
use tokio_tungstenite::{connect_async, tungstenite::Message};

mod config;
mod info;
mod model;

#[tokio::main]
async fn main() {
    let c = config::Config::new();
    let (ws_stream, _) = connect_async(c.url).await.unwrap();
    println!("Connected to the websocket server!");

    let (mut write, mut read) = ws_stream.split();

    write.send(Message::text(c.auth_secret)).await.unwrap();

    let result = match read.next().await {
        None => {
            eprintln!("get empty response");
            return;
        }
        Some(message) => match message {
            Ok(Message::Text(mess)) => mess,
            Err(e) => {
                eprintln!("{e}");
                return;
            }
            _ => return,
        },
    };

    if result == "auth success" {
        println!("success to report")
    }

    let mem_swap = info::get_total_mem_swap();
    let host = model::Host {
        name: c.name,
        mem_total: mem_swap.0,
        swap_total: mem_swap.1,
        arch: "amd64".to_string(),
    };

    let mut interval = time::interval(Duration::from_secs(2));
    loop {
        interval.tick().await;
        let data = model::Data::new()
            .add_host(host.clone())
            .add_host_state(model::HostState {
                cpu: 10.0,
                mem_used: 10,
                swap_used: 10,
            });
        let result = serde_json::to_string(&data).unwrap();
        let mut encoder: GzEncoder<Vec<u8>> = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&result.as_bytes()).unwrap();
        let result = encoder.finish().unwrap();
        write.send(Message::binary(result)).await.unwrap();
    }
}
