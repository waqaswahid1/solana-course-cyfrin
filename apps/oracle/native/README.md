# Oracle (Native)

Complete all tasks below
- Implement an oracle program
- Test locally with LiteSVM
- Deploy locally to `solana-test-validator` and test with Rust script

# Task 1 - Implement [`instructions::init`](https://github.com/Cyfrin/solana-course/blob/main/apps/oracle/native/exercise/src/instructions/init.rs)

- Check that `oracle_account` is not initialized
- Store the `owner` and `price` into `oracle_account`

# Task 2 - Implement [`instructions::update`](https://github.com/Cyfrin/solana-course/blob/main/apps/oracle/native/exercise/src/instructions/update.rs)

-  Check that the `oracle.owner` signed this instruction to update the price
- Update the price

# Build

```shell
cargo build-sbf
```

# Test with LiteSVM
```shell
cargo test -- --nocapture
```

# Test with script

Run local validator
```shell
solana config set -ul
solana-test-validator
```

Deploy program
```shell
solana program deploy ./target/deploy/oracle.so
```

Execute demo script
```shell
PROGRAM_ID=your program ID
RPC=http://localhost:8899
KEYPAIR=path to key pair

cargo run --example demo $KEYPAIR $RPC $PROGRAM_ID
```

