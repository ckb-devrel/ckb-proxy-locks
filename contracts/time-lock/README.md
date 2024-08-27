# Time-Locked Script

## Overview

This script implements a time-lock functionality on the Nervos CKB blockchain, ensuring that certain conditions involving time and specific lock script hashes are met before a transaction can be processed.

## Features

- **Time-Locking**: Assets are restricted from movement until a specified timestamp has passed.
- **Lock Script Verification**: Ensures transaction processing involves a specific lock script hash.

## Usage

**Steps to lock**:
1. Include the script as a lock script in the outputs of a transaction.
2. Pass the timestamp until which the assets should remain locked as the first 8 bytes of the script arguments.
3. Pass the hash of the lock script that must be present in the inputs of the transaction as the remaining 32 bytes of the script arguments.
4. Send the transaction to the blockchain.

**Steps to unlock**:
1. Include a cell output locked by the time-lock Script as one of the cell inputs in a transaction.
2. Include another cell output that is locked by the lock script hash specified in the time-lock script as another cell input in the transaction.
3. Set the current timestamp in at least one of the `since` fields of the cell inputs.
4. Send the transaction to the blockchain.
