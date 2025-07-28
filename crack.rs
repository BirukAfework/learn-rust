use std::env;
use std::fs::files;
use std::io::{BuffRead, BuffReader};
use sha2::{Sha256, Digest};
use std::process::exit;

fn main() {

    let  args:Vec<String> = env::args().collect();

    if args.len() != 2{
        Println!("Invalid amount of agruments");
        Println!("[+] Usage: Cargo run pass-hash (Sha256)");
        exit(1);
    }
     
    let wanted_hash =&args
}