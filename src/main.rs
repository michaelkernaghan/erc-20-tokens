mod erc20_abi;
use web3::transports::Http;
use web3::Web3;
use web3::types::Address;
use crate::erc20_abi::ERC20_ABI;
use web3::ethabi::Contract as EthabiContract;

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

#[tokio::main]
async fn main() {
    let web3 = connect_to_ethereum_node().await;
    let token_address = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"; // USDC
    let user_address = "0x54263d2C8D849006C97De20D3Eb9A68D90Fdb5A0"; // mk personal
    //let user_address = "0x7b0Dd9A2E4Bd593071DC3f67f8F02f4264608280"; // eg personal
    interact_with_erc20(&web3, token_address, user_address).await;
}