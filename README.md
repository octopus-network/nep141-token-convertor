# nep141-token-convertor

The purpose of this contract is to provide conversion service for nep141 tokens in the whitelist.

Contents:

- [Terminology](#terminology)
- [Function specification](#function-specification)
  - [Whitelist management](#Whitelist-management)
  - [Create a conversion pool](#Create-a-conversion-pool)
  - [Delete a conversion pool](#Delete-a-conversion-pool)
  - [Transfer token to contract](#Transfer-token-to-contract)
  - [Withdraw token from pool](#Withdraw-token-from-pool)
  - [Pause and resume contract](#Pause-and-resume-contract)
  - [View functions](#View-functions)

## Terminology

- `nep141`: [A standard interface for fungible tokens in near network.](https://nomicon.io/Standards/FungibleToken/Core)
- `conversion pool`: A conversion pool contains a pair of tokens and allows the user to convert a kind of token to another at a certain rate.
- `pool creator`: People who create a conversion pool.
- `rate`: The `pool creator` can set the converting rate when creating a pool. Then users can convert tokens by this immutable rate in this pool no matter what direction of conversion is.
- `whitelist`: `Pool creator` can only create a conversion pool for tokens in a whitelist.
- `reversible`: By default, the conversion pool is one-way mapping, which means users can only convert token A to B. But when creating a pool, the creator can also select whether users are allowed to convert tokens reversely, which means the users can exchange token A and token B in both directions.
- `user`: People who use a conversion pool to convert tokens.
- `admin`: People who can manage whitelist, change deposit near amount when creating a pool and delete pools.
- `from_token`: If a conversion pool can convert `token A` to `token B`, using `from_token` refer to `token A`.
- `to_token`: If a conversion pool can convert `token A` to `token B`, using `to_token` refer to `token B`.

## Function specification

### Whitelist management

Based on the design of `nep141`, different tokens can use the same icon and name, so a whitelist is required for protecting users from fraud.

In this contract, the actions that `admin` can perform are as the following:

- Add token into the whitelist.
- Remove token from the whitelist.

### Create a conversion pool

Anyone can create a conversion pool for a pair of tokens in the whitelist. When someone creates a pool, he needs to set the conversion `rate` and whether the pool is `reversible`. The `rate` and `reversible` of the pool can't be updated or deleted after it is created. And creator needs to deposit some near base on the current config when creating a pool, these near will be refunded to the creator when the pool is deleted.

### Delete a conversion pool

The pool creator and admin can delete the pool. Before a pool is deleted, it requires tokens in the pool should be  withdrawn. The near tokens that are deposited when creating the pool will transfer to the creator after the pool is deleted.

### Transfer token to contract

Anyone who transfers tokens to this contract needs to specify the purpose. If not, tokens will be fully refunded.

In this contract, the valid purposes are as the following:

- `Adding liquidity` - Anyone can transfer `nep141 token` to this contract for adding liquidity to a pool. There are two rules for checking if transferred tokens are meaningful:

  - Transferred token must be `from token` or `to token`.
  - Transferred token can be `to token` only when `conversion pool` is `reversible`.
- `Converting token` - Anyone can transfer `nep141 token` to this contract for converting one type of token into another. When users try to convert tokens, they should know two rules as the following:

  - User can only transfer `from token` for converting it into `to token`. And if `pool creator` set `conversion pool` `reversible`, the users can also transfer `to token` for converting it into `from token`.

  * User can specify a `minimum received amount` when users are converting. If the pool can’t satisfy the `minimum received amount`, all transferred tokens will be fully refunded.

These functions will be implemented by nep141's interface: [ft_on_transfer](https://nomicon.io/Standards/FungibleToken/Core#reference-level-explanation). When nep141 token is transferred into this contract by calling function `ft_transfer_call` of token contract, certain information which specifies the purpose can be attached by param `msg`.

### Withdraw token from pool

The pool creator and admin can withdraw tokens from the pool to the creator account.

### Pause and resume contract

Admin can pause and resume contract for enhancing security. When the contract is pausing, most contract functions will be unavailable.

### View functions

This contract has a set of view functions for anyone to get the status detail of this contract.
