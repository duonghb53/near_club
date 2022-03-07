
# Detailed Installation / Quickstart
## If you don't have Rust
Install Rust https://rustup.rs/
## If you have never used near-cli
1. Install near-cli: `npm i -g near-cli`
2. Create testnet account: [Wallet](https://wallet.testnet.near.org)
3. Login: `near login`

## Build
 >./build.sh
 
## Demo
### 1. Setup Account
> ID_MASTER=your_master_acc.testnet
> 
> ID1=sub_acc1.$ID_MASTER
> 
> ID2=sub_acc2.$ID_MASTER
> 
> ID3=sub_acc3.$ID_MASTER

#### Create: 
> near create-account $ID1 --masterAccount $ID_MASTER --initialBalance 10
> 
> near create-account $ID2 --masterAccount $ID_MASTER --initialBalance 10
> 
> near create-account $ID3 --masterAccount $ID_MASTER --initialBalance 10

#### Delete:
> near delete $ID1 $ID_MASTER
> 
> near delete $ID2 $ID_MASTER
> 
> near delete $ID3 $ID_MASTER

### 2. Check state account: 
> near state $ID_MASTER | grep -E "(Account|formattedAmount)"
> 
> near state $ID1 | grep -E "(Account|formattedAmount)"
> 
> near state $ID2 | grep -E "(Account|formattedAmount)"
> 
> near state $ID3 | grep -E "(Account|formattedAmount)"

### 3. Deploy contract:
> near deploy --accountId $ID_MASTER --wasmFile ./res/your_contract.wasm

#### 4. Check storage:
> near call $ID_MASTER get_all_auctions_going_on --accountId $ID_MASTER
> 
> near call $ID_MASTER get_all_auctions_closed --accountId $ID_MASTER
> 
> near call $ID_MASTER get_auction_by_id '{"auction_id": 1}' --accountId $ID_MASTER
> 
> near call $ID_MASTER get_all_tokens --accountId $ID_MASTER
> 
> near call $ID_MASTER get_token_by_id '{"token_id": 1}' --accountId $ID_MASTER

#### 5. Bet
#### Create Instant Contract:
> near call $ID_MASTER new '{"owner_id": "'$ID_MASTER'"}' --accountId $ID_MASTER

##### Mint NFT:
> near call $ID_MASTER mint_nft '{"owner_id": "'$ID_MASTER'"}' --accountId $ID_MASTER --deposit 0.1

#### Create auction:
> near call $ID_MASTER create_auction '{"token_id": 1, "price": 5}' --accountId $ID_MASTER --deposit 0.1

#### Bid:
> near call $ID_MASTER bid '{"auction_id": 2}' --accountId $ID1 --deposit 3.5
> 
> near call $ID_MASTER bid '{"auction_id": 2}' --accountId $ID2 --deposit 3
> 
> near call $ID_MASTER bid '{"auction_id": 2}' --accountId $ID3 --deposit 4.5

##### Close auction:
> near call $ID_MASTER close_auction '{"auction_id": 2}' --accountId $ID_MASTER
