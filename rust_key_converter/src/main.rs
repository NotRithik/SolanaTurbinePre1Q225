use bs58;
use std::io::{self, BufRead};

fn base58_to_wallet() {
    println!("Enter your name:");
    let stdin = io::stdin();
    let base58 = stdin.lock().lines().next().unwrap().unwrap(); // gdtKSTXYULQNx87fdD3YgXkzVeyFeqwtxHm6WdEb5a9YJRnHse7GQr7t5pbepsyvUCk7VvksUGhPt4SZ8JHVSkt
    let wallet = bs58::decode(base58).into_vec().unwrap();
    println!("{:?}", wallet);
}

fn wallet_to_base58() {
    let wallet: Vec<u8> = vec![
        180, 212, 158, 51, 93, 65, 123, 43, 146, 152, 30, 226, 53, 39, 183, 64, 199, 175, 147, 54,
        135, 83, 147, 164, 104, 207, 221, 143, 155, 235, 8, 64, 146, 11, 178, 227, 169, 192, 105,
        28, 0, 249, 174, 243, 126, 190, 11, 92, 46, 148, 142, 205, 7, 90, 132, 245, 171, 158, 189,
        154, 248, 42, 26, 170,
    ];
    let base58 = bs58::encode(wallet).into_string();
    println!("{:?}", base58);
}

fn main() {
    base58_to_wallet();
    wallet_to_base58();
}
