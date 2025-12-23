use std::io::{self, Write};

use crate::bank::Bank;
mod accounts;
mod bank;
mod tools;

fn main() -> Result<(), std::io::Error> {
    let mut bank = Bank::new("Capitalist Spriggy".to_string());
    loop {
        println!("\n🏦 Welcome to {} Bank (NOT FDIC INSURED)", bank.owner);
        println!(
            "Choose an option:
open
deposit
withdraw
tax
about
exit"
        );

        let cmd = read_line("> ")?;

        match cmd.as_str() {
            "open" => {
                let name = read_line("Account name: ")?;
                let deposit = read_f64("Initial deposit: ")?;

                if let Err(_) = bank.create_accout(name, deposit) {
                    println!("❌ Account creation failed");
                }
            }

            "deposit" => {
                let name = read_line("Account name: ")?;
                let amount = read_f64("Deposit amount: ")?;

                if let Err(_) = bank.deposit_money(amount, name) {
                    println!("❌ Deposit failed");
                }
            }

            "withdraw" => {
                let name = read_line("Account name: ")?;
                let amount = read_f64("Withdraw amount: ")?;

                if let Err(_) = bank.withdraw_money(amount, name) {
                    println!("Withdrawal failed!");
                }
            }

            "tax" => {
                println!("📄 Bank tax status: {}", bank.tax_info());
            }

            "about" => {
                println!("{}", bank);
            }

            "exit" => {
                println!("Bank closed. Funds mysteriously missing...");
                break;
            }

            _ => println!("Unknown command, try again!"),
        }
    }

    Ok(())
}

fn read_line(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn read_f64(prompt: &str) -> io::Result<f64> {
    loop {
        let input = read_line(prompt)?;
        match input.parse::<f64>() {
            Ok(v) => return Ok(v),
            Err(_) => println!("That is NOT a Number lil dummy"),
        }
    }
}
