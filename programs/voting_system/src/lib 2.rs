use anchor_lang::prelude::*;

declare_id!("D94uN6r38GjYcdcETAbtwnPemSTMbPX8ahgE27sL3526");

#[program]
pub mod voting_system {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
