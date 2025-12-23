use std::fmt::Display;

use crate::tools::rfloat::{RESTRICT_DIGITS, RFloat};
#[derive(Debug)]
pub struct Accounts {
    pub owner: String,
    pub balance: RFloat,
}

impl Accounts {
    pub fn new(name: String, initial_deposit: f64) -> Option<Self> {
        if initial_deposit < 200.0 {
            println!("Need at least 200 to make an Account!");
            return None;
        }

        let restricted = RFloat::new(initial_deposit, RESTRICT_DIGITS)?;

        Some(Accounts {
            owner: name,
            balance: restricted,
        })
    }
    pub fn deposit(&mut self, deposit: f64) -> () {
        self.balance.float += deposit;
        self.balance.restrict_self();
    }
    pub fn withdraw(&mut self, withdraw_amount: f64) -> Result<(), ()> {
        // <--- Ik thats cursed, thast the POINT
        if self.balance.float - withdraw_amount < 0.0 {
            println!("You can't overdraft!");
            return Err(());
        } else {
            self.balance.float -= withdraw_amount;
            self.balance.restrict_self();
        }
        Ok(())
    }
}

impl Display for Accounts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Account Owner: {}\nBalance: {}",
            self.owner, self.balance
        )
    }
}
