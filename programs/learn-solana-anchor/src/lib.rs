use anchor_lang::prelude::*;

declare_id!("23VKzPHLb4AisXqdRocpcwAGEGq1o9qS3um12dgKWRiw");

#[program]
pub mod learn_solana_anchor {
    use super::*;

    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result<()> {
        //* Get reference to the account
        let base_account = &mut ctx.accounts.base_account;
        //* Initialize total gifs
        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        //*build the struct
        let item = ItemStuct {
            gif_link,
            user_addres: *user.to_account_info().key,
        };

        //* update the account
        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }
}

//* Attach certain variables to StartStuffOff context
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space= 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

//* Specify what data you want in AddGif Context
//* Update totol_gifs in base_account
//* Add the signer who call the AddGif method to the stuct so that we can save it
#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

//* Add custom struct for us to work with
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStuct {
    pub gif_link: String,
    pub user_addres: Pubkey,
}

//* Tell Solana what we want to store in this account
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStuct>,
}

// use anchor_lang::prelude::*;

// declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// #[program]
// pub mod learn_solana_anchor {
//     use super::*;

//     pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
//         //get reference to the account
//         let base_account = &mut ctx.accounts.base_account;
//         //Initalize Total Gifs
//         base_account.total_gifs = 0;
//         Ok(())
//     }

//     pub fn add_gif(ctx: Context<AddGif>) -> Result<()> {
//         // Get a reference to the account and increment total_gifs.
//         let base_account = &mut ctx.accounts.base_account;
//         base_account.total_gifs += 1;
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct Initialize<'info> {
//     #[account(init, payer = user, space = 9000)]
//     pub base_account: Account<'info, BaseAccount>,
//     #[account(mut)]
//     //* proving that user who is running actually own the wallet
//     pub user: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }

// #[derive(Accounts)]
// pub struct AddGif<'info> {
//     #[account(mut)]
//     pub base_account: Account<'info, BaseAccount>,
// }

// #[account]
// pub struct BaseAccount {
//     pub total_gifs: u64,
// }
