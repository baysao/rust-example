use async_std::task;
use env_logger;
use jsonrpc_http_server::jsonrpc_core::IoHandler;
use jsonrpc_http_server::ServerBuilder;
use rand::prelude::*;
use rust_example::node_p2p;
use rust_example::node_rpc::{PokemonRpcImpl, Rpc};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::{error::Error, thread};
use tokio::sync::mpsc;

#[async_std::main]
//#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let mut rng = rand::thread_rng();

    //Grpc server
    let port: u16;
    if let Some(arg) = std::env::args().nth(1) {
        port = arg.parse()?;
    } else {
        port = rng.gen_range(50051..50100)
    }
    println!("Rpc port: {}", port);
    let grpc_socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
    let (request_sender, request_receiver) = mpsc::unbounded_channel();
    let jsonrpc_handle = thread::spawn(move || {
        let mut io = IoHandler::default();
        let pokemon_service = PokemonRpcImpl {
            sender: request_sender,
        };
        io.extend_with(pokemon_service.to_delegate());

        let server = ServerBuilder::new(io)
            .threads(3)
            .start_http(&grpc_socket)
            .unwrap();
        server.wait();
    });

    let p2p_port = rng.gen_range(40001..40100);
    let p2p_addr = format!("{}{}", "/ip4/0.0.0.0/tcp/", p2p_port);
    let p2p_handle = thread::spawn(move || {
        let swarm = task::block_on(node_p2p::create_swarm(&p2p_addr)).unwrap();
        let p2p_node = node_p2p::init_node(swarm, request_receiver);
        task::block_on(p2p_node).unwrap();
    });

    p2p_handle.join().unwrap();
    jsonrpc_handle.join().unwrap();
    Ok(())
}
