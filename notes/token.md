# Token CLI

https://www.solana-program.com/docs/token

```shell
cargo install spl-token-cli

# Setup
solana config set -ul
solana-test-validator
solana airdrop 1

# Get wallet address
spl-token address

# Create token
spl-token create-token

# Check token info
TOKEN_ADDR=...
spl-token display $TOKEN_ADDR

# Create token account (ATA)
spl-token create-account $TOKEN_ADDR

# Mint to wallet (recipient is ATA)
spl-token mint $TOKEN_ADDR 100

# Check token balance
spl-token balance $TOKEN_ADDR

# List all tokens owned by this wallet
spl-token accounts
```
