#!/bin/bash
set -e

source ./variables.sh

cd ..
bash build.sh &&
cd scripts

if [ "$1" == "deploy" ]; then
  near deploy $CONVERTOR_CONTRACT_ACCOUNT_ID ../res/$CONVERTOR_WASM_NAME new '{"owner": "'$OWNER_ACCOUNT_ID'", "create_pool_deposit": "'$CREATE_POOL_DEPOSIT_NEAR_AMOUNT'"}'
elif [ "$1" == "redeploy" ]; then
  near deploy $CONVERTOR_CONTRACT_ACCOUNT_ID ../res/$CONVERTOR_WASM_NAME
elif [ "$1" == "clean" ]; then
  bash clear-state.sh && near deploy $CONVERTOR_CONTRACT_ACCOUNT_ID ../res/$CONVERTOR_WASM_NAME new '{"owner": "'$OWNER_ACCOUNT_ID'", "create_pool_deposit": "'$CREATE_POOL_DEPOSIT_NEAR_AMOUNT'"}'
fi
