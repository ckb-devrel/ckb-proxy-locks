# Time-Locked Script

## Overview

This script implements a time-lock Script on the Nervos CKB blockchain, ensuring that cells locked by this Script can only be unlocked after a specified time has passed and certain lock script hash is validated.

## Features

- **Time-Locking**: Assets are restricted from movement until a specified timestamp has passed.
- **Lock Script Verification**: Ensures transaction processing involves a specific lock script hash.

## Usage

**Steps to lock**:
1. Include the Script as a lock script in the outputs of a transaction.
2. Pass the hash of the lock script that must be present in the inputs of the transaction as the first 32 bytes of the script arguments.
3. Pass the timestamp until which the assets should remain locked as the remaining 8 bytes of the script arguments.
4. Send the transaction to the blockchain.

**Steps to unlock**:
1. Include a cell output locked by the Script as one of the cell inputs in a transaction.
2. Include a cell output that is locked by the lock script, whose hash is specified in the time-lock Script's arguments, as another cell input in the transaction.
3. Set the [`since` field](https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0017-tx-valid-since/0017-tx-valid-since.md) of each of the cell input.
4. Send the transaction to the blockchain.
