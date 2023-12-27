use chrono::{DateTime, Local};
use rand::prelude::Distribution;
use serde::{Deserialize, Serialize};

pub struct AlphaSpaceNemeric;

impl Distribution<u8> for AlphaSpaceNemeric {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZbcdefghijklmnopqrstuvwxyz0123456789  ";
        loop {
            let var = rng.next_u32() >> (32 - 6);
            if var < CHARSET.len() as u32 {
                return CHARSET[var as usize];
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Row {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub reg_date: DateTime<Local>,
    pub mod_date: Option<DateTime<Local>>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Order {
    ASC,
    DESC,
}

impl Default for Order {
    fn default() -> Self {
        Self::ASC
    }
}

impl From<String> for Order {
    fn from(value: String) -> Self {
        match value.to_uppercase().as_str() {
            "ASC" => Self::ASC,
            "DESC" => Self::DESC,
            _ => Self::default(),
        }
    }
}
