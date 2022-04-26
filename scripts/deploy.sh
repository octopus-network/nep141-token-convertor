#!/bin/bash
set -e

source ./variables.sh

cd ..
bash build.sh
cd scripts

if [ "$1" == "deploy" ]; then
  near deploy $CONVERTOR_CONTRACT_ACCOUNT_ID ../res/$CONVERTOR_WASM_NAME new '{"white_list_admin": "'$WHITELIST_ADMIN_ACCOUNT_ID'"}'
elif [ "$1" == "redeploy" ]; then
  near deploy $CONVERTOR_CONTRACT_ACCOUNT_ID ../res/$CONVERTOR_WASM_NAME
elif [ "$1" == "clean" ]; then
  bash clear-state.sh && near deploy $CONVERTOR_CONTRACT_ACCOUNT_ID ../res/$CONVERTOR_WASM_NAME new '{"white_list_admin": "'$WHITELIST_ADMIN_ACCOUNT_ID'"}'
fi
