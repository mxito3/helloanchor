use anchor_lang::prelude::*;

declare_id!("HNu3kRfbe5ALaJad2ffPumu4RTE8U8iu2rvwLh8LjLvs");

#[program]
pub mod helloanchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
