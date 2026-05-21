use std::env;
use std::str::FromStr;
use serde::Deserialize;
// Solana SDK
use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, native_token::LAMPORTS_PER_SOL};

// структуры для работы с json ({"solana":{"usd":86.15}})
#[derive(Deserialize, Debug)]
struct BinancePrice {price:String,}

// Функция для получения боланса кошелька вход -> Адрес кошелька, Клиент
fn grep_balance(ax0:&String, cx1:&RpcClient) -> f64 {
	 let pubkey = Pubkey::from_str(ax0).expect("invalid addres"); // Парсим адресс string в тип
	 let lamports = cx1.get_balance(&pubkey).expect("RPC Error");
	 lamports as f64 / LAMPORTS_PER_SOL as f64	
}

// Функция возвращает цену Sol в долларах вход -> Сылка Апи, Структура json 
fn grep_solana_price(reqwest_link:&str) -> BinancePrice {
	let response = reqwest::blocking::get(reqwest_link).expect("Er:Line:23");
	let data: BinancePrice = response.json().expect("Er:Line:24");
	data
} 

// вывод
fn output(sol_usdt:f64,sol_in_wallet:f64){
	println!(
	"\n\
	 +=================================+\n\
	 === SOLANA WALLET            ===\n\
	 === {:<24} ===\n\
	 === {:<24} ===\n\
	 +================================+"
	,sol_in_wallet,sol_in_wallet*sol_usdt );
}


fn main() {
	let args: Vec<String> = env::args().collect(); // Аргумент командной строку
	if args.len() < 2 {eprintln!("no wallet addres");return;} // Есть ли аргументы 
	let wallet_addres = args[1].clone().to_string(); // Адрес кошелька
	let rpc = "https://api.mainnet.solana.com".to_string(); // RPC нода solana mainnet
	let client = RpcClient::new(rpc);// Клиент --> RPC нода 
	// HTTP client ссылка для цену solana / usd
	let b_rpc = "https://api.binance.com/api/v3/ticker/price?symbol=SOLUSDT";
	let price_data = grep_solana_price(b_rpc); // берем данные из бинанса
	let current_price:f64 = price_data.price.parse().unwrap_or(0.0); // достаем из структуры данные и парсим их
	output(current_price, grep_balance(&wallet_addres,&client));
	
}


