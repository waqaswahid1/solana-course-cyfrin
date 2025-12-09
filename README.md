# solana-course

# Course intro

- Solana CLI 3.0
- Solana SDK and program 2.2
- Anchor 0.31

- [ ] [Course intro](./notes/course_intro.md)
    - Prerequisites
        - Rust
        - Blockchain
    - Learn
        - AI (starter -> fix code)
            - Anchor -> Native
            - test
        - crates.rs -> docs.rs
- [ ] [Setup](./notes/install.md)
    - Install
    - CLI basics
    - Wallet
    - Exercises
        - native, anchor, exercise, solution
            - README is exercise
        - Debug
            - `msg`
        - Anchor tests in Rust are unreliable

# Core concepts
- [ ] [Solana vs Ethereum](./notes/eth-sol.png)
- [ ] [Accounts](./notes/account.png)
    - data
    - lamports
    - owner
    - Keypair must prove existence of keypair before sending transaction to create account, receive SOL, etc
    - Rent
        - Data must be empty if SOL = 0
- [ ] [Programs, instructions and transactions](./notes/program.png)
    - [ ] program id
        - how is it derived?
            - Randomly generated private key typically stored under  `target/deploy/my_program.json`
            - Private key needed for upgrade
        - Pub key needed for user interaction
    - [ ] [Limitations](https://solana.com/docs/programs/limitations)
- [ ] [PDA](./notes/pda.png)
- [ ] [CPI](https://solana.com/docs/core/cpi)
    - permissions
    - 4 depth
- [ ] [IDL](https://solana.com/developers/guides/advanced/idls)

# Hello
- [ ] [Native](./apps/hello/native)
    - Borsh
    - `entrypoint`
    - `msg`
    - Build, test, deploy
    - `cargo build-sbf`
    - Test
        - Script
            - `solana-test-calidator`
            - `.so`
    - Deploy
        - [Solana explorer](https://explorer.solana.com/)
    - Exercises
- [ ] [Anchor](./apps/hello/anchor)
    - `anchor init hello --test-template rust`
    - https://www.anchor-lang.com/docs/basics/program-structure
        - `declare_id` -> Anchor.toml
        - `#program`
        - `Accounts`

# Oracle
- [ ] [Native](./apps/oracle/native)
    - State - Borsh
    - Program owns oracle account
    - Program owns oracle account
    - Oracle data space
    - Order of account is important
    - `owner` must sign
- [ ] [Anchor](./apps/oracle/anchor)
    - `anchor keys sync`
    - `InitSpace`
    - `Signer`
    - `mut`
    - `init`
    - `constraint`
    - Discriminator

# Piggy bank - PDA
- [ ] [Native](./apps/piggy-bank/native)
    - CPI - invoke
    - Calculate PDA before PDA is initialized
    - Create PDA
    - Transfer SOL
    - Manually send SOL
    - Account data must be empty when account has 0 SOL
- [ ] [Anchor](./apps/piggy-bank/anchor)
    - PDA bump, init and close

# Dutch auction
- [ ] [SPL Token](./notes/spl.png)
- [ ] [Token CLI](./notes/token.md)
    - Create mint
        - Mint account state
        - Mint authority
    - Create token account
    - Mint tokens
- [ ] [Anchor](./apps/auction/anchor)
- [ ] [Native](./apps/auction/native)

# AMM
- [ ] [Anchor](./apps/amm/anchor)
- [ ] [Native](./apps/amm/native)

# CPI and IDL
- [ ] [Anchor](./apps/cpi/anchor)
    - compile counter
    - `Factory counter; Signer<info>`
    // In project root, copy IDL files to idls
    //  cp target/idl/*.json ./idls/
    - `Anchor.toml`
    - tests/Cargo.toml (`no-entrypoint`)
- [ ] [Native](./apps/cpi/native)
    - No auto generated IDL

# Wormhole
- Overview
    - Guardian network, consensus model
    - VAA and message relaying
- NTT
    - Overview (SPL to ETH ERC20)
    - CLI setup (NTT 1.6.0)
        - https://wormhole.com/docs/products/token-transfers/native-token-transfers/get-started/
        - ERC20 / SPL
            - [Deploy ERC20](https://sepolia.etherscan.io/address/0x0c3d43954B0b312D591739980E0A157621B581BC)
            ```shell
            export ERC20=...
            ```
            - Deploy SPL
                ```shell
                solana config set -ud
                solana balance
                solana airdrop 1

                cargo install spl-token-cli
                spl-token --version

                spl-token create-token
                MINT=...

                spl-token create-account $MINT
                ATA=...

                spl-token mint $MINT 1000

                https://explorer.solana.com/address/AJA8ApLpXHr9pM2CS85y9g4UgFtXuM71gXU9vGBTmMcB?cluster=devnet
                mint
                73Rgt8CZCJez89VtJdRd84kfUPVSprS2Sy5V7Skmq1bU
                ```

                ```shell
                export SPL=...
                ```

                - Install NTT CLI
                ```shell
                ntt --version
                ```
                - Init NTT project
                - Deploy to EVM
                    ```shell
                    export ETH_PRIVATE_KEY=
                    export SEPOLIA_SCAN_API_KEY=
                    ```
                - Set rate limits
                - Set mint authority to NTTManager
            - Deploy to SVM
                - `solana airdrop 10`
                - Solana and Anchor version
                    ```shell
                    Install latest Solana CLI
                    sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"

                    NTT specific version
                    sh -c "$(curl -sSfL https://release.anza.xyz/v1.18.26/install)"

                    cargo install --git https://github.com/solana-foundation/anchor --tag v0.29.0 anchor-cli

                    solana --version
                    anchor --version
                    ```
                - Demo with NTT SDK
                  https://github.com/wormhole-foundation/demo-ntt-ts-sdk
                  - comment out optional RPC url
                  - set dst address and amount
                  - ts-ignore wh
                  https://wormholescan.io/#/tx/26ZaFXBfHwSzSQL9xEWmGpJheV4kPcSeAQVbTNizaxeo9Cf8Zq5QDDygXGs6roquLRKhaaWYoG6yFnUyxpNw2s1X?network=Testnet
    - Token transfer demo
- Deploy AMM + swap using NTT

# Resources

- [Solana docs](https://solana.com/docs)
- [Solana program](https://www.solana-program.com/)
- [solscan](https://solscan.io/)
- [Solana faucet](https://faucet.solana.com/)
- [GitHub - Solana program](https://github.com/solana-program)
- [GitHub - Anchor](https://github.com/solana-foundation/anchor)
- [Anchor doc](https://www.anchor-lang.com/docs)
- [GitHub - solana-developers/program-examples](https://github.com/solana-developers/program-examples)
- [GitHub - litesvm](https://github.com/LiteSVM/litesvm)
- [docs.rs - litesvm](https://docs.rs/litesvm/latest/litesvm/)
- [Solana explorer](https://explorer.solana.com/)
- [crates.io](https://crates.io/)
- [Solana playground](https://beta.solpg.io/)
- [GitHub - Wormhole](https://github.com/wormhole-foundation)
- [GitHub - Wormhole NTT](https://github.com/wormhole-foundation/native-token-transfers)
- [GitHub - Wormhole TypeScript SDK](https://github.com/wormhole-foundation/wormhole-sdk-ts)
- [Wormhole - NTT](https://wormhole.com/docs/products/token-transfers/native-token-transfers/overview/)
- [Wormhole Advanced Tech Workshop: NTT](https://www.youtube.com/watch?v=ltZmeyjUxRk)
- [Remix](https://remix.ethereum.org)
