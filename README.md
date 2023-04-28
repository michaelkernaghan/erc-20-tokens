
# Ethereum ERC20 Token Interaction and Transaction History

This Rust project demonstrates how to interact with an Ethereum ERC20 token and retrieve transaction history for a specific user address. The code uses the Web3 library to connect to an Ethereum node and interact with the ERC20 token contract, as well as retrieve transaction details.

## Features

- Connect to an Ethereum node using Infura.
- Interact with an ERC20 token contract to get token details (name, symbol) and user's token balance.
- Retrieve transaction history for a given user address and a specified number of blocks.

## Prerequisites

- Rust programming language (https://www.rust-lang.org/tools/install)
- An Ethereum node provider (e.g., Infura) with an API key

## Dependencies

- web3 (0.15.0)
- tokio (1.0)

## Usage

1. Clone this repository:

```bash
git clone https://github.com/michaelkernaghan/ethereum_erc20_interaction.git
```

2. Change the working directory:

```bash
cd ethereum_erc20_interaction
```

3. Update the Ethereum node URL in the `connect_to_ethereum_node()` function with your API key:

```rust
let transport = Http::new(
    "https://mainnet.infura.io/v3/YOUR_API_KEY"
).unwrap();
```

4. Compile the project:

```bash
cargo build
```

5. Run the project:

```bash
cargo run
```

The output will display the ERC20 token details, user's token balance, and transaction history for the specified user address and number of blocks.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[Unlicense](https://github.com/unlicense)