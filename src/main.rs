extern crate sha2;

use sha2::{Sha256, Digest};
use std::io::{self, BufRead};
use std::fs::File;
use std::process::exit;

fn main() {
    let passfile = "src/rockyou.txt";

    let mut pass = String::new();
    println!("Enter the password you want to crack: ");
    io::stdin().read_line(&mut pass).expect("Failed to read input");
    let pass = pass.trim(); // Usamos una nueva variable `pass` para almacenar la contraseña.

    let mut attempts = 1;

    println!("Attempting to crack: {}!\n", pass);

    let passlist = File::open(passfile).expect("Failed to open password file");
    let readpasslist = io::BufReader::new(passlist);

    for line in readpasslist.lines() {
        let line = line.expect("Failed to read line");
        let password = line.trim(); // No necesitas convertirlo en bytes, ya que ya es un String.
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        let password_hash = format!("{:x}", hasher.finalize());

        println!("[{}] {} == {}", attempts, password, password_hash);

        if &password_hash == pass {
            println!("Password found! {} hashes to {} in {} attempts.", password, password_hash, attempts);
            exit(0);
        }
        attempts += 1;
    }

    println!("Password not found.");
}
