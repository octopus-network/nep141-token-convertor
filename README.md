# nep141-token-convertor

The purpose of this contract is to provide conversion service for nep141 tokens in whitelist.

Contents:

- [nep141-token-convertor](#nep141-token-convertor)
- [Terminology](#terminology)
- [Function specification](#function-specification)
  - [create conversion pool](#create-conversion-pool)
  - [deposit token into pool](#deposit-token-into-pool)
  - [convert](#convert)
  - [view functions](#view-functions)
- [contract interface](#contract-interface)
  - [view methods](#view-methods)
  - [change methods](#change-methods)
  - [ft_on_transfer](#ft_on_transfer)

# Terminology

- `nep141`:[ A standard interface for fungible tokens in near network.](https://nomicon.io/Standards/FungibleToken/Core)
- `conversion pool`: A conversion pool contain a pair of tokens and allows user convert a kind of token to another at a certain rate.
- `rate`: Pool creator can set converting rate when creating pool.Then user will convert token by this immutable rate in this pool.
- `whitelist`: Pool creator can only create a conversion pool for tokens in whitelist.
- `reversible`: By default, the conversion pool is one-way mapping which means you can only convert token A to B.But creator can select if allows user convert token reversely when creating pool which means you can both convert token A  to B or B to A.

# Function specification

## create conversion pool

Anyone can create a conversion pool for a pair of tokens in whitelist.When someone create a pool,he needs to set conversion rate and if pool is reversible.Pool cann't be deleted after it created.

## deposit token into pool

Anyone can deposit nep141 tokens into a conversion pool.And contract will check whether token address is coincident,if not,contract will refund tokens.

## convert

Anyone can select a conversion pool and convert a kind of token to another at a certain rate.

## view functions

This contract has a set of view functions for anyone to get the status detail of this contract.

# contract interface

## view methods

```rust
// get all pools by token conversion direction
fn get_pools_by_token_direction(in_token: TokenId,out_token: TokenId )->Vec<ConversionPool>;
```

## change methods

```rust
// create conversion pool
fn create_conversion_pool(token_from: ValidatorId, token_to: ValidatorId,is_reversible: bool,rate: u32)
```

## ft_on_transfer

- Some functions wil be implemented by nep141's interface: [ft_on_transfer](https://nomicon.io/Standards/FungibleToken/Core#reference-level-explanation).When nep141 token transfer into contract by : `ft_transfer_call`, it can be attached some information by param:  `msg` .
- So we define a enum Type `TokenTransferMessage`:
  ```
  pub enum TokenTransferMessage {
  	// deposit token into conversion pool
  	DepositIntoPool {
  		// pool id
  		pool_id: PoolId,
  	},
  	// user convert token by send ConverAction to a pool.
  	Convert {
  		// a group of convert action.
  		convert_actions: Vec<ConvertAction>;
  	}
  }
  
  pub struct ConvertAction {
      // pool id
      pub pool_id: u64,
      // input token
      pub in_token: AccountId,
      // input token amount
      pub in_token_amount: U128,
      // output token
      pub out_token: AccountId,
      // except output token amount
      pub except_out_token_amount: U128
  }
  ```
