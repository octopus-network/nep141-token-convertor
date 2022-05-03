use crate::common::constant::{convertor_contract_id, string_to_account};
use crate::common::contracts::{
    deploy_test_token_contract, print_execution_result, setup_convertor_contract, NearContract,
};
use itertools::Itertools;
use near_sdk::json_types::U128;
use near_sdk::{AccountId, Balance};
use near_sdk_sim::{call, to_yocto, view, ContractAccount, ExecutionResult, UserAccount};
use nep141_token_convertor_contract::conversion_pool::ConversionPool;
use nep141_token_convertor_contract::types::PoolId;
use nep141_token_convertor_contract::FtMetaData;
use nep141_token_convertor_contract::TokenConvertorContract;
use test_token::ContractContract as TestTokenContract;

pub struct Convertor {
    pub contract: ContractAccount<TokenConvertorContract>,
}

impl NearContract<TokenConvertorContract> for Convertor {
    fn get_contract(&self) -> &ContractAccount<TokenConvertorContract> {
        &self.contract
    }
}

impl Convertor {
    pub fn get_pools(&self, from_index: u32, limit: u32) -> Vec<ConversionPool> {
        let contract = &self.contract;
        view!(contract.get_pools(from_index, limit)).unwrap_json::<Vec<ConversionPool>>()
    }

    pub fn get_whitelist(&self) -> Vec<FtMetaData> {
        let contract = &self.contract;
        view!(contract.get_whitelist()).unwrap_json::<Vec<FtMetaData>>()
    }
    // fn get_pools(&self, from_index: u32, limit: u32) -> Vec<ConversionPool> {
    //     view()
    //
    // }

    pub fn extend_whitelisted_tokens(
        &self,
        signer: &UserAccount,
        tokens: Vec<FtMetaData>,
    ) -> ExecutionResult {
        let contract = &self.contract;

        let result = call!(signer, contract.extend_whitelisted_tokens(tokens));
        print_execution_result(&result);
        result
    }

    pub fn remove_whitelisted_tokens(
        &self,
        signer: &UserAccount,
        tokens: Vec<AccountId>,
    ) -> ExecutionResult {
        let contract = &self.contract;

        let result = call!(signer, contract.remove_whitelisted_tokens(tokens));
        print_execution_result(&result);
        result
    }

    pub fn set_pool_create_deposit_amount(
        &self,
        signer: &UserAccount,
        amount: U128,
    ) -> ExecutionResult {
        let contract = &self.contract;

        let result = call!(signer, contract.set_pool_create_deposit_amount(amount));
        print_execution_result(&result);
        result
    }

    pub fn create_pool(
        &self,
        signer: &UserAccount,
        token_from: AccountId,
        token_to: AccountId,
        is_reversible: bool,
        in_token_rate: u32,
        out_token_rate: u32,
        attach: Option<Balance>,
    ) -> ExecutionResult {
        let contract = &self.contract;
        let result = if attach.is_some() {
            call!(
                signer,
                contract.create_pool(
                    token_from,
                    token_to,
                    is_reversible,
                    in_token_rate,
                    out_token_rate
                ),
                deposit = attach.unwrap()
            )
        } else {
            call!(
                signer,
                contract.create_pool(
                    token_from,
                    token_to,
                    is_reversible,
                    in_token_rate,
                    out_token_rate
                )
            )
        };
        // let result = call!(
        //     signer,
        //     contract.create_pool(token_from,token_to,is_reversible,in_token_rate,out_token_rate));
        print_execution_result(&result);
        result
    }

    pub fn withdraw_token(
        &self,
        signer: &UserAccount,
        pool_id: PoolId,
        token_id: AccountId,
        amount: U128,
    ) -> ExecutionResult {
        let contract = &self.contract;
        let result = call!(
            signer,
            contract.withdraw_token(pool_id, token_id, amount),
            deposit = 1
        );
        print_execution_result(&result);
        result
    }

    pub fn delete_pool(&self, signer: &UserAccount, pool_id: PoolId) -> ExecutionResult {
        let contract = &self.contract;
        let result = call!(signer, contract.delete_pool(pool_id), deposit = 1);
        print_execution_result(&result);
        result
    }
}

pub fn setup_pools() -> (
    UserAccount,
    UserAccount,
    Convertor,
    UserAccount,
    UserAccount,
    Vec<FtMetaData>,
    Vec<ContractAccount<TestTokenContract>>,
) {
    let (root, admin, convertor) = setup_convertor_contract();
    let creator = root.create_user(string_to_account("creator"), to_yocto("100"));
    let user = root.create_user(string_to_account("user"), to_yocto("100"));
    let whitelist_tokens = vec![
        FtMetaData {
            token_id: string_to_account("usdt"),
            decimals: 6,
        },
        FtMetaData {
            token_id: string_to_account("usdc"),
            decimals: 6,
        },
        FtMetaData {
            token_id: string_to_account("usdn"),
            decimals: 6,
        },
    ];
    convertor.extend_whitelisted_tokens(&admin, whitelist_tokens.clone());
    let token_contracts = whitelist_tokens
        .iter()
        .map(|e| {
            deploy_test_token_contract(
                &root,
                e.token_id.clone(),
                vec![
                    convertor_contract_id(),
                    string_to_account("creator"),
                    string_to_account("user"),
                ],
            )
        })
        .collect_vec();

    return (
        root,
        admin,
        convertor,
        creator,
        user,
        whitelist_tokens,
        token_contracts,
    );
}
