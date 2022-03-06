use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{AccountId, Balance};
use std::collections::HashMap;
use std::vec::Vec;

pub type TokenId = u32;
pub type AuctionId = u32;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Auction {
    pub auction_id: AuctionId,
    pub token_id: TokenId,
    pub owner_id: AccountId,
    pub started_price: Balance,
    pub created_at: u64,
    pub is_active: bool,
    pub participants: HashMap<AccountId, Balance>,
    pub winner: AccountId,
    pub closed_price: Balance,
}

impl Auction {
    pub fn calculate_min_amount(&self) -> Balance {
        let prices: Vec<&Balance> = self.participants.values().collect();

        let min_price = prices.iter().min();

        match min_price {
            Some(&min_price) => *min_price,
            None => 0,
        }
    }

    pub fn find_winner(&mut self, price: Balance) {
        let winner = self
            .participants
            .iter()
            .find_map(|(key, &val)| {
                if val == price {
                    Some(key.clone())
                } else {
                    None
                }
            })
            .unwrap();
        self.winner = winner;
    }
}
