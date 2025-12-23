use std::fmt::Display;

use crate::accounts::Accounts;

#[derive(Debug)]
pub struct Bank {
    pub owner: String,
    pub accounts: Vec<Accounts>,
}
impl Display for Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let total_funds = self.total_funds(); // <--- TAX EVASI-(gets FBI'ed)
        write!(
            f,
            "{} owns the bank and the bank has: {} accounts\nTotal funds: {}",
            self.owner,
            self.accounts.iter().count(),
            total_funds
        )
    }
}

impl Bank {
    pub fn new(owner: String) -> Self {
        Bank {
            owner,
            accounts: Vec::new(),
        }
    }
    pub fn create_accout(&mut self, name: String, initial_deposit: f64) -> Result<(), ()> {
        let account = match Accounts::new(name, initial_deposit) {
            Some(account) => account,
            None => {
                self.owner_talks("There was a trouble creating your account!");
                return Err(());
            }
        };

        self.accounts.push(account);
        self.owner_talks("Account Created Successfully!");
        Ok(())
    }
    //I'm too lazy to make a giant ThisError Enum
    pub fn withdraw_money(&mut self, withdraw_amount: f64, name: String) -> Result<(), ()> {
        let account = match self.find_account_with_name(&name) {
            Some(at) => &mut self.accounts[at],
            None => {
                println!("No Account with the name {} Found!", name);
                return Err(());
            }
        };
        account.withdraw(withdraw_amount)?;
        self.owner_talks("WithDraw Successful!");
        Ok(())
    }
    pub fn deposit_money(&mut self, deposit_amount: f64, name: String) -> Result<(), ()> {
        let account = match self.find_account_with_name(&name) {
            Some(at) => &mut self.accounts[at],
            None => {
                println!("No Account with the name {} Found!", name);
                return Err(());
            }
        };
        account.deposit(deposit_amount);
        self.owner_talks("Deposit Successful!");
        Ok(())
    }
    fn owner_talks(&self, words: &str) -> () {
        println!("{}: {}", self.owner, words)
    }
    pub fn tax_info(&self) -> TaxObligations {
        let total_funds = self.total_funds();
        if total_funds > 1000 {
            TaxObligations::NonCompliant
        } else {
            TaxObligations::VeryNonCompliant
        }
    }
    fn total_funds(&self) -> i32 {
        let total_funds = self.accounts.iter().fold(0, |acc: i32, x: &Accounts| {
            acc + x.balance.float.floor() as i32
        });
        total_funds
    }
    //returns index to the vec that corresponds to the name;
    fn find_account_with_name(&self, name: &String) -> Option<usize> {
        for (index, accoutns) in self.accounts.iter().enumerate() {
            if accoutns.owner == *name {
                return Some(index);
            }
        }
        return None;
    }
}
#[derive(Debug)]
pub enum TaxObligations {
    NonCompliant,
    VeryNonCompliant,
}

impl Display for TaxObligations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match self {
            Self::NonCompliant => "NonCompliant",
            _ => "VeryNonCompliant",
        };
        write!(f, "{}", status)
    }
}
