# CKB Proxy Locks [![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/ckb-devrel/ckb-proxy-locks)

A collection of utility lock scripts and type scripts for the Nervos CKB blockchain. This project provides various proxy mechanisms, time-based locks, and other useful smart contract primitives.

## Overview

This repository contains 8 different smart contracts designed to provide flexible locking mechanisms and utility functions for CKB applications:

### Lock Scripts

1. **[Lock Proxy Lock](#lock-proxy-lock)** - Delegates unlocking authority to another lock script
2. **[Input Type Proxy Lock](#input-type-proxy-lock)** - Unlocks when a specific type script appears in transaction inputs
3. **[Output Type Proxy Lock](#output-type-proxy-lock)** - Unlocks when a specific type script appears in transaction outputs
4. **[Single Use Lock](#single-use-lock)** - Can only be unlocked by consuming a specific outpoint
5. **[Time Lock](#time-lock)** - Since-based lock (time/block/epoch) with additional lock script verification
6. **[Type Burn Lock](#type-burn-lock)** - Unlocks when a specific type script is burned (appears in inputs but not outputs)

### Type Scripts

7. **[Easy to Discover Type](#easy-to-discover-type)** - Reveals cell data through type script arguments

### Utility Scripts

8. **[Always Success](#always-success)** - A simple script that always succeeds (for testing purposes)

## Quick Start

### Prerequisites

- Rust toolchain with `riscv64imac-unknown-none-elf` target
- Clang (for building)
- Make

### Installation

1. Clone the repository:
```bash
git clone https://github.com/ckb-ecofund/ckb-proxy-locks.git
cd ckb-proxy-locks
```

2. Install the required Rust target:
```bash
make prepare
```

3. Build all contracts:
```bash
make build
```

4. Run tests:
```bash
make test
```

### Building Individual Contracts

To build a specific contract:
```bash
make build CONTRACT=lock-proxy-lock
```

## Contract Details

### Lock Proxy Lock

**Purpose**: Delegates unlocking authority to another lock script.

**How it works**:
- Takes a 32-byte lock script hash as argument
- Can be unlocked if any input cell uses the specified lock script
- Useful for creating hierarchical permission systems

**Arguments**:
- `args[0..32]`: Hash of the owner lock script

**Use cases**:
- Multi-signature wallets
- Delegated authority systems
- Hierarchical access control

### Input Type Proxy Lock

**Purpose**: Unlocks when a specific type script appears in transaction inputs.

**How it works**:
- Takes a 32-byte type script hash as argument
- Can be unlocked if any input cell has the specified type script
- Enables type-script-based authorization

**Arguments**:
- `args[0..32]`: Hash of the required input type script

**Use cases**:
- Token-gated access
- NFT-based permissions
- Conditional unlocking based on asset ownership

### Output Type Proxy Lock

**Purpose**: Unlocks when a specific type script appears in transaction outputs.

**How it works**:
- Takes a 32-byte type script hash as argument
- Can be unlocked if any output cell has the specified type script
- Useful for ensuring certain assets are created in the transaction

**Arguments**:
- `args[0..32]`: Hash of the required output type script

**Use cases**:
- Ensuring token minting
- Conditional payments
- Asset creation requirements

### Single Use Lock

**Purpose**: Can only be unlocked by consuming a specific outpoint.

**How it works**:
- Takes an outpoint (36 bytes) as argument
- Can only be unlocked if the specified outpoint appears in transaction inputs
- Provides one-time unlock capability

**Arguments**:
- `args[0..36]`: The specific outpoint that must be consumed

**Use cases**:
- One-time payments
- Voucher systems
- Single-use authorizations

### Time Lock

**Purpose**: Time-based lock with additional lock script verification.

**How it works**:
- Requires both time conditions and lock script presence
- Takes a lock script hash (32 bytes) and since value (8 bytes) as arguments
- Can only be unlocked after the specified since condition is met AND when the required lock script is present
- Uses CKB's `since` field mechanism for time/block-based constraints

**Arguments**:
- `args[0..32]`: Hash of the required lock script
- `args[32..40]`: Since value (8 bytes, little-endian) - can represent block number, epoch, or timestamp

**Use cases**:
- Vesting schedules
- Time-delayed payments
- Escrow with time conditions

### Type Burn Lock

**Purpose**: Unlocks when a specific type script is burned (destroyed).

**How it works**:
- Takes a 32-byte type script hash as argument
- Can be unlocked only when the specified type script appears in inputs but NOT in outputs
- Ensures the type script is consumed/burned in the transaction

**Arguments**:
- `args[0..32]`: Hash of the type script that must be burned

**Use cases**:
- Token burning mechanisms
- Proof of destruction
- Conditional unlocking based on asset burning

### Easy to Discover Type

**Purpose**: A type script that reveals cell data through its arguments.

**How it works**:
- Takes a 32-byte data hash as argument
- Validates that all output cells with this type script have data matching the hash
- Makes cell data discoverable through the type script arguments

**Arguments**:
- `args[0..32]`: Hash of the expected cell data

**Use cases**:
- Data integrity verification
- Making cell data easily discoverable
- Content addressing

### Always Success

**Purpose**: A utility script that always succeeds.

**How it works**:
- Simply returns success (0) without any validation
- Used primarily for testing and development

**Use cases**:
- Testing and development
- Placeholder scripts
- Unconditional success scenarios

## Development

### Project Structure

```
├── contracts/           # Individual contract implementations
│   ├── always-success/
│   ├── easy-to-discover-type/
│   ├── input-type-proxy-lock/
│   ├── lock-proxy-lock/
│   ├── output-type-proxy-lock/
│   ├── single-use-lock/
│   ├── time-lock/
│   └── type-burn-lock/
├── tests/              # Integration tests
├── build/              # Compiled binaries
├── scripts/            # Build scripts
└── deployment.toml     # Deployment configuration
```

### Building

The project uses a workspace structure with individual Makefiles for each contract. The root Makefile coordinates building all contracts.

**Build all contracts:**
```bash
make build
```

**Build in debug mode:**
```bash
make build MODE=debug
```

**Build specific contract:**
```bash
make build CONTRACT=time-lock
```

### Testing

Tests are located in the `tests/` directory and use the `ckb-testtool` framework.

**Run all tests:**
```bash
make test
```

**Run tests with output:**
```bash
make test CARGO_ARGS="-- --nocapture"
```

### Code Quality

**Check code:**
```bash
make check
```

**Run clippy:**
```bash
make clippy
```

**Format code:**
```bash
make fmt
```

## Deployment

The project includes a `deployment.toml` configuration file for deploying contracts to CKB networks. All contracts are configured with `enable_type_id = false` for simpler deployment.

### Generating Checksums

For reproducible builds, you can generate checksums:

```bash
make checksum
```

This creates a `build/checksums-release.txt` file with SHA256 hashes of all built contracts.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass: `make test`
6. Submit a pull request

### Adding New Contracts

To generate a new contract from template:

```bash
make generate CRATE=my-new-contract
```

This will create a new contract in `contracts/my-new-contract/` and update the workspace configuration.

## License

This project is open source. Please check the individual contract files for specific license information.

## Acknowledgments

*This project was bootstrapped with [ckb-script-templates].*

[ckb-script-templates]: https://github.com/cryptape/ckb-script-templates
