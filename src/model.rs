use serde_derive::{Deserialize, Serialize};
use std::sync::mpsc::Sender;

#[derive(Deserialize, Serialize, Debug)]
pub struct Pokemon {
    pub name: String,
    pub color: String,
    pub eye_num: u32,
    pub nose_num: u32,
    pub mouth_num: u32,
}

#[derive(Debug)]
pub enum Errors {
    NoSuchPerson,
}

#[derive(Debug)]
pub enum Commands {
    // PutPokemon(String, String),
    StorePokemon(Pokemon),
    GetPokemon(String, Sender<Responses>),
}
#[derive(Debug)]
pub enum Responses {
    Success(),
    Error(),
    GotPokemon(String),
}
