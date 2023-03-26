use std::{
    collections::HashMap,
    error::Error,
    io::{stdin, stdout, Read, Write},
};

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

#[derive(Debug)]
enum Command {
    Deposit {
        account: String,
        amount: u64,
    },
    Withdraw {
        account: String,
        amount: u64,
    },
    Send {
        sender: String,
        recipient: String,
        amount: u64,
    },
    Print,
    Quit,
    Unknown,
}

fn read_from_stdin(label: &str) -> String {
    std::io::stdout().write_all(label.as_bytes()).unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn parse_command(command: String) -> Command {
    match command.as_str() {
        "deposit" => Command::Deposit {
            account: read_from_stdin("Account:\n"),
            amount: read_from_stdin("Amount:\n").parse::<u64>().unwrap(),
        },
        "withdraw" => Command::Withdraw {
            account: read_from_stdin("Account:\n"),
            amount: read_from_stdin("Amount:\n").parse::<u64>().unwrap(),
        },
        "send" => Command::Send {
            sender: read_from_stdin("Sender:\n"),
            recipient: read_from_stdin("Recipient:\n"),
            amount: read_from_stdin("Amount:\n").parse::<u64>().unwrap(),
        },
        "print" => Command::Print,
        "quit" => Command::Quit,
        _ => Command::Unknown,
    }
}

fn main() {
    let mut accounts = Accounts::new();
    loop {
        let command = parse_command(read_from_stdin("Command:\n"));
        if let Command::Deposit { account, amount } = command {
            accounts.deposit(account.as_str(), amount).unwrap();
        } else if let Command::Withdraw { account, amount } = command {
            accounts.withdraw(account.as_str(), amount).unwrap();
        } else if let Command::Send {
            sender,
            recipient,
            amount,
        } = command
        {
            accounts
                .send(sender.as_str(), recipient.as_str(), amount)
                .unwrap();
        } else if let Command::Print = command {
            println!("{:?}", accounts);
        } else if let Command::Unknown = command {
            println!(
                "Sorry, try again, available commands are deposit, withdraw, send, print, quit"
            );
        } else if let Command::Quit = command {
            println!("Ok, bye!");
            break;
        }
    }
}
