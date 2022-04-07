# ufunep141-token-convertor

The purpose of this contract is to provide conversion service for nep141 tokens in whitelist.

Contents:

- [Terminology](#terminology)
- [Function specification](#function-specification)
  - [Whitelist management](#Whitelist-management)
  - [Create conversion pool](#Create-conversion-pool)
  - [Transfer token to contract](#Transfer-token-to-contract)
  - [View functions](#View-functions)
- [Contract interfaces](#Contract-interfaces)
  - [Pool creator interfaces](#Pool-creator-interfaces)
  - [Whitelist admin interfaces](#whitelist-admin-interfaces)
  - [Types of msg field in ft_transfer_call](#Types-of-msg-field-in-ft_transfer_call)
    - [AddLiquidity](#AddLiquidity)
    - [Convert](#Convert)

## Terminology

- `nep141`: [A standard interface for fungible tokens in near network.](https://nomicon.io/Standards/FungibleToken/Core)
- `conversion pool`: A conversion pool contain a pair of tokens and allows user convert a kind of token to another at a certain rate.
- `pool creator`: People who create a conversion pool.
- `rate`: Pool creator can set converting rate when creating pool.Then user can convert token by this immutable rate in this pool no matter what direction of conversion is.
- `whitelist`: Pool creator can only create a conversion pool for tokens in whitelist.
- `reversible`: By default, the conversion pool is one-way mapping, which means you can only convert token A to B. But when create pool, creator can also select whether users are allowed to convert token reversely, which means the users can exchange token A and token B in both directions.
- `user`: People who use a conversion pool to convert tokens.
- `admin`: People who can manage whitelist.
- `from_token`: If a conversion pool can convert `token A` to `token B`, we use `from_token` refer to `token A`.
- `to_token`: If a conversion pool can convert `token A` to `token B`, we use `to_token` refer to `token B`.

## Function specification

### Whitelist management

Based on the design of `nep141`, different tokens can use same icon and name, so we need to provide a whitelist to protect users from fraud.

In this contract, the actions that `admin` can perform are as the following:

- Add token into whitelist.
- Remove token from whitelist.

Refer to [whitelist admin interfaces](#whitelist-admin-interfaces) for the contract interfaces.

### Create conversion pool

Anyone can create a conversion pool for a pair of tokens in whitelist. When someone creates a pool, he needs to set conversion `rate` and whether the pool is `reversible`. The `rate` and `reversible` of the pool can't be updated or deleted after it is created.

Refer to [pool creator interfaces](#pool-creator-interfaces) for the contract interfaces.

### Transfer token to contract

Anyone transfers token to this contract needs to specify purpose. If not, tokens will be fully refunded.

In this contract, the valid purposes are as the following:

- `Adding liquidity` - Anyone can transfer `nep141 token` to this contract for adding liquidity to a pool. We have two rules for checking if transferred token is meaningful:

  - Transferred token must be `from token` or `to token`.
  - Transferred token can be `to token` only when `conversion pool` is `reversible`.
- `Converting token` - Anyone can transfer `nep141 token` to this contract for converting a type of token into another.When you try to convert token,you should know two rule as the following:

  - You can only transfer `from token` for converting it into `to token`. And if `pool creator` set `conversion pool` `reversible`, the users can also transfer `to token` for converting it into `from token`.

  * You can specify a `minimum received amount` when you are converting.If pool can’t satisfy the `minimum received amount`,all of you transferred tokens will be fully refunded.

These functions will be implemented by nep141's interface: [ft_on_transfer](https://nomicon.io/Standards/FungibleToken/Core#reference-level-explanation). When nep141 token is transferred into this contract by calling function `ft_transfer_call` of token contract, a certain information which specifies the purpose can be attached by param `msg`.

Refer to [types of msg field in ft_transfer_call](#types-of-msg-field-in-ft_transfer_call) for more detail information of the `msg` field.

### View functions

This contract has a set of view functions for anyone to get the status detail of this contract.

## Contract interfaces

### Pool creator interfaces

```rust
// create conversion pool
fn create_conversion_pool(token_from: AccountId, token_to: AccountId,is_reversible: bool,rate: u32,rate_decimal: u32);
```

### Whitelist admin interfaces

```rust
// Extend whitelisted tokens with new tokens. Only can be called by owner.
fn add_whitelisted_tokens(tokens: Vec<AccountId>);
// Remove whitelisted token. Only can be called by owner.
fn remove_whitelisted_tokens(tokens: Vec<AccountId>);
```

### Types of msg field in ft_transfer_call

Some functions wil be implemented by nep141's interface: [ft_transfer_call](https://nomicon.io/Standards/FungibleToken/Core#reference-level-explanation). When nep141 token transfer into contract by : `ft_transfer_call`, it can be attached some information by param:  `msg` . Here we will define an enum type: `enum TokenTransferMessage`,and then we define some **enum items**  for different usages:

#### AddLiquidity

The function specification refer to [transfer token to contract](#transfer-token-to-contract).

```rust
pub enum TokenTransferMessage {
	AddLiquidity {
		pool_id: u64,
	}
}
```

#### Convert

The function specification refer to [transfer token to contract](#transfer-token-to-contract).

```rust
pub enum TokenTransferMessage {
	Convert {
		// a group of convert action.
		convert_actions: Vec<ConvertAction>
	}
}
// user convert a type of token into another in some pool
// user can specify except receive token id and amount.
pub struct ConvertAction {
    // pool id
    pub pool_id: u64,
    // except receive token
    pub except_receive_token_id: AccountId,
    // except output token amount
    pub except_receive_token_amount: U128
}
```
