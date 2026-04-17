use anchor_lang::prelude::*;

declare_id!("9CKTjAC628x5fobfJYMMJeXQeZXohcSiXUvZaMwRuQ8w");

#[program]
pub mod profile_system {
    use super::*;

    pub fn initialize_profile(ctx: Context<InitializeProfile>, username: String, bio: String) -> Result<()> {
        let profile = &mut ctx.accounts.profile;
        profile.authority = ctx.accounts.authority.key();
        profile.username = username;
        profile.bio = bio;
        profile.bump = ctx.bumps.profile;
        Ok(())
    }

    pub fn update_profile(ctx: Context<UpdateProfile>, username: String, bio: String) -> Result<()> {
        let profile = &mut ctx.accounts.profile;
        profile.username = username;
        profile.bio = bio;
        Ok(())
    }

    pub fn close_profile(_ctx: Context<CloseProfile>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeProfile<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 1 + (4 + 32) + (4 + 200), 
        seeds = [b"profile", authority.key().as_ref()],
        bump
    )]
    pub profile: Account<'info, Profile>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateProfile<'info> {
    #[account(
        mut,
        has_one = authority,
        seeds = [b"profile", authority.key().as_ref()],
        bump = profile.bump,
    )]
    pub profile: Account<'info, Profile>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct CloseProfile<'info> {
    #[account(
        mut,
        has_one = authority,
        close = authority, 
        seeds = [b"profile", authority.key().as_ref()],
        bump = profile.bump,
    )]
    pub profile: Account<'info, Profile>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[account]
pub struct Profile {
    pub authority: Pubkey,
    pub bump: u8,
    pub username: String,
    pub bio: String,
}