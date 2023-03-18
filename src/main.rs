use std::collections::HashMap;

#[derive(Debug)]
struct Accounts {
    accounts: HashMap<String, u64>,
}

impl Accounts {
    pub fn new() -> Self {
        Accounts {
            accounts: Default::default(),
        }
    }
    fn deposit(&mut self, signer: &str, amount: u64) -> Result<Tx, AccountingError> {
        if let Some(account) = self.accounts.get_mut(signer) {
            (*account)
                .checked_add(amount)
                .and_then(|r| {
                    *account = r;
                    Some(r)
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

    fn withdraw(&mut self, signer: &str, amount: u64) -> Result<Tx, AccountingError> {
        if let Some(account) = self.accounts.get_mut(signer) {
            (*account)
                .checked_sub(amount)
                .and_then(|r| {
                    *account = r;
                    Some(r)
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

    fn send(
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

#[derive(Debug)]
enum AccountingError {
    AccountNotFound(String),
    AccountUnderFunded(String, u64),
    AccountOverFunded(String, u64),
}

#[derive(Debug)]
enum Tx {
    Deposit { account: String, amount: u64 },
    Withdraw { account: String, amount: u64 },
}

fn main() {
    println!("Hello, accounting world!");

    // We are using simple &str instances as keys
    // for more sophisticated keys (e.g. hashes)
    // the data type could remain the same
    let bob = "bob";
    let alice = "alice";
    let charlie = "charlie";
    let initial_amount = 100;

    // Creates the basic ledger and a tx log container
    let mut ledger = Accounts::new();
    let mut tx_log = vec![];

    // Deposit an amount to each account
    for signer in &[bob, alice, charlie] {
        let status = ledger.deposit(*signer, initial_amount);
        println!("Depositing {} for {}: {:?}", signer, initial_amount, status);
        // Add the resulting transaction to a list of transactions
        // .unwrap() will crash the program if the status is an error.
        tx_log.push(status.unwrap());
    }

    // Send currency from one account (bob) to the other (alice)
    let send_amount = 10_u64;
    let status = ledger.send(bob, alice, send_amount);
    println!(
        "Sent {} from {} to {}: {:?}",
        send_amount, bob, alice, status
    );

    // Add both transactions to the transaction log
    let (tx1, tx2) = status.unwrap();
    tx_log.push(tx1);
    tx_log.push(tx2);

    // Withdraw everything from the accounts
    let tx = ledger.withdraw(charlie, initial_amount).unwrap();
    tx_log.push(tx);
    let tx = ledger
        .withdraw(alice, initial_amount + send_amount)
        .unwrap();
    tx_log.push(tx);

    // Here we are withdrawing too much and there won't be a transaction
    println!(
        "Withdrawing {} from {}: {:?}",
        initial_amount,
        bob,
        ledger.withdraw(bob, initial_amount)
    );
    // Withdrawing the expected amount results in a transaction
    let tx = ledger.withdraw(bob, initial_amount - send_amount).unwrap();
    tx_log.push(tx);

    // {:?} prints the Debug implementation, {:#?} pretty-prints it
    println!("Ledger empty: {:?}", ledger);
    println!("The TX log: {:#?}", tx_log);
}
