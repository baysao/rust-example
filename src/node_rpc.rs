use crate::model::{Commands, Pokemon, Responses};
use jsonrpc_core::{Params, Result};
use jsonrpc_derive::rpc;
use std::sync::mpsc::channel;
use tokio::sync::mpsc::UnboundedSender;

#[rpc]
pub trait Rpc {
    #[rpc(name = "put")]
    fn put(&self, params: Params) -> Result<String>;

    #[rpc(name = "get")]
    fn get(&self, name: String) -> Result<Pokemon>;
}

pub struct PokemonRpcImpl {
    pub sender: UnboundedSender<Commands>,
}
impl Rpc for PokemonRpcImpl {
    fn put(&self, params: Params) -> Result<String> {
        let pokemon: Pokemon = params.parse().unwrap();
        match self.sender.send(Commands::StorePokemon(pokemon)) {
            Ok(_) => Ok(String::from("Ok")),
            Err(_) => Ok(String::from("Error")),
        }
    }

    fn get(&self, name: String) -> Result<Pokemon> {
        let (response_sender, response_receiver) = channel();
        let key = name.clone();
        match self.sender.send(Commands::GetPokemon(key, response_sender)) {
            Ok(_) => {
                let pokemon = match response_receiver.recv() {
                    Ok(res) => match res {
                        Responses::GotPokemon(content) => {
                            let res: Pokemon = serde_json::from_str(&content).unwrap();
                            res
                        }
                        _ => Pokemon {
                            name,
                            color: "".to_string(),
                            eye_num: 0,
                            nose_num: 0,
                            mouth_num: 0,
                        },
                    },
                    Err(_) => Pokemon {
                        name,
                        color: "".to_string(),
                        eye_num: 0,
                        nose_num: 0,
                        mouth_num: 0,
                    },
                };
                Ok(pokemon)
            }
            Err(_) => Ok(Pokemon {
                name,
                color: "".to_string(),
                eye_num: 0,
                nose_num: 0,
                mouth_num: 0,
            }),
        }
    }
}
