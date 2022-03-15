#!/bin/bash
export MAIN_ACCOUNT=duonghb2.testnet
export NEAR_ENV=testnet
export ACCOUNT_ID=$MAIN_ACCOUNT
export CONTRACT_ID=duonghb_ft.$MAIN_ACCOUNT
export BOB_ID=bob.$MAIN_ACCOUNT
export GAS_BASE=200000000000000
export YOCTO_NEAR=0.000000000000000000000001

#####################################
# - Deploy Sample Smart Contract tạo 1 fungible token
# - Ví dụ:
#   B1: Deploy ft 
#   B2: Init new ft
#   B3: Get the fungible token metadata
#   B4: Check Balance of Bob Token
#   B5: Transfer Token
#   B6: Check Balance of My account 


################## B1: Deploy contract ##################
echo "###################### Build Contract #####################"
./build.sh

echo "################### DELETE ACCOUNT ###################"
near delete $CONTRACT_ID $ACCOUNT_ID
near delete $BOB_ID $ACCOUNT_ID

echo "################### CREATE ACCOUNT ###################"
near create-account $CONTRACT_ID --masterAccount $ACCOUNT_ID --initialBalance 10
near create-account $BOB_ID --masterAccount $ACCOUNT_ID --initialBalance 10

echo "################### CREATE CONTRACT ###################"
near deploy $CONTRACT_ID --accountId $ACCOUNT_ID --wasmFile ./res/fungible_token.wasm
###################### End B1: Deploy contract #####################

################## B2: Init new ft ##################
echo "################### B2: Init new ft ###################"
near call $CONTRACT_ID new '{"owner_id": "'$CONTRACT_ID'", "total_supply": "1000000000", "metadata": { "spec": "ft-1.0.0", "name": "DuongHB Token", "symbol": "HBD", "decimals": 18 }}' --accountId $ACCOUNT_ID
################## End B2: Init new ft ##################

################## B3: Get the fungible token metadata ##################
echo "################### B3: GET METADATA ###################"
near view $CONTRACT_ID ft_metadata
################## End B3: Get the fungible token metadata ##################


################## B4: Check Balance of Bob Token ##################
echo "################### B4: CHECK BALANCE OF BOB ###################"
near view $CONTRACT_ID ft_balance_of '{"account_id": "'$BOB_ID'"}'
################## End B4: Check Balance of Bob Token ##################


################## B5: Transfer Token ##################
echo "################### B5: CHECK USED GAS ###################"
near call $CONTRACT_ID ft_transfer '{"receiver_id": "'$BOB_ID'", "amount": "19"}' --accountId $ACCOUNT_ID --amount 0.000000000000000000000001
################## End B5: Transfer Token ##################

################## B6: Check Balance of My account ##################
echo "################### B6: CHECK PREPAID GAS ###################"
near view $CONTRACT_ID ft_balance_of '{"account_id": "'$ACCOUNT_ID'"}'
################## End B6: Check Balance of My account ##################
