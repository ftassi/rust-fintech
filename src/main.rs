mod errors;
mod tx;
mod accounts;

use std::io::Write;
use accounts::Accounts;
use crate::errors::{AccountingError, ParsingError};

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
