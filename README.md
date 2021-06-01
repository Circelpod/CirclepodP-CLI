# Circelpod Protocol CLI

## Create SPL Account

> 首次執行請先使用 `spl-token create-token` [創造 Token](https://github.com/solana-labs/solana-program-library/blob/f3a8fae2f5816d68afcfbf60593fac95c65373f5/docs/src/token.md)

`RUST_LOG=debug cargo run create-spl-account`
然後到[explorer](https://explorer.solana.com/)查詢你的交易。會發現有多一個SPL Token帳號。

ADDRESS	TOKEN	CHANGE	POST BALANCE
EADJPCufGrM8KoVLdz89qu7WGC3zT4kcaR4rJH8F44Et MFE51guQQyixqGVFqQxijW667RzpriU3gabiFV6TmCG 0	0 tokens

## Create SPL Token Mint

> How to use spl-token library and rust code to create SPL Token?
> If you want to use spl-token library and rust code to create SPL Token, please refer to the following code.
> 如果你想使用 spl-token 庫與 Rust 代碼，創建 SPL Token，請參考以下原始碼
> Example:
> src/create_spl_token/mod.rs

`RUST_LOG=debug cargo run create-spl-token`
然後到[explorer](https://explorer.solana.com/)查詢你的交易。會發現有多一個 Token。
