use std::{
    collections::HashMap,
    error::Error,
    io::{stdin, stdout, Read, Write}, num::ParseIntError,
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

#[derive(Debug)]
enum ParsingError {
    InvalidAmount(ParseIntError)
}

fn read_amount() -> Result<u64, ParsingError> {
    read_from_stdin("Amount:\n").parse::<u64>().map_err(ParsingError::InvalidAmount)
}

fn parse_command(command: String) -> Result<Command, ParsingError> {
    match command.as_str() {
        "deposit" => {
            let account = read_from_stdin("Account:\n");
            let amount = read_amount(); 
            amount.map(|amount| Command::Deposit { account, amount})
        },
        "withdraw" => {
            let account = read_from_stdin("Account:\n");
            let amount = read_amount(); 
            amount.map(|amount| Command::Withdraw { account, amount})
        },
        "send" => {
            let sender = read_from_stdin("Sender:\n");
            let recipient = read_from_stdin("Recipient:\n");
            let amount = read_amount(); 
            amount.map(|amount| Command::Send { sender, recipient, amount})
        },
        "print" => Ok(Command::Print),
        "quit" => Ok(Command::Quit),
        _ => Ok(Command::Unknown),
    }
}

fn main() {
    let mut accounts = Accounts::new();
    loop {
        let command = parse_command(read_from_stdin("Command:\n"));
        if let Ok(Command::Deposit { account, amount }) = command {
            accounts.deposit(account.as_str(), amount).unwrap();
        } else if let Ok(Command::Withdraw { account, amount }) = command {
            accounts.withdraw(account.as_str(), amount).unwrap();
        } else if let Ok(Command::Send {
            sender,
            recipient,
            amount,
        }) = command
        {
            accounts
                .send(sender.as_str(), recipient.as_str(), amount)
                .unwrap();
        } else if let Ok(Command::Print) = command {
            println!("{:?}", accounts);
        } else if let Ok(Command::Unknown) = command {
            println!(
                "Sorry, try again, available commands are deposit, withdraw, send, print, quit"
            );
        } else if let Ok(Command::Quit) = command {
            println!("Ok, bye!");
            break;
        } else if let Err(e) = command {
            println!("Error {:?}", e);
        }
    }
}
