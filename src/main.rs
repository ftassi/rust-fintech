use std::io::Write;
use accounts::Accounts;

use crate::errors::{AccountingError, ParsingError};

mod errors{
    use std::num::ParseIntError;


    #[derive(Debug)]
    pub enum ParsingError {
        InvalidAmount(ParseIntError),
    }

    #[derive(Debug)]
    pub enum AccountingError {
        AccountNotFound(String),
        AccountUnderFunded(String, u64),
        AccountOverFunded(String, u64),
    }
}

mod tx {
    #[derive(Debug)]
    pub enum Tx {
        Deposit { account: String, amount: u64 },
        Withdraw { account: String, amount: u64 },
    }
}

mod accounts {
    use std::collections::HashMap;

    use crate::{tx::Tx, errors::AccountingError};


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

fn read_amount() -> Result<u64, ParsingError> {
    read_from_stdin("Amount:\n")
        .parse::<u64>()
        .map_err(ParsingError::InvalidAmount)
}

fn parse_command(command: String) -> Result<Command, ParsingError> {
    match command.as_str() {
        "deposit" => {
            let account = read_from_stdin("Account:\n");
            let amount = read_amount();
            amount.map(|amount| Command::Deposit { account, amount })
        }
        "withdraw" => {
            let account = read_from_stdin("Account:\n");
            let amount = read_amount();
            amount.map(|amount| Command::Withdraw { account, amount })
        }
        "send" => {
            let sender = read_from_stdin("Sender:\n");
            let recipient = read_from_stdin("Recipient:\n");
            let amount = read_amount();
            amount.map(|amount| Command::Send {
                sender,
                recipient,
                amount,
            })
        }
        "print" => Ok(Command::Print),
        "quit" => Ok(Command::Quit),
        _ => Ok(Command::Unknown),
    }
}

fn command_result<T>(result: Result<T, AccountingError>) {
    match result {
        Ok(_) => println!("Ok"),
        Err(e) => println!("Command failed {:?}", e),
    }
}

fn main() {
    let mut accounts = Accounts::new();
    loop {
        match parse_command(read_from_stdin("Command:\n")) {
            Ok(Command::Deposit { account, amount }) => {
                command_result(accounts.deposit(account.as_str(), amount));
            }
            Ok(Command::Withdraw { account, amount }) => {
                command_result(accounts.withdraw(account.as_str(), amount));
            }
            Ok(Command::Send {
                sender,
                recipient,
                amount,
            }) => {
                command_result(accounts.send(sender.as_str(), recipient.as_str(), amount));
            }
            Ok(Command::Unknown) => {
                println!("Unknow command");
            }
            Ok(Command::Quit) => {
                println!("Ok, bye!");
                break;
            }
            Ok(Command::Print) => {
                println!("{:?}", accounts);
            }
            Err(e) => {
                println!("Error: {:?}", e)
            }
        };
    }
}
