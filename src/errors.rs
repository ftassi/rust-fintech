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
