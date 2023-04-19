use crate::{errors::AccountingError, tx::Tx};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Accounts {
    accounts: HashMap<String, u64>,
}

impl Accounts {
    pub fn new() -> Self {
        Accounts {
            accounts: Default::default(),
        }
    }
    pub fn deposit(&mut self, signer: &str, amount: u64) -> Result<Tx, AccountingError> {
        if let Some(account) = self.accounts.get_mut(signer) {
            (*account)
                .checked_add(amount)
                .map(|r| {
                    *account = r;
                    r
                })
                .ok_or(AccountingError::AccountOverFunded(
                    signer.to_string(),
                    amount,
                ))
                .map(|_| Tx::Deposit {
                    account: signer.to_string(),
                    amount,
                })
        } else {
            self.accounts.insert(signer.to_string(), amount);
            Ok(Tx::Deposit {
                account: signer.to_string(),
                amount,
            })
        }
    }

    pub fn withdraw(&mut self, signer: &str, amount: u64) -> Result<Tx, AccountingError> {
        if let Some(account) = self.accounts.get_mut(signer) {
            (*account)
                .checked_sub(amount)
                .map(|r| {
                    *account = r;
                    r
                })
                .ok_or(AccountingError::AccountUnderFunded(
                    signer.to_string(),
                    amount,
                ))
                .map(|_| Tx::Withdraw {
                    account: signer.to_string(),
                    amount,
                })
        } else {
            Err(AccountingError::AccountNotFound(signer.to_string()))
        }
    }

    pub fn send(
        &mut self,
        sender: &str,
        recipient: &str,
        amount: u64,
    ) -> Result<(Tx, Tx), AccountingError> {
        if !self.accounts.contains_key(sender) {
            return Err(AccountingError::AccountNotFound(sender.to_string()));
        }

        if !self.accounts.contains_key(recipient) {
            return Err(AccountingError::AccountNotFound(recipient.to_string()));
        }

        let withdraw = self.withdraw(sender, amount)?;
        let deposit = self.deposit(recipient, amount)?;
        Ok((withdraw, deposit))
    }
}

#[cfg(test)]
mod tests {}
