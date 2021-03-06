# Circelpod Protocol CLI

## Create SPL Account

This effect is equivalent to [`spl-token create-account`](https://spl.solana.com/token)

> How to use spl-token library and rust code to create SPL Token Account?

> If you want to use spl-token library and rust code to create SPL Token Account, please refer to the following code(SPL Token has been created)

> 如果你想使用 spl-token 庫與 Rust 代碼，創建 SPL Token Account，請參考以下原始碼（已經創造 SPL Token）

> Example:
> src/create_spl_account/mod.rs

> 首次執行請先使用 `spl-token create-token` [創造 Token](https://github.com/solana-labs/solana-program-library/blob/f3a8fae2f5816d68afcfbf60593fac95c65373f5/docs/src/token.md)

`RUST_LOG=debug cargo run spl create-account`
然後到[explorer](https://explorer.solana.com/)查詢你的交易。會發現有多一個SPL Token帳號。

ADDRESS	TOKEN	CHANGE	POST BALANCE
EADJPCufGrM8KoVLdz89qu7WGC3zT4kcaR4rJH8F44Et MFE51guQQyixqGVFqQxijW667RzpriU3gabiFV6TmCG 0	0 tokens

## Create SPL Token Mint

This effect is equivalent to [`spl-token create-token`](https://spl.solana.com/token)

> How to use spl-token library and rust code to create SPL Token?

> If you want to use spl-token library and rust code to create SPL Token, please refer to the following code.

> 如果你想使用 spl-token 庫與 Rust 代碼，創建 SPL Token，請參考以下原始碼

> Example:
> src/create_spl_token/mod.rs

`RUST_LOG=debug cargo run spl create-token`
然後到[explorer](https://explorer.solana.com/)查詢你的交易。會發現有多一個 Token。

if you want set decimals, you can use `RUST_LOG=debug cargo run spl create-token {decimals}`
