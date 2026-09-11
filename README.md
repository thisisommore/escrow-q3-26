# Escrow

This is basic escrow supporting trade between two tokens between two users.
For example user a wants to sell 1 sol and get 100 USDC

## Accounts

### Ecrow

This stores all makers params like sell mint buy mint, receive amount and others which are required for swap between two parties.

### Vault

This is Token Account which stores user funds.
The authority is escrow

## Instructions

### Make

This creates escrow and deposit funds into vault

## Update

This updates receive amount from maker.

### Refund

This returns funds from vault to user and detroys accounts

### Take

This transfers funds from taker to makers account and from vault to takes account and closes accounts.
This completes the escrow process.

## Tests

Tests are inside tests folder and using SVMLite
Run `anchor test` to test.
