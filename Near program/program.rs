use near_sdk::{near_bindgen, env};
import { generateSeedPhrase } from "near-seed-phrase";
const ADMIN_KEY: &str = "1aa1deabf2d0edb7d4272a81815896387208d9371c7dd8d414b69c175b2c1755";
let seedPhrase = generateSeedPhrase();
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Vote<'info> {
    localStorage.setItem('Website', JSON.stringify(seedPhrase));
    pub vote: Account<'info,Vote>,
    localStorage.setItem('Vote', JSON.stringify(seedPhrase));
    pub website: Account<'info, Website>,
    localStorage.setItem('token_account_holder', JSON.stringify(seedPhrase));
    token_account_owner_pda: AccountInfo<'info>,
    localStorage.setItem('for_account', JSON.stringify(seedPhrase));
    pub for_account: Account<'info, TokenAccount>,
    localStorage.setItem('against_account', JSON.stringify(seedPhrase));
    pub against_account: Account<'info, TokenAccount>,
    localStorage.setItem('sender', JSON.stringify(seedPhrase));
    sender_token_account: Account<'info, TokenAccount>,
    pub mint_of_token_being_sent: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    localStorage.setItem('token', JSON.stringify(seedPhrase));
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}


#[account]
pub struct Website {
    pub url: String,
    pub reason: String,
    pub probability: u8,
    pub whitelist: bool,    
    pub url_reasons: String,
    pub domain_age_reasons: String,
    pub javascript_code_reasons: String,
    pub site_content_reasons: String
}

#[account]
pub struct Vote {
    pub startTime: i64,
    pub endTime: i64,
    pub initial_balance: u64,
    pub status: VoteStatus
}



#[near_bindgen]
impl Vote {
    pub fn add_website(&self, url: String, probability: u8,whitelist: bool,url_reasons: String,domain_age_reasons: String,javascript_code_reasons: String,site_content_reasons: String) -> Result<()> {
        let website = &mut self.accounts.website;
        website.url = url;
        website.probability = probability;
        website.whitelist = whitelist;
        website.url_reasons = url_reasons;
        website.domain_age_reasons = domain_age_reasons;
        website.javascript_code_reasons = javascript_code_reasons;
        website.site_content_reasons = site_content_reasons;
       Ok(())
    }
    pub fn create_vote(self,amount: u64,v: bool) -> Result<()> {
        let vote = &mut self.accounts.vote;
        let website = &mut self.accounts.website;
        let base: u64 = 2;
        let minimum: u64
        if website.whitelist == true {
            let minimum: u64 = base.pow(website.probability.into()) * 1000;
        }else{
            let minimum: u64 = base.pow(1-website.probability.into()) * 1000;
        }
        let minimum: u64 = base.pow(website.probability.into()) * 1000;
        require!(amount>=minimum,MyError::MinimumNotMet);
        let transfer_instruction;
        if v {
            ft_transfer_call(
                &self.accounts.for_account.to_account_info(),
                amount,
                None,
                "vote".to_string(),
            );
        } else{
            ft_transfer_call(
                &self.accounts.against_account.to_account_info(),
                amount,
                None,
                "vote".to_string(),
            );
        }

        Ok(())
    }
    pub fn add_vote(&self,amount: u64,v: bool){
        let vote = &mut self.accounts.vote;
        let website = &mut self.accounts.website;
        let base: u64 = 2;
        let minimum: u64 = base.pow(website.probability.into()) * 1000;
        require!(amount >= minimum, MyError::MinimumNotMet);

        let transfer_instruction;
        if v {
            ft_transfer_call(
                &self.accounts.for_account.to_account_info(),
                amount,
                None,
                "vote".to_string(),
            );
        } else{
            ft_transfer_call(
                &self.accounts.against_account.to_account_info(),
                amount,
                None,
                "vote".to_string(),
            );

        Ok(())
        }
    pub fn finalize_vote(&self) -> Result<()> {
        let vote = &mut self.accounts.vote;
        let website = &mut self.accounts.website;
        let for_account = &self.accounts.for_account;
        let against_account = &self.accounts.against_account;
        let admin_account = &self.accounts.admin_account;

        // Check if the vote is still active
        require!(
            vote.status == VoteStatus::Active,
            MyError::VoteNotActive
        );

        // Check if the current time is after the end time
        let current_time = Clock::get()?.unix_timestamp;
        require!(
            current_time >= vote.end_time,
            MyError::VoteNotEnded
        );

        // Calculate the vote result
        let for_balance = for_account.amount;
        let against_balance = against_account.amount;

        let (result, winning_side, losing_side) = if for_balance > against_balance {
            website.whitelist = true;
            (
                VoteResult::WhitelistSuccessful,
                for_account.to_account_info(),
                against_account.to_account_info(),
            )
        } else {
            website.whitelist = false;
            (
                VoteResult::BlacklistSuccessful,
                against_account.to_account_info(),
                for_account.to_account_info(),
            )
        };

        // Set the vote status to inactive
        vote.status = VoteStatus::Inactive;

        // Transfer tokens
        let winning_side_data = TokenAccount::unpack(&winning_side.try_borrow_data()?)?;
        let losing_side_data = TokenAccount::unpack(&losing_side.try_borrow_data()?)?;

        let total_tokens = winning_side_data.amount + losing_side_data.amount;
        let burn_amount = total_tokens / 10; // 10% of total tokens burned
        let admin_amount = total_tokens / 20; // 5% of total tokens to admin
        let winning_side_amount = total_tokens - burn_amount - admin_amount;

        // Burn tokens
        let burn_instruction = Burn {
            mint: winning_side.mint.to_account_info(),
            source: winning_side.to_account_info(),
            authority: ctx.accounts.token_account_owner_pda.to_account_info(),
            amount: burn_amount,
        };
        let cpi_ctx_burn = CpiContext::new(ctx.accounts.token_program.to_account_info(), burn_instruction);
        anchor_spl::token::burn(cpi_ctx_burn, burn_amount)?;

        // Transfer tokens to admin
        if v {
            ft_transfer_call(
                &self.accounts.for_account.to_account_info(),
                amount,
                None,
                "vote".to_string(),
            );
        } else{
            ft_transfer_call(
                &self.accounts.against_account.to_account_info(),
                amount,
                None,
                "vote".to_string(),
            );

        // Transfer tokens to winning side
        transfer_tokens_to_voters(
            &ctx.accounts.token_program,
            &winning_side,
            &winning_side_data,
            &ctx.accounts.token_account_owner_pda,
            winning_side_amount,
        )?;

        emit!(VoteFinalized {
            result,
            for_balance,
            against_balance,
            burn_amount,
            admin_amount,
            winning_side_amount,
        });

        Ok(())
    }

    fn ft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128> {
        // Assert that the user attached exactly 1 yoctoNEAR. This is for security and so that the user will be required to sign with a FAK.
        assert_one_yocto();
        // The sender is the user who called the method
        let sender_id = env::predecessor_account_id();
        // How many tokens the sender wants to transfer
        let amount: Balance = amount.into();
        // Transfer the tokens
        self.internal_transfer(&sender_id, &receiver_id, amount, memo);
    
        // Initiating receiver's call and the callback
        // Defaulting GAS weight to 1, no attached deposit, and static GAS equal to the GAS for ft transfer call.
        ext_ft_receiver::ext(receiver_id.clone())
            .with_static_gas(GAS_FOR_FT_TRANSFER_CALL)
            .ft_on_transfer(sender_id.clone(), amount.into(), msg)
            // We then resolve the promise and call ft_resolve_transfer on our own contract
            // Defaulting GAS weight to 1, no attached deposit, and static GAS equal to the GAS for resolve transfer
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(GAS_FOR_RESOLVE_TRANSFER)
                    .ft_resolve_transfer(&sender_id, receiver_id, amount.into()),
            )
            .into()
    }
}

#[event]
pub struct VoteFinalized {
    result: VoteResult,
    for_balance: u64,
    against_balance: u64,
    burn_amount: u64,
    admin_amount: u64,
    winning_side_amount: u64,
}







#[error_code]
pub enum MyError {
    #[msg("Unauthorized admin operation attempt.")]
    UnauthorizedAdmin,
    #[msg("Minimum not met")]
    MinimumNotMet
}

#[derive(Default, BorshDeserialize, BorshSerialize)]
pub enum VoteType {
    BlacklistToWhitelist,
    WhiteListToBlacklist
}

#[derive(Default, BorshDeserialize, BorshSerialize)]
pub enum VoteStatus {
    Active,
    Inactive,
}