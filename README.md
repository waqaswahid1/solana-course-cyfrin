# solana-course

TODO: clean up
TODO: update solana?
TODO: upload exercises

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
        - native -> anchor or anchor -> native
        - Debug
            - `msg`
        - Anchor tests in Rust are unreliable

# Core concepts
- [x] [Solana vs Ethereum](./notes/eth-sol.png)
- [x] [Accounts](./notes/account.png)
- [x] [Programs, instructions and transactions](./notes/program.png)
- [x] [PDA](./notes/pda.png)
- [x] [CPI](https://solana.com/docs/core/cpi)
- [x] [IDL](https://solana.com/developers/guides/advanced/idls)

# Hello
- [x] [How is the program id derived?](./notes/program-id.md)
- [x] [Limitations](https://solana.com/docs/programs/limitations)
- [Native](./apps/hello/native)
- [Anchor](./apps/hello/anchor)

# Oracle
- [x] [State](./notes/state.md)
- [Native](./apps/oracle/native)
- [Anchor](./apps/oracle/anchor)

# Piggy bank - PDA
- [x] [Manually transfer of SOL](./notes/transfer-sol.png)
- [Native](./apps/piggy/native)
- [Anchor](./apps/piggy/anchor)

# Dutch auction
- [x] [SPL Token](./notes/spl.png)
- [x] [Token CLI](./notes/token.md)
- [Anchor](./apps/auction/anchor)
- [Native](./apps/auction/native)

# AMM
- [x] [Constant sum AMM](https://www.desmos.com/calculator/4ro4f7iwlz)
- [Anchor](./apps/amm/anchor)
- [Native](./apps/amm/native)

# CPI and IDL
- [Anchor](./apps/cpi/anchor)
- [Native](./apps/cpi/native)

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
