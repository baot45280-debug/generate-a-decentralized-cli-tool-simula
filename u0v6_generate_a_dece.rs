// u0v6_generate_a_dece.rs

use clap::{App, Arg};
use tokio;

mod sim {
    pub struct Simulator {
        pub nodes: Vec<String>,
        pub tx_queue: Vec<String>,
    }

    impl Simulator {
        pub async fn new(nodes: Vec<String>) -> Self {
            Simulator {
                nodes,
                tx_queue: vec![],
            }
        }

        pub async fn add_node(&mut self, node: String) {
            self.nodes.push(node);
        }

        pub async fn send_tx(&mut self, tx: String) {
            self.tx_queue.push(tx);
        }

        pub async fn run(&mut self) {
            tokio::spawn(async move {
                for node in self.nodes.clone() {
                    println!("Simulating node: {}", node);
                }
            });

            tokio::spawn(async move {
                for tx in self.tx_queue.clone() {
                    println!("Processing tx: {}", tx);
                }
            });
        }
    }
}

#[tokio::main]
async fn main() {
    let matches = App::new("Decentralized CLI Tool Simulator")
        .version("0.1")
        .author("Your Name")
        .about("Simulates a decentralized CLI tool")
        .arg(
            Arg::with_name("nodes")
                .long("nodes")
                .short("n")
                .multiple(true)
                .takes_value(true)
                .required(true)
                .help("Node addresses"),
        )
        .arg(
            Arg::with_name("txs")
                .long("txs")
                .short("t")
                .multiple(true)
                .takes_value(true)
                .help("Transactions to simulate"),
        )
        .get_matches();

    let nodes = matches.values_of("nodes").unwrap().map(|x| x.to_string()).collect();
    let txs = matches.values_of("txs").map(|x| x.map(|x| x.to_string()).collect());

    let mut sim = sim::Simulator::new(nodes).await;

    if let Some(txs) = txs {
        for tx in txs {
            sim.send_tx(tx).await;
        }
    }

    sim.run().await;
}