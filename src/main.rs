mod erc20_abi;
use web3::transports::Http;
use web3::Web3;
use web3::types::Address;
use crate::erc20_abi::ERC20_ABI;
use web3::ethabi::Contract as EthabiContract;
use web3::types::{ BlockId, BlockNumber, Transaction };

async fn connect_to_ethereum_node() -> Web3<Http> {
    let transport = Http::new(
        "https://mainnet.infura.io/v3/a0c2d9cf999a42a2abab53016a992e8a"
    ).unwrap();
    Web3::new(transport)
}

use web3::contract::Contract;

fn create_token_contract(web3: &Web3<Http>, token_address: &str) -> Contract<Http> {
    let address: Address = token_address.parse().unwrap();
    let abi_bytes = ERC20_ABI.as_bytes();
    let web3_ethabi_contract: EthabiContract = serde_json::from_slice(abi_bytes).unwrap();
    Contract::new(web3.eth(), address, web3_ethabi_contract)
}

async fn interact_with_erc20(web3: &Web3<Http>, token_address: &str, user_address: &str) {
    let token_contract = create_token_contract(web3, token_address);
    // Get token name
    let token_name: String = token_contract
        .query("name", (), None, web3::contract::Options::default(), None).await
        .unwrap();
    println!("Token name: {}", token_name);
    // Get token symbol
    let token_symbol: String = token_contract
        .query("symbol", (), None, web3::contract::Options::default(), None).await
        .unwrap();
    println!("Token symbol: {}", token_symbol);
    // Get user's token balance
    let user_address: Address = user_address.parse().unwrap();
    let balance: web3::types::U256 = token_contract
        .query("balanceOf", user_address, None, web3::contract::Options::default(), None).await
        .unwrap();
    println!("User's token balance: {}", balance);
}

async fn get_transactions(web3: &Web3<Http>, user_address: &str, from_block: u64) {
    let user_address: Address = user_address.parse().unwrap();
    let mut transactions: Vec<Transaction> = vec![];
    let latest_block_number = web3.eth().block_number().await.unwrap();
    let start_block = web3::types::U256::from(latest_block_number.as_u64()) - web3::types::U256::from(from_block as u64); // Added from_block
    for block_number in start_block.as_u64()..=latest_block_number.as_u64() {
        println!("Checking block number: {}", block_number); // Added print statement
        let block = web3
            .eth()
            .block(BlockId::Number(BlockNumber::Number(block_number.into())))
            .await
            .unwrap();
        if let Some(block) = block {
            for transaction_hash in block.transactions {
                let transaction = web3.eth().transaction(web3::types::TransactionId::Hash(transaction_hash)).await.unwrap();
                if let Some(transaction) = transaction {
                    if transaction.from == user_address || transaction.to == Some(user_address) {
                        transactions.push(transaction);
                    }
                }
            }
        }
    }
    println!("Found {} transactions", transactions.len());
    for (index, transaction) in transactions.iter().enumerate() {
        println!(
            "Transaction {}: Hash: {:?}, From: {:?}, To: {:?}, Value: {:?}",
            index + 1,
            transaction.hash,
            transaction.from,
            transaction.to,
            transaction.value
        );
    }
}

#[tokio::main]
async fn main() {
    let web3 = connect_to_ethereum_node().await;
    let token_address = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"; // USDC
    let user_address = "0x3063A09bdf5A290eA13858Bf961C1C36ddc83D6a"; 
    let from_block = 	100; // how many blocks to look backward   
    interact_with_erc20(&web3, token_address, user_address).await;
    get_transactions(&web3, user_address, from_block).await;
}