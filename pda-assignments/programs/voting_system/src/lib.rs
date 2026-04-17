use anchor_lang::prelude::*;

declare_id!("9YsTm26FR9VE5QdbWvCc7vBAn3qyz6MzTy1VCstvjCRv");

#[program]
pub mod voting_system {
    use super::*;

    pub fn create_poll(ctx: Context<CreatePoll>, poll_id: u64, question: String) -> Result<()> {
        let poll = &mut ctx.accounts.poll;
        poll.authority = ctx.accounts.authority.key();
        poll.poll_id = poll_id;
        poll.question = question;
        poll.total_votes = 0;
        poll.bump = ctx.bumps.poll;
        Ok(())
    }

    pub fn cast_vote(ctx: Context<CastVote>, choice: u8) -> Result<()> {
        let vote_record = &mut ctx.accounts.vote_record;
        let poll = &mut ctx.accounts.poll;

        vote_record.poll = poll.key();
        vote_record.voter = ctx.accounts.voter.key();
        vote_record.choice = choice;
        vote_record.bump = ctx.bumps.vote_record;

        poll.total_votes += 1;
        Ok(())
    }

    pub fn close_poll(_ctx: Context<ClosePoll>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct CreatePoll<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 1 + 8 + (4 + 100) + 8,
        seeds = [b"poll", poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub poll: Account<'info, Poll>,
    
    #[account(
        init,
        payer = voter,
        space = 8 + 32 + 32 + 1 + 1,
        seeds = [b"vote", poll.key().as_ref(), voter.key().as_ref()],
        bump
    )]
    pub vote_record: Account<'info, VoteRecord>,
    
    #[account(mut)]
    pub voter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClosePoll<'info> {
    #[account(
        mut,
        has_one = authority,
        close = authority,
        seeds = [b"poll", poll.poll_id.to_le_bytes().as_ref()],
        bump = poll.bump
    )]
    pub poll: Account<'info, Poll>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[account]
pub struct Poll {
    pub authority: Pubkey,
    pub bump: u8,
    pub poll_id: u64,
    pub question: String,
    pub total_votes: u64,
}

#[account]
pub struct VoteRecord {
    pub poll: Pubkey,
    pub voter: Pubkey,
    pub bump: u8,
    pub choice: u8,
}