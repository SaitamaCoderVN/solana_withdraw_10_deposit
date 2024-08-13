# Solana Withdraw 10 Deposit

This is a Solana project using the Anchor framework to create a program that allows users to deposit and withdraw funds.

## Description

This project implements a smart contract on the Solana blockchain that enables:
- Users to deposit SOL into a vault account
- Withdrawal of funds from the vault (under certain conditions)

## Installation

1. Install Rust and Solana CLI
2. Install Anchor Framework:
   ```
   npm install -g @coral-xyz/anchor-cli
   ```
3. Clone the repository and install dependencies:
   ```
   git clone <repository-url>
   cd solana_withdraw_10_deposit
   npm install
   ```

## Development

- Edit the smart contract in `programs/solana_withdraw_10_deposit/src/lib.rs`
- Write tests in the `tests/` directory
- Run tests:
   ```
   anchor test
   ```

## Deployment

Use the deployment script in `migrations/deploy.ts`:
   ```
   anchor deploy
   ```

## Technologies Used

- Solana
- Anchor Framework
- Rust
- TypeScript

## Contributing

Contributions are welcome. Please open an issue or create a pull request.

## License

[MIT](https://choosealicense.com/licenses/mit/)