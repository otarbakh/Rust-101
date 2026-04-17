use anchor_lang::prelude::*;

declare_id!("Bt4ew32QCK2huiBvetibvc7Poc65vwyT82qMg5peBejJ");

#[program]
pub mod note_taking {
    use super::*;

    pub fn create_note(ctx: Context<CreateNote>, title: String, content: String) -> Result<()> {
        let note = &mut ctx.accounts.note;
        note.authority = ctx.accounts.authority.key();
        note.title = title;
        note.content = content;
        note.bump = ctx.bumps.note;
        Ok(())
    }

    pub fn update_note(ctx: Context<UpdateNote>, _title: String, new_content: String) -> Result<()> {
        let note = &mut ctx.accounts.note;
        note.content = new_content;
        Ok(())
    }

    pub fn delete_note(_ctx: Context<DeleteNote>, _title: String) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreateNote<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 1 + (4 + title.len()) + (4 + 500),
        seeds = [b"note", authority.key().as_ref(), title.as_bytes()],
        bump
    )]
    pub note: Account<'info, Note>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct UpdateNote<'info> {
    #[account(
        mut,
        has_one = authority,
        seeds = [b"note", authority.key().as_ref(), title.as_bytes()],
        bump = note.bump,
    )]
    pub note: Account<'info, Note>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteNote<'info> {
    #[account(
        mut,
        has_one = authority,
        close = authority,
        seeds = [b"note", authority.key().as_ref(), title.as_bytes()],
        bump = note.bump,
    )]
    pub note: Account<'info, Note>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[account]
pub struct Note {
    pub authority: Pubkey,
    pub bump: u8,
    pub title: String,
    pub content: String,
}