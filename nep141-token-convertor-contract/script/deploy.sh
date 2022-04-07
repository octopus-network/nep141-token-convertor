#!/bin/bash
set -e

source ./variables.sh

## deploy and init
#bash ./build.sh && near deploy $CONVERTOR_CONTRACT_ACCOUNT_ID ../out/$CONVERTOR_WASM_NAME new '{"white_list_admin": "'$WHITELIST_ADMIN_ACCOUNT_ID'"}'

# re-deploy after init
#bash ./build.sh && near deploy $CONVERTOR_CONTRACT_ACCOUNT_ID ../out/$CONVERTOR_WASM_NAME

# clean-state than init
bash ./clear-state.sh && bash ./build.sh && near deploy $CONVERTOR_CONTRACT_ACCOUNT_ID ../out/$CONVERTOR_WASM_NAME new '{"white_list_admin": "'$WHITELIST_ADMIN_ACCOUNT_ID'"}'
