use actix::prelude::*;
use std::{env, io, time::SystemTime};

/// A payload with a counter
#[derive(Debug, Message)]
#[rtype(result = "()")]
enum Payload {
    Basic { id: usize },
    InnerPayloadType2,
}

#[derive(Debug)]
struct Node {
    id: usize,
    limit: usize,
    next: Recipient<Payload>,
}

impl Actor for Node {
    type Context = Context<Self>;
}

impl Handler<Payload> for Node {
    type Result = ();

    fn handle(&mut self, msg: Payload, _: &mut Context<Self>) {
        match msg {
            Payload::Basic { id } => {
                if id >= self.limit {
                    println!(
                        "Actor {} reached limit of {} (payload was {})",
                        self.id, self.limit, id
                    );

                    System::current().stop();
                    return;
                }

                println!(
                    "Actor {} received message {} of {}",
                    self.id, id, self.limit
                );
                self.next.do_send(Payload::Basic{id: id + 1});
            },
            Payload::InnerPayloadType2 => {

            }
        }
    }
}

fn main() -> io::Result<()> {
    let sys = System::new();

    let (n_nodes, n_rounds) = parse_args();

    let now = SystemTime::now();

    sys.block_on(async {
        println!("Setting up {} nodes", n_nodes);
        let limit = n_nodes * n_rounds;

        let node = Node::create(move |ctx| {
            let first_addr = ctx.address();

            let node = Node {
                id: 1,
                limit,
                next: first_addr.recipient(),
            };
            println!("Node_1: {:?}", node);
            let mut prev_addr = node.start();

            for id in 2..n_nodes {
                let node = Node {
                    id,
                    limit,
                    next: prev_addr.recipient(),
                };
                println!("Node_{}: {:?}", id, node);
                prev_addr = node.start();
            }

            let node = Node {
                id: n_nodes,
                limit,
                next: prev_addr.recipient(),
            };
            println!("Node_{}: {:?}", n_nodes, node);
            return node;
        });

        println!(
            "Sending start message and waiting for termination after {} messages...",
            limit
        );

        node.send(Payload::InnerPayloadType2).await.unwrap();
        node.send(Payload::Basic{id: 1}).await.unwrap();
    });

    sys.run().unwrap();

    match now.elapsed() {
        Ok(elapsed) => println!(
            "Time taken: {}.{:06} seconds ({} msg/second)",
            elapsed.as_secs(),
            elapsed.subsec_micros(),
            (n_nodes * n_rounds * 1000000) as u128 / elapsed.as_micros()
        ),
        Err(e) => println!("An error occurred: {:?}", e),
    }

    Ok(())
}

fn parse_args() -> (usize, usize) {
    let mut args = env::args();

    if let Some(app_name) = args.next() {
        let n_nodes = args
            .next()
            .and_then(|val| val.parse::<usize>().ok())
            .unwrap_or_else(|| print_usage_and_exit(&app_name));

        if n_nodes <= 1 {
            eprintln!("Number of nodes must be > 1");
            ::std::process::exit(1);
        }

        let n_rounds = args
            .next()
            .and_then(|val| val.parse::<usize>().ok())
            .unwrap_or_else(|| print_usage_and_exit(&app_name));

        if args.next().is_some() {
            print_usage_and_exit(&app_name);
        }
        return (n_nodes, n_rounds);
    }
    return (0, 0);
}

fn print_usage_and_exit(app_name: &String) -> ! {
    eprintln!("Usage:");
    eprintln!("{} - <num-nodes> <num-times-message-around-ring>", app_name);
    eprintln!("cargo run -- <num-nodes> <num-times-message-around-ring>");

    ::std::process::exit(1);
}
