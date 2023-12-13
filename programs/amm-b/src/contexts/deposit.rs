use anchor_lang::prelude::*;

use anchor_spl::{token_interface::{Mint, TokenInterface, TokenAccount}, associated_token::AssociatedToken};
use crate::state::Config;
use crate::errors::AmmError;

#[derive(Accounts)]
pub struct Deposit {}
