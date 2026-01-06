# Marginfi CLI Guide

A command-line interface for interacting with the Marginfi lending protocol on Solana.

## Table of Contents

- [Installation](#installation)
- [Profile Setup](#profile-setup)
- [Admin Commands](#admin-commands)
  - [Initialize Fee State](#1-initialize-fee-state-one-time-per-program)
  - [Create Marginfi Group](#2-create-marginfi-group)
  - [Add Bank](#3-add-bank)
  - [Configure Bank Oracle](#4-configure-bank-oracle)
  - [Update Bank Configuration](#5-update-bank-configuration)
  - [Update Group Configuration](#6-update-group-configuration)
  - [Setup Emissions](#7-setup-emissions)
  - [Fee Management](#8-fee-management)
- [User Commands](#user-commands)
  - [Create Account](#1-create-marginfi-account)
  - [Deposit](#2-deposit)
  - [Withdraw](#3-withdraw)
  - [Borrow](#4-borrow)
  - [Liquidate](#5-liquidate)
- [Query Commands](#query-commands)
- [Token Commands](#token-commands)
  - [Create Token](#1-create-token)
  - [Mint Tokens](#2-mint-tokens)
  - [Transfer Tokens](#3-transfer-tokens)
  - [Check Balance](#4-check-balance)
- [Parameter Reference](#parameter-reference)

---

## Installation

Build the CLI from the repository root:

```bash
# Standard build (uses mainnet-beta program ID)
cargo build -p marginfi-v2-cli --release

# Build for devnet
cargo build -p marginfi-v2-cli --release --no-default-features --features devnet

# Build for staging
cargo build -p marginfi-v2-cli --release --no-default-features --features staging
```

The binary will be at `./target/release/mfi`.

---

## Profile Setup

Before using the CLI, create a profile to store your configuration.

### Create a Profile

```bash
cargo run --bin mfi profile create \
  --name devnet \
  --cluster devnet \
  --keypair-path ~/.config/solana/id.json \
  --rpc-url https://api.devnet.solana.com \
  --program-id 4sbjN16fgtnGMFaJ6s7aw9ha1uUQ9qLBv5gHSui2aNUU
```

**Parameters:**
| Parameter | Description |
|-----------|-------------|
| `--name` | Profile name for reference |
| `--cluster` | Solana cluster: `devnet`, `mainnet`, `localnet` |
| `--keypair-path` | Path to your Solana keypair file |
| `--rpc-url` | RPC endpoint URL |
| `--program-id` | (Optional) Marginfi program ID |
| `--group` | (Optional) Default marginfi group to use |
| `--account` | (Optional) Default marginfi account to use |

### List Profiles

```bash
cargo run --bin mfi profile list
```

### Switch Profile

```bash
cargo run --bin mfi profile set devnet
```

### Show Current Profile

```bash
cargo run --bin mfi profile show
```

### Update Profile

```bash
cargo run --bin mfi profile update devnet \
  --group BSWU1nFn7QRk2hhcP2yvJmzh656XmP1QyMVzBkxzFvV7 \
  --rpc-url https://your-new-rpc.com
```

---

## Admin Commands

These commands require admin privileges and are used to set up and manage the lending protocol.

### 1. Initialize Fee State (One-time per program)

This must be run once before any groups can be created. It sets up the global fee configuration for the protocol.

```bash
cargo run --bin mfi group init-fee-state \
  --admin BsHLjrzrStpW5aWgGh9qRt8bQtLiymfUcKqS5d6nKG9f \
  --fee-wallet BsHLjrzrStpW5aWgGh9qRt8bQtLiymfUcKqS5d6nKG9f \
  --bank-init-flat-sol-fee 1000 \
  --liquidation-flat-sol-fee 500 \
  --program-fee-fixed 0.01 \
  --program-fee-rate 0.02 \
  --liquidation-max-fee 0.5
```

**Parameters:**
| Parameter | Description | Example |
|-----------|-------------|---------|
| `--admin` | Global fee admin pubkey (can modify fees later) | Your wallet address |
| `--fee-wallet` | Wallet that receives all protocol fees | Your fee collection wallet |
| `--bank-init-flat-sol-fee` | Lamports charged when creating a bank | `1000` (0.000001 SOL) |
| `--liquidation-flat-sol-fee` | Lamports charged per liquidation | `500` |
| `--program-fee-fixed` | Fixed percentage fee collected by protocol | `0.01` (1%) |
| `--program-fee-rate` | Variable percentage fee rate | `0.02` (2%) |
| `--liquidation-max-fee` | Max percentage profit liquidators can claim | `0.5` (50%) |

**For development/testing with zero fees:**
```bash
cargo run --bin mfi group init-fee-state \
  --admin <YOUR_WALLET> \
  --fee-wallet <YOUR_WALLET> \
  --bank-init-flat-sol-fee 0 \
  --liquidation-flat-sol-fee 0 \
  --program-fee-fixed 0 \
  --program-fee-rate 0 \
  --liquidation-max-fee 0
```

---

### 2. Create Marginfi Group

Create a new lending group. A group is a container for multiple banks (lending pools).

```bash
cargo run --bin mfi group create
```

**With optional parameters:**
```bash
cargo run --bin mfi group create \
  --admin <ADMIN_PUBKEY> \
  --override \
  --is-arena-group false
```

**Parameters:**
| Parameter | Description |
|-----------|-------------|
| `--admin` | (Optional) Admin pubkey, defaults to your wallet |
| `--override` / `-f` | Override existing profile group setting |
| `--is-arena-group` | Set to `true` for arena-style groups |

---

### 3. Add Bank

Add a new bank (lending pool) for a specific token to your group.

```bash
cargo run --bin mfi group add-bank \
  --mint So11111111111111111111111111111111111111112 \
  --seed \
  --asset-weight-init 0.8 \
  --asset-weight-maint 0.9 \
  --liability-weight-init 1.2 \
  --liability-weight-maint 1.1 \
  --deposit-limit-ui 1000000 \
  --borrow-limit-ui 500000 \
  --optimal-utilization-rate 0.8 \
  --plateau-interest-rate 0.1 \
  --max-interest-rate 3.0 \
  --insurance-fee-fixed-apr 0.01 \
  --insurance-ir-fee 0.02 \
  --group-fixed-fee-apr 0.01 \
  --group-ir-fee 0.02 \
  --risk-tier collateral \
  --oracle-max-age 60 \
  --global-fee-wallet BsHLjrzrStpW5aWgGh9qRt8bQtLiymfUcKqS5d6nKG9f
```

**Parameters:**

| Parameter | Description | Example |
|-----------|-------------|---------|
| `--mint` | Token mint address | `So11111111111111111111111111111111111111112` (WSOL) |
| `--seed` | Use PDA for bank address (recommended) | flag |
| `--asset-weight-init` | Initial margin asset weight (0-1) | `0.8` = 80% collateral value |
| `--asset-weight-maint` | Maintenance margin asset weight | `0.9` = 90% collateral value |
| `--liability-weight-init` | Initial margin liability weight (>1) | `1.2` = 120% liability value |
| `--liability-weight-maint` | Maintenance margin liability weight | `1.1` = 110% liability value |
| `--deposit-limit-ui` | Max deposits allowed (in UI units) | `1000000` (1M tokens) |
| `--borrow-limit-ui` | Max borrows allowed (in UI units) | `500000` (500K tokens) |
| `--optimal-utilization-rate` | Target utilization rate | `0.8` (80%) |
| `--plateau-interest-rate` | Interest rate at optimal utilization | `0.1` (10% APR) |
| `--max-interest-rate` | Maximum interest rate | `3.0` (300% APR) |
| `--insurance-fee-fixed-apr` | Fixed APR for insurance fund | `0.01` (1%) |
| `--insurance-ir-fee` | Interest rate fee for insurance | `0.02` (2%) |
| `--group-fixed-fee-apr` | Fixed APR for group/protocol | `0.01` (1%) |
| `--group-ir-fee` | Interest rate fee for group | `0.02` (2%) |
| `--risk-tier` | Risk classification | `collateral` or `isolated` |
| `--oracle-max-age` | Max oracle staleness in seconds | `60` |
| `--global-fee-wallet` | Fee collection wallet | Your fee wallet address |

**Common Token Mints:**
- WSOL: `So11111111111111111111111111111111111111112`
- USDC (Mainnet): `EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v`
- USDC (Devnet): `4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU`

---

### 4. Configure Bank Oracle

Set up the price oracle for a bank. This must be done after adding a bank.

```bash
cargo run --bin mfi bank update-oracle <BANK_PUBKEY> \
  --oracle-type 3 \
  --oracle-key <ORACLE_PUBKEY>
```

**Example for WSOL bank with Pyth oracle:**
```bash
cargo run --bin mfi bank update-oracle 49DA6fLBXGjZGxmo4o4bNysH5dhcEAcGQD8aVRWD8kER \
  --oracle-type 3 \
  --oracle-key J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix
```

**Oracle Types:**
| Type | Description |
|------|-------------|
| `3` | Pyth Pull Oracle |
| `4` | Switchboard Pull Oracle |
| `5` | Staked Pyth Pull Oracle |

**Finding Oracle Addresses:**
- Pyth Devnet: https://pyth.network/developers/price-feed-ids
- Pyth Mainnet: https://pyth.network/price-feeds

---

### 5. Update Bank Configuration

Modify existing bank parameters.

```bash
cargo run --bin mfi bank update <BANK_PUBKEY> \
  --asset-weight-init 0.75 \
  --asset-weight-maint 0.85 \
  --deposit-limit-ui 2000000 \
  --borrow-limit-ui 1000000 \
  --operational-state operational
```

**All available parameters:**
| Parameter | Description |
|-----------|-------------|
| `--asset-weight-init` | Initial margin asset weight |
| `--asset-weight-maint` | Maintenance margin asset weight |
| `--liability-weight-init` | Initial margin liability weight |
| `--liability-weight-maint` | Maintenance margin liability weight |
| `--deposit-limit-ui` | Max deposits (UI units) |
| `--borrow-limit-ui` | Max borrows (UI units) |
| `--operational-state` | `operational`, `paused`, or `reduce-only` |
| `--opr-ur` | Optimal utilization rate |
| `--p-ir` | Plateau interest rate |
| `--m-ir` | Max interest rate |
| `--if-fa` | Insurance fee fixed APR |
| `--if-ir` | Insurance IR fee |
| `--pf-fa` | Protocol fixed fee APR |
| `--pf-ir` | Protocol IR fee |
| `--pf-or` | Protocol origination fee |
| `--risk-tier` | `collateral` or `isolated` |
| `--asset-tag` | Asset tag (0=default, 1=SOL, 2=Staked SOL LST) |
| `--usd-init-limit` | Soft USD init limit |
| `--oracle-max-age` | Max oracle age in seconds |
| `--oracle-max-confidence` | Max oracle confidence |
| `--permissionless-bad-debt-settlement` | Allow permissionless bad debt settlement |
| `--freeze-settings` | Permanently freeze bank settings |

---

### 6. Update Group Configuration

Update admin roles for a group.

```bash
cargo run --bin mfi group update \
  --new-admin <NEW_ADMIN_PUBKEY> \
  --new-emode-admin <EMODE_ADMIN_PUBKEY> \
  --new-curve-admin <CURVE_ADMIN_PUBKEY> \
  --new-limit-admin <LIMIT_ADMIN_PUBKEY> \
  --new-emissions-admin <EMISSIONS_ADMIN_PUBKEY> \
  --is-arena-group false
```

---

### 7. Setup Emissions

Configure token rewards/emissions for a bank.

```bash
cargo run --bin mfi bank setup-emissions <BANK_PUBKEY> \
  --deposits \
  --borrows \
  --mint <EMISSIONS_TOKEN_MINT> \
  --rate-apr 0.05 \
  --total-amount-ui 1000000
```

**Parameters:**
| Parameter | Description |
|-----------|-------------|
| `--deposits` | Enable emissions for depositors |
| `--borrows` | Enable emissions for borrowers |
| `--mint` | Token mint for emissions rewards |
| `--rate-apr` | Emissions rate as APR |
| `--total-amount-ui` | Total tokens allocated for emissions |

**Update emissions:**
```bash
cargo run --bin mfi bank update-emissions <BANK_PUBKEY> \
  --deposits \
  --borrows \
  --rate 0.03 \
  --additional-amount-ui 500000
```

**Disable emissions:**
```bash
cargo run --bin mfi bank update-emissions <BANK_PUBKEY> \
  --deposits \
  --borrows \
  --disable
```

---

### 8. Fee Management

**Collect fees from a bank:**
```bash
cargo run --bin mfi bank collect-fees <BANK_PUBKEY> <FEE_ATA>
```

**Withdraw fees:**
```bash
cargo run --bin mfi bank withdraw-fees <BANK_PUBKEY> <AMOUNT> \
  --destination-address <DESTINATION_PUBKEY>
```

**Withdraw insurance:**
```bash
cargo run --bin mfi bank withdraw-insurance <BANK_PUBKEY> <AMOUNT> \
  --destination-address <DESTINATION_PUBKEY>
```

**Configure group fees:**
```bash
cargo run --bin mfi group config-group-fee --enable-program-fee true
```

**Propagate fee state to a group:**
```bash
cargo run --bin mfi group propagate-fee --marginfi-group <GROUP_PUBKEY>
```

---

## User Commands

These commands are for end users interacting with the lending protocol.

### 1. Create Marginfi Account

Create a new marginfi account to start lending/borrowing.

```bash
cargo run --bin mfi account create
```

---

### 2. Deposit

Deposit tokens into a bank.

```bash
cargo run --bin mfi account deposit <BANK_PUBKEY> <AMOUNT>
```

**Example - Deposit 10 SOL:**
```bash
cargo run --bin mfi account deposit 49DA6fLBXGjZGxmo4o4bNysH5dhcEAcGQD8aVRWD8kER 10.0
```

**Deposit up to limit (if deposit would exceed limit, deposit max allowed):**
```bash
cargo run --bin mfi account deposit <BANK_PUBKEY> <AMOUNT> true
```

---

### 3. Withdraw

Withdraw tokens from a bank.

```bash
cargo run --bin mfi account withdraw <BANK_PUBKEY> <AMOUNT>
```

**Example - Withdraw 5 SOL:**
```bash
cargo run --bin mfi account withdraw 49DA6fLBXGjZGxmo4o4bNysH5dhcEAcGQD8aVRWD8kER 5.0
```

**Withdraw all:**
```bash
cargo run --bin mfi account withdraw <BANK_PUBKEY> 0 --all
```

---

### 4. Borrow

Borrow tokens from a bank.

```bash
cargo run --bin mfi account borrow <BANK_PUBKEY> <AMOUNT>
```

**Example - Borrow 100 USDC:**
```bash
cargo run --bin mfi account borrow <USDC_BANK_PUBKEY> 100.0
```

---

### 5. Liquidate

Liquidate an unhealthy position.

```bash
cargo run --bin mfi account liquidate \
  --liquidatee-marginfi-account <UNHEALTHY_ACCOUNT_PUBKEY> \
  --asset-bank <COLLATERAL_BANK_PUBKEY> \
  --liability-bank <DEBT_BANK_PUBKEY> \
  --ui-asset-amount 100.0
```

---

### 6. Close Account

Close your marginfi account (must have zero balances).

```bash
cargo run --bin mfi account close
```

---

## Query Commands

### Get Group Info

```bash
# Get specific group
cargo run --bin mfi group get <GROUP_PUBKEY>

# Get group from profile
cargo run --bin mfi group get

# Get all groups
cargo run --bin mfi group get-all
```

### Get Bank Info

```bash
# Get specific bank
cargo run --bin mfi bank get <BANK_PUBKEY>

# Get all banks in a group
cargo run --bin mfi bank get-all <GROUP_PUBKEY>

# Get all banks in profile's group
cargo run --bin mfi bank get-all
```

### Get Account Info

```bash
# Get specific account
cargo run --bin mfi account get <ACCOUNT_PUBKEY>

# Get account from profile
cargo run --bin mfi account get

# List all accounts
cargo run --bin mfi account list
```

### Inspect Oracle

```bash
cargo run --bin mfi bank inspect-price-oracle <BANK_PUBKEY>
```

### Show Oracle Ages

```bash
# Show all oracle ages
cargo run --bin mfi show-oracle-ages

# Show only stale oracles
cargo run --bin mfi show-oracle-ages --only-stale
```

---

## Token Commands

Utility commands for creating and managing SPL tokens. Useful for testing and development.

### 1. Create Token

Create a new SPL token mint.

```bash
cargo run --bin mfi token create
```

**With optional parameters:**
```bash
cargo run --bin mfi token create \
  --decimals 6 \
  --mint-authority <AUTHORITY_PUBKEY> \
  --freeze-authority <FREEZE_AUTHORITY_PUBKEY>
```

**Parameters:**
| Parameter | Description | Default |
|-----------|-------------|---------|
| `--decimals` | Number of decimal places for the token | `9` |
| `--mint-authority` | Pubkey that can mint new tokens | Your wallet |
| `--freeze-authority` | Pubkey that can freeze token accounts | None |

**Example output:**
```
Token mint created successfully!
Mint address: 7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU
Decimals: 9
Mint authority: BsHLjrzrStpW5aWgGh9qRt8bQtLiymfUcKqS5d6nKG9f
Signature: 5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp...
```

---

### 2. Mint Tokens

Mint tokens to a destination wallet. Creates the Associated Token Account (ATA) if it doesn't exist.

```bash
cargo run --bin mfi token mint \
  --mint <MINT_ADDRESS> \
  --amount <AMOUNT> \
  --to <DESTINATION_WALLET>
```

**Example - Mint 1000 tokens:**
```bash
cargo run --bin mfi token mint \
  --mint 7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU \
  --amount 1000.0 \
  --to BsHLjrzrStpW5aWgGh9qRt8bQtLiymfUcKqS5d6nKG9f
```

**Parameters:**
| Parameter | Description |
|-----------|-------------|
| `--mint` | The token mint address |
| `--amount` | Amount to mint in UI units (e.g., `100.5`) |
| `--to` | Destination wallet address (ATA will be created if needed) |

**Note:** You must be the mint authority to mint tokens.

---

### 3. Transfer Tokens

Transfer tokens from your wallet to another wallet.

```bash
cargo run --bin mfi token transfer \
  --mint <MINT_ADDRESS> \
  --amount <AMOUNT> \
  --to <DESTINATION_WALLET>
```

**Example - Transfer 50 tokens:**
```bash
cargo run --bin mfi token transfer \
  --mint 7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU \
  --amount 50.0 \
  --to 9aE476sH92Vz7DMPyq5WLPkrKWivxeuTKEFKd2sZZcde
```

**Parameters:**
| Parameter | Description |
|-----------|-------------|
| `--mint` | The token mint address |
| `--amount` | Amount to transfer in UI units |
| `--to` | Destination wallet address (ATA will be created if needed) |

---

### 4. Check Balance

Get the token balance for a wallet.

```bash
cargo run --bin mfi token balance \
  --mint <MINT_ADDRESS>
```

**Check another wallet's balance:**
```bash
cargo run --bin mfi token balance \
  --mint <MINT_ADDRESS> \
  --owner <WALLET_ADDRESS>
```

**Parameters:**
| Parameter | Description | Default |
|-----------|-------------|---------|
| `--mint` | The token mint address | Required |
| `--owner` | Wallet address to check | Your wallet |

**Example output:**
```
Token Balance
=============
Mint: 7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU
Owner: BsHLjrzrStpW5aWgGh9qRt8bQtLiymfUcKqS5d6nKG9f
Token Account (ATA): 3fGHmKE8LHr5Bxyz...
Balance: 1000 (native: 1000000000000)
Decimals: 9
```

---

## Parameter Reference

### Risk Tiers

| Tier | Description |
|------|-------------|
| `collateral` | Can be used as collateral for borrowing |
| `isolated` | Cannot be used as collateral, isolated risk |

### Operational States

| State | Description |
|-------|-------------|
| `operational` | Normal operation, deposits and borrows enabled |
| `paused` | All operations paused |
| `reduce-only` | Only withdrawals and repayments allowed |

### Asset Tags

| Tag | Description |
|-----|-------------|
| `0` | Default |
| `1` | SOL |
| `2` | Staked SOL LST |

### Weight Parameters

- **Asset weights** (0-1): Higher = more collateral value. `0.8` means 80% of deposit counts as collateral.
- **Liability weights** (>1): Higher = more conservative. `1.2` means debt counts as 120% for margin calculations.

### Interest Rate Model

The interest rate follows a kinked curve:
1. Below optimal utilization: Rate increases linearly to `plateau-interest-rate`
2. Above optimal utilization: Rate increases steeply to `max-interest-rate`

---

## Example: Complete Setup Flow

```bash
# 1. Create profile
cargo run --bin mfi profile create \
  --name devnet \
  --cluster devnet \
  --keypair-path ~/.config/solana/id.json \
  --rpc-url https://api.devnet.solana.com \
  --program-id 4sbjN16fgtnGMFaJ6s7aw9ha1uUQ9qLBv5gHSui2aNUU

# 2. Initialize fee state (one-time)
cargo run --bin mfi group init-fee-state \
  --admin <YOUR_WALLET> \
  --fee-wallet <YOUR_WALLET> \
  --bank-init-flat-sol-fee 1000 \
  --liquidation-flat-sol-fee 500 \
  --program-fee-fixed 0.01 \
  --program-fee-rate 0.02 \
  --liquidation-max-fee 0.5

# 3. Create group
cargo run --bin mfi group create

# 4. Add WSOL bank
cargo run --bin mfi group add-bank \
  --mint So11111111111111111111111111111111111111112 \
  --seed \
  --asset-weight-init 0.8 \
  --asset-weight-maint 0.9 \
  --liability-weight-init 1.2 \
  --liability-weight-maint 1.1 \
  --deposit-limit-ui 1000000 \
  --borrow-limit-ui 500000 \
  --optimal-utilization-rate 0.8 \
  --plateau-interest-rate 0.1 \
  --max-interest-rate 3.0 \
  --insurance-fee-fixed-apr 0.01 \
  --insurance-ir-fee 0.02 \
  --group-fixed-fee-apr 0.01 \
  --group-ir-fee 0.02 \
  --risk-tier collateral \
  --oracle-max-age 60 \
  --global-fee-wallet <YOUR_WALLET>

# 5. Configure oracle
cargo run --bin mfi bank update-oracle <BANK_PUBKEY> \
  --oracle-type 3 \
  --oracle-key <PYTH_SOL_USD_ORACLE>

# 6. Create user account
cargo run --bin mfi account create

# 7. Deposit
cargo run --bin mfi account deposit <BANK_PUBKEY> 10.0
```

---

## Troubleshooting

### "AccountOwnedByWrongProgram" Error
This usually means the CLI was built with a different program ID than your deployed program. Rebuild with the correct feature:
```bash
cargo clean
cargo build -p marginfi-v2-cli --release --no-default-features --features devnet
```

### Profile Not Found
Make sure you've created and set a profile:
```bash
cargo run --bin mfi profile list
cargo run --bin mfi profile set <PROFILE_NAME>
```

### Transaction Confirmation Required
Most write operations require typing the profile name to confirm. Use `--skip-confirmation` to bypass (use with caution):
```bash
cargo run --bin mfi -- --skip-confirmation group create
```
