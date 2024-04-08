use anchor_lang::prelude::*;

declare_id!("66k9e3kdzatxQqV3rbp7gYjtH1BxCHKXxwdT8gqJHDKp");

#[program]
mod sleepin {
    use super::*;

    pub fn create_user_data_acc(ctx: Context<CreateUserDataAcc>) -> Result<()> {
        let data_acc = &mut ctx.accounts.data_acc;
        data_acc.data_owner = *ctx.accounts.authority.key;
        Ok(())
    }

    pub fn add_data(
        ctx: Context<AddToDataAcc>,
        date: String,
        encrypted_data: String,
    ) -> Result<()> {
        let data_acc = &mut ctx.accounts.data_acc;
        data_acc.data_date = date;
        data_acc.user_encrypted_data = encrypted_data;
        Ok(())
    }

    pub fn get_data(ctx: Context<GetData>) -> Result<()> {
        let data_acc = &ctx.accounts.data_acc;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddToDataAcc<'info> {
    authority: Signer<'info>,

    #[account(mut)]
    data_acc: Account<'info, UserDataAcc>,
}

#[derive(Accounts)]
pub struct GetData<'info> {
    #[account()]
    data_acc: Account<'info, UserDataAcc>,
}

#[derive(Accounts)]
pub struct CreateUserDataAcc<'info> {
    #[account(init, payer = authority, space = 8 + 40 + 10 * 32 + 10 + 1000)]
    // space miktarını düzenle //
    data_acc: Account<'info, UserDataAcc>,
    #[account(mut)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

#[account]
pub struct UserDataAcc {
    data_owner: Pubkey,
    data_date: String, // Date, Encrypted Data with respect
    user_encrypted_data: String,
}

