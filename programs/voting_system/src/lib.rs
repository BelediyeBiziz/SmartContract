use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey,
};

declare_id!("B7B3mxmT8KtZCsZVtLWAG9132V8wq2XGeyRpcVBj3US1");

#[program]
pub mod voting_system {
    use super::*;

    pub fn create_proposal(ctx: Context<CreateProposal>, proposal_uid: String) -> Result<()> {
        let proposal: &mut Account<Proposal> = &mut ctx.accounts.proposal;
        let user_proposal_vote: &mut Account<MyVote> = &mut ctx.accounts.user_proposal_vote;

        let (pda, _bump) = Pubkey::find_program_address(
            &[proposal_uid.as_bytes(), ctx.accounts.user.key().as_ref()],
            ctx.program_id
        );

        if user_proposal_vote.key() != pda {
            return Err(ProgramError::InvalidAccountData.into());
        }
        


        proposal.proposal_uid = proposal_uid;
        proposal.yes_votes = 0;
        proposal.no_votes = 0;

        user_proposal_vote.vote = VoteOption::Any;

        msg!("--> PDA: {}", pda);
        msg!("--> Client PDA: {}", user_proposal_vote.key());

        Ok(())
    }


    pub fn vote_proposal(ctx: Context<VotetoProposal>, proposal_uid: String, vote: VoteOption) -> Result<()> {
        let proposal: &mut Account<Proposal> = &mut ctx.accounts.proposal;
        let user_proposal_vote: &mut Account<MyVote> = &mut ctx.accounts.user_proposal_vote;

        let (pda, _bump) = Pubkey::find_program_address(
            &[proposal_uid.as_bytes(), ctx.accounts.user.key().as_ref()],
            ctx.program_id
        );

        if user_proposal_vote.key() != pda {
            return Err(ProgramError::InvalidAccountData.into());
        }
        

        msg!("--> User Vote PDA: {}", pda);
        msg!("--> User Prev. Vote: {:?}", user_proposal_vote.vote);

        // require!(user_proposal_vote.vote == VoteOption::Any, ErrorCode::InvalidVoteOption);


        msg!("--> Voting to proposal: {}", &proposal_uid);
        msg!("--> Vote in vote proposal: {:?}", proposal.yes_votes);
        msg!("--> vote: {:?}", vote);
        proposal.proposal_uid = proposal_uid;
        match vote {
            VoteOption::Yes => {
                proposal.yes_votes += 1;
                user_proposal_vote.vote = VoteOption::Yes;
            }
            VoteOption::No => {
                proposal.no_votes += 1;
                user_proposal_vote.vote = VoteOption::No;
            }
            _=>{}
        }
        Ok(())
    }

}

#[derive(Accounts)]
#[instruction(proposal_uid: String)]
pub struct CreateProposal<'info> {
    #[account(init, 
        seeds = [proposal_uid.as_bytes()],
        bump,
        payer=user, space = 8 + 36 + 8 +8)]
    pub proposal: Account<'info, Proposal>,
    #[account(init, 
        seeds = [proposal_uid.as_bytes(), user.key.as_ref()],
        bump,
        payer=user, space = 8 + 20)]
    pub user_proposal_vote: Account<'info, MyVote>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(proposal_uid: String)]
pub struct VotetoProposal<'info> {
    #[account(
        mut, 
        seeds = [proposal_uid.as_bytes()],
        bump
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(
        mut, 
        seeds = [proposal_uid.as_bytes(), user.key.as_ref()],
        bump
    )]
    pub user_proposal_vote: Account<'info, MyVote>,

    #[account(mut)]
    pub user: Signer<'info>,

}



#[account]
pub struct Proposal {
    pub proposal_uid: String,
    pub yes_votes: u64,
    pub no_votes: u64,
}

#[account]
pub struct MyVote {
    pub vote: VoteOption,
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug,PartialEq,Eq)]
pub enum VoteOption {
    Yes,
    No,
    Any
}

#[error_code]
pub enum ErrorCode {
    #[msg("The provided vote option is invalid.")]
    InvalidVoteOption,
}

