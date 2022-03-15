#!/bin/bash
export MAIN_ACCOUNT=duonghb2.testnet
export NEAR_ENV=testnet
export ACCOUNT_ID=$MAIN_ACCOUNT
export CONTRACT_ID=cross_contract.$MAIN_ACCOUNT
export GAS_BASE=200000000000000

#####################################
# - Deploy Sample Smart Contract sử dụng Cross-Contract
# - Ví dụ:
#   B1: Deploy cross contract check white list
#   B2: Thực hiện call kiểm tra account mike.testnet có trong whitelist từ trong smart contract white.demo.testnet không? Sử dụng callback_promise_result
#   B3: Thực hiện call kiểm tra account mike.testnet có trong whitelist từ trong smart contract white.demo.testnet không? Sử dụng callback_arg_macro
#   B4: Check used gas
#   B5: Check prepaid gas    

################## B1: Deploy contract ##################
echo "###################### Build Contract #####################"
./build.sh

echo "################### DELETE ACCOUNT ###################"
near delete $CONTRACT_ID $ACCOUNT_ID

echo "################### CREATE ACCOUNT ###################"
near create-account $CONTRACT_ID --masterAccount $ACCOUNT_ID --initialBalance 10

echo "################### CREATE CONTRACT ###################"
near deploy $CONTRACT_ID --accountId $ACCOUNT_ID --wasmFile ./res/cross_contract.wasm
###################### End B1: Deploy contract #####################

################## B2: Check whitelist contract ##################
echo "################### B2: CHECK WHITELIST ###################"
near call $CONTRACT_ID xcc_use_promise_result --accountId $ACCOUNT_ID --gas $GAS_BASE
################## End B2: Check whitelist contract ##################

################## B3: Check whitelist contract ##################
echo "################### B3: CHECK WHITELIST ###################"
near call $CONTRACT_ID xcc_use_arg_macro --accountId $ACCOUNT_ID --gas $GAS_BASE
################## End B3: Check whitelist contract ##################

################## B4: Check used gas ##################
echo "################### B4: CHECK USED GAS ###################"
near view $CONTRACT_ID used_gas
################## End B4: Check used gas ##################

################## B5: Check prepaid gas ##################
echo "################### B5: CHECK PREPAID GAS ###################"
near view $CONTRACT_ID prepaid_gas
################## End B5: Check prepaid gas ##################
