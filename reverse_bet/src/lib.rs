//! This contract implements a reverse auciton by storage on Near blockchain
//!
//! The contract provives methods to [mint_nft] / [create_auction] / [close_auction] / [bid]
//! [get current info auctions, tokens]

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise};
use std::collections::HashMap;

near_sdk::setup_alloc!();

mod auction;
mod nft;

use auction::{Auction, AuctionId, TokenId};
use nft::Token;

const TRANSFER_FEE: Balance = 1_000_000_000_000_000_000_000;
const MINT_FEE: Balance = 1_00_000_000_000_000_000_000_000; // 0.1 Near
const CREATE_AUCTION_FEE: Balance = 1_00_000_000_000_000_000_000_000; // 0.1 Near

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub owner_id: AccountId,
    pub token_id: TokenId,
    pub auction_id: AuctionId,
    pub auction_by_id: UnorderedMap<AuctionId, Auction>,
    pub auction_by_owner: UnorderedMap<AccountId, Vec<AuctionId>>,
    pub token_by_id: UnorderedMap<TokenId, Token>,
    pub auction_going_on: Vec<AuctionId>,
    pub auction_closed: Vec<AuctionId>,
}

#[near_bindgen]
impl Contract {
    /// Initial smart contract
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            token_id: 0,
            auction_id: 0,
            auction_by_id: UnorderedMap::new(b"auction_by_id".to_vec()),
            auction_by_owner: UnorderedMap::new(b"auction_by_owner".to_vec()),
            token_by_id: UnorderedMap::new(b"token_by_id".to_vec()),
            auction_going_on: Vec::new(),
            auction_closed: Vec::new(),
        }
    }

    /// Mint a token
    #[payable]
    pub fn mint_nft(&mut self, owner_id: AccountId) {
        let amount = env::attached_deposit();
        assert_eq!(amount, MINT_FEE, "Need 0.1N to mint nft");

        self.token_id += 1;
        let token = Token {
            owner_id: owner_id,
            token_id: self.token_id,
            is_active: false,
        };

        self.token_by_id.insert(&self.token_id, &token);
    }

    /// Transfer token to new account
    #[private]
    fn transfer_nft(&mut self, new_owner_id: AccountId, token_id: TokenId) {
        assert!(
            env::is_valid_account_id(new_owner_id.as_bytes()),
            "Account receiver is not valid"
        );

        let mut token = self.token_by_id.get(&token_id).unwrap();

        let sender = env::predecessor_account_id();
        if sender != self.owner_id && sender != token.owner_id {
            env::panic("You can't transfer NFT".as_bytes());
        }

        token.transfer(new_owner_id.clone());
        self.token_by_id.insert(&token_id, &token);
    }

    /// Create an anction to bid
    #[payable]
    pub fn create_auction(&mut self, token_id: TokenId, price: Balance) {
        let amount = env::attached_deposit();
        assert_eq!(amount, CREATE_AUCTION_FEE, "Need 0.1N to create an auction");
        let mut token = self.token_by_id.get(&token_id).unwrap();

        let account_id = env::predecessor_account_id();
        assert_eq!(token.owner_id, account_id, "You don't owner this token");
        assert_eq!(token.is_active, false, "This token is active");

        self.auction_id += 1;
        let auction = Auction {
            auction_id: self.auction_id,
            token_id: token.token_id,
            owner_id: account_id,
            started_price: price * 1_000_000_000_000_000_000_000_000,
            created_at: env::block_timestamp(),
            is_active: true,
            participants: HashMap::new(),
            winner: AccountId::new(),
            closed_price: 0,
        };

        token.is_active = true;
        self.token_by_id.insert(&token_id, &token);
        self.auction_going_on.push(self.auction_id);
        self.auction_by_id.insert(&self.auction_id, &auction);
    }

    // Close auction and reveal amount
    #[payable]
    pub fn close_auction(&mut self, auction_id: AuctionId) {
        let mut auction = self.auction_by_id.get(&auction_id).unwrap_or_else(|| {
            env::panic("This auction does not exists!".as_bytes());
        });

        let account_id = env::predecessor_account_id();

        if account_id != self.owner_id && account_id != auction.owner_id {
            env::panic("You can't close auction".as_bytes());
        }

        auction.is_active = false;
        let min_price = auction.calculate_min_amount();
        auction.find_winner(min_price);

        auction.closed_price = min_price;
        self.transfer_ft_to_seller(auction.owner_id.clone(), min_price);

        let participants = auction
            .participants
            .clone()
            .into_iter()
            .filter(|&(_, val)| val != min_price)
            .collect();

        self.transfer_ft_back_to_participants(participants);

        self.transfer_nft(auction.winner.clone(), auction.token_id);
        self.auction_by_id.insert(&auction_id, &auction);

        self.auction_going_on.retain(|&x| x != auction_id);
        self.auction_closed.push(auction_id);
    }

    /// Bid to auction
    #[payable]
    pub fn bid(&mut self, auction_id: AuctionId) {
        let mut auction = self.auction_by_id.get(&auction_id).unwrap_or_else(|| {
            env::panic("This auction does not exists!".as_bytes());
        });

        assert_eq!(auction.is_active, true, "This auction is not active");

        let amount = env::attached_deposit();
        let account_id = env::predecessor_account_id();

        assert_eq!(
            amount < auction.started_price,
            true,
            "The price must be lower current price"
        );

        assert!(
            !auction.participants.contains_key(&account_id),
            "You have Already bid"
        );
        let mut token = self.token_by_id.get(&auction.token_id).unwrap();
        token.is_active = false;
        self.token_by_id.insert(&auction.token_id, &token);

        auction.participants.insert(account_id, amount);
        self.auction_by_id.insert(&auction_id, &auction);
    }

    pub fn get_all_auctions_going_on(&self) -> Vec<AuctionId> {
        self.auction_going_on.clone()
    }

    pub fn get_all_auctions_closed(&self) -> Vec<AuctionId> {
        self.auction_closed.clone()
    }

    pub fn get_all_tokens(&self) -> Vec<TokenId> {
        self.token_by_id.iter().map(|(k, _)| k).collect()
    }

    pub fn get_auction_by_id(&self, auction_id: AuctionId) -> Option<Auction> {
        self.auction_by_id.get(&auction_id)
    }

    pub fn get_token_by_id(&self, token_id: TokenId) -> Option<Token> {
        self.token_by_id.get(&token_id)
    }

    #[private]
    fn transfer_ft_back_to_participants(&self, _participants: HashMap<AccountId, Balance>) {
        for (account, balance) in _participants.iter() {
            let account = Promise::new(account.clone());
            account.transfer(balance - TRANSFER_FEE);
        }
    }

    #[private]
    fn transfer_ft_to_seller(&self, _owner_id: AccountId, _auction_price: Balance) {
        let account = Promise::new(_owner_id.into());
        account.transfer(_auction_price);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};
    use near_sdk::{AccountId, Balance, Timestamp};
    fn bob() -> AccountId {
        "bob.testnet".to_string()
    }
    fn senna() -> AccountId {
        "senna.testnet".to_string()
    }

    fn get_context(
        _account_id: String,
        storage_usage: u64,
        block_timestamp: Timestamp,
        attached_deposit: Balance,
    ) -> VMContext {
        VMContext {
            current_account_id: _account_id.clone(),
            signer_account_id: _account_id.clone(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: _account_id,
            input: vec![],
            block_index: 0,
            block_timestamp,
            account_balance: 100_000_000_000_000_000_000_000_000_000,
            account_locked_balance: 0,
            storage_usage,
            attached_deposit,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }
    #[test]
    fn test_mint_nft() {
        let context = get_context(senna(), 0, 0, MINT_FEE);
        testing_env!(context);
        let mut contract = Contract::new(senna());
        contract.mint_nft(senna());
        contract.mint_nft(senna());
        testing_env!(get_context(bob(), env::storage_usage(), 0, MINT_FEE));
        contract.mint_nft(bob());
        assert_eq!(contract.token_by_id.get(&1).unwrap().owner_id, senna(), "");
        assert_eq!(contract.token_by_id.get(&2).unwrap().owner_id, senna(), "");
        assert_eq!(contract.token_by_id.get(&3).unwrap().owner_id, bob(), "");
    }
    #[test]
    fn test_transfer_nft() {
        let context = get_context(senna(), 0, 0, MINT_FEE);
        testing_env!(context);
        let mut contract = Contract::new(senna());
        contract.mint_nft(senna());
        contract.mint_nft(senna());
        contract.transfer_nft(bob(), 2);
        assert_eq!(contract.token_by_id.get(&2).unwrap().owner_id, bob(), "");
    }
    #[test]
    fn test_create_close_auction() {
        let context = get_context(senna(), 0, 0, MINT_FEE);
        testing_env!(context);
        let mut contract = Contract::new(senna());
        contract.mint_nft(bob());
        contract.mint_nft(bob());
        testing_env!(get_context(
            bob(),
            env::storage_usage(),
            0,
            CREATE_AUCTION_FEE
        ));
        let price: Balance = 5;
        contract.create_auction(1, price);
        contract.create_auction(2, price);
        contract.bid(1);
        assert_eq!(contract.auction_by_id.get(&1).unwrap().is_active, true, "");
        assert_eq!(contract.token_by_id.get(&2).unwrap().owner_id, bob(), "");
        contract.close_auction(1);
        assert_eq!(contract.auction_by_id.get(&1).unwrap().is_active, false, "");
    }
}
