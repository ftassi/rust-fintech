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
mod tests {
    use super::Accounts;
    use crate::{errors::AccountingError, tx::Tx};

    #[test]
    fn cannot_withdraw_from_non_existent_account() {
        let mut accounts = Accounts::new();
        assert_eq!(
            accounts.withdraw("Alice", 100).unwrap_err(),
            AccountingError::AccountNotFound("Alice".to_string())
        );
    }

    #[test]
    fn cannot_send_from_non_existent_account() {
        let mut accounts = Accounts::new();
        accounts.deposit("Bob", 100).unwrap();
        assert_eq!(
            accounts.send("Alice", "Bob", 100).unwrap_err(),
            AccountingError::AccountNotFound("Alice".to_string())
        );
    }

    #[test]
    fn account_cannot_be_underfunded() {
        let mut accounts = Accounts::new();
        accounts.deposit("Alice", 100).unwrap();
        accounts.deposit("Bob", 100).unwrap();

        assert_eq!(
            accounts.send("Alice", "Bob", 101).unwrap_err(),
            AccountingError::AccountUnderFunded("Alice".to_string(), 101)
        );

        assert_eq!(
            accounts.withdraw("Alice", 101).unwrap_err(),
            AccountingError::AccountUnderFunded("Alice".to_string(), 101)
        );
    }

    #[test]
    fn account_cannot_be_overfunded() {
        let mut accounts = Accounts::new();
        accounts.deposit("Alice", u64::MAX).unwrap();
        assert_eq!(
            accounts.deposit("Alice", 1).unwrap_err(),
            AccountingError::AccountOverFunded("Alice".to_string(), 1)
        );
    }

    #[test]
    fn deposit() {
        let mut accounts = Accounts::new();
        let tx = accounts.deposit("Alice", 100).unwrap();

        assert_eq!(
            tx,
            Tx::Deposit {
                account: "Alice".to_string(),
                amount: 100,
            }
        );
        assert_eq!(accounts.accounts.get("Alice"), Some(&100));
    }

    #[test]
    fn withdraw() {
        let mut accounts = Accounts::new();
        accounts.deposit("Alice", 100).unwrap();
        let tx = accounts.withdraw("Alice", 50).unwrap();

        assert_eq!(
            tx,
            Tx::Withdraw {
                account: "Alice".to_string(),
                amount: 50,
            }
        );
        assert_eq!(accounts.accounts.get("Alice"), Some(&50));
    }
}
