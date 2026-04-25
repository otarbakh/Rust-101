use anchor_lang::prelude::*;

declare_id!("Qrr27z85DyPvTgaoPt4Q6qXN9ZmKe4xirXsKp5b6XCc");

#[program]
pub mod movie_review_program {
    use super::*;

    pub fn add_movie_review(
        ctx: Context<AddMovieReview>,
        title: String,
        description: String,
        rating: u8,
    ) -> Result<()> {
        
        require!(rating >= 1 && rating <= 10, MovieReviewError::InvalidRating);

        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.reviewer = ctx.accounts.initializer.key();
        movie_review.title = title;
        movie_review.rating = rating;
        movie_review.description = description;
        movie_review.bump = ctx.bumps.movie_review;

        msg!("Movie Review added for: {}", movie_review.title);
        Ok(())
    }


    pub fn update_movie_review(
        ctx: Context<UpdateMovieReview>,
        _title: String, 
        new_description: String,
        new_rating: u8,
    ) -> Result<()> {
        require!(new_rating >= 1 && new_rating <= 10, MovieReviewError::InvalidRating);

        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.description = new_description;
        movie_review.rating = new_rating;

        msg!("Movie Review updated!");
        Ok(())
    }


    pub fn delete_movie_review(_ctx: Context<DeleteMovieReview>, _title: String) -> Result<()> {
        msg!("Movie Review deleted and lamports returned.");
        Ok(())
    }
}



#[derive(Accounts)]
#[instruction(title: String)]
pub struct AddMovieReview<'info> {
    #[account(
        init,
        seeds = [b"movie-review", initializer.key().as_ref(), title.as_bytes()],
        bump,
        payer = initializer,
        space = 8 + 32 + 4 + title.len() + 1 + 4 + 200 + 1 
    )]
    pub movie_review: Account<'info, MovieReview>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct UpdateMovieReview<'info> {
    #[account(
        mut,
        seeds = [b"movie-review", initializer.key().as_ref(), title.as_bytes()],
        bump = movie_review.bump,
        has_one = reviewer, 
    )]
    pub movie_review: Account<'info, MovieReview>,
    pub reviewer: Signer<'info>, // ეს ველი საჭიროა has_one-ისთვის
    #[account(mut)]
    pub initializer: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteMovieReview<'info> {
    #[account(
        mut,
        seeds = [b"movie-review", initializer.key().as_ref(), title.as_bytes()],
        bump = movie_review.bump,
        has_one = reviewer,
        close = initializer 
    )]
    pub movie_review: Account<'info, MovieReview>,
    pub reviewer: Signer<'info>,
    #[account(mut)]
    pub initializer: Signer<'info>,
}



#[account]
pub struct MovieReview {
    pub reviewer: Pubkey,    
    pub title: String,       
    pub rating: u8,          
    pub description: String, 
    pub bump: u8,            
}

#[error_code]
pub enum MovieReviewError {
    #[msg("რეიტინგი უნდა იყოს 1-დან 10-მდე.")]
    InvalidRating,
}