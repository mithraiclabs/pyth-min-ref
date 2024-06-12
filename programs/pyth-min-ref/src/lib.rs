use anchor_lang::prelude::*;

declare_id!("2KKV8Xtcom9DcP273i7r47WpeHsmKTXwxmCfqZJwH9fF");

pub mod errors;
pub mod instructions;

use crate::instructions::*;

#[program]
pub mod pyth_min_ref {
    use super::*;

    pub fn echo_price(ctx: Context<EchoPrice>) -> Result<()> {
        instructions::echo_price::echo_price(ctx)
    }
}
