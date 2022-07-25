use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token;
use anchor_spl::token::{MintTo, Token};
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v2};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod simple_nft {
    use super::*;

    pub fn mint_nft(
        _ctx: Context<MintNFT>, 
        creator_key: Pubkey,
        uri: String, 
        title: String,
        symbol: String,
    ) -> Result<()> {
        
        // SPL Token Mint 과정
        msg!("Initializing Mint NFT");
        let cpi_accounts = MintTo {
            mint: _ctx.accounts.mint.to_account_info(),
            to: _ctx.accounts.token_account.to_account_info(),
            authority: _ctx.payer.to_account_info(),
        };
        msg!("CPI(Token Program) Account Assigned")
        let cpi_program = _ctx.accounts.token_program.to_account_info();
        msg!("Token Program Assigned");
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        msg!("Token Program Context Assigned");
        token::mint_to(cpi_ctx, 1)?;
        msg!("Token Minted !!!");

        // Token Metadata 삽입 과정
        msg!("Account Info Assgined")
        let creator = vec![
            mpl_token_metadata::state::Creator {
                address: creator_key, // Who is Creator ??
                verified: false, // Why False ??
                share: 100,
            },
            mpl_token_metadata::state::Creator {
                address: _ctx.accounts.mint_authority.key(),
                verified: false,
                share: 0,
            }
        ]
        msg!("Creator Assigned");
        invoke(
            &create_metadata_accounts_v2(
                _ctx.accounts.token_metadata_program.key(),
                _ctx.accounts.metadata.key(),
                _ctx.accounts.mint.key(),
                _ctx.accounts.mint_authority.key(),
                _ctx.accounts.payer.key(),
                // upgrade_authority
                _ctx.accounts.payer.key(), 
                title,
                symbol,
                uri,
                Some(creator),
                1,  // seller_fee_basis_points
                true, // update_authority_is_signer
                false, // is_mutable
                None, // collection
                None, // uses
            ),
            account_info.as_slice(),
        )?;
        msg!("Metadata Account Created !!!");

        // Master Edition 삽입 과정
        let master_edition_infos = vec![
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            // 방금 메타데이터 계정을 만들었기 때문에 존재할 것임.
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];
        msg!("Master Edition Account Infos Assigned");
        invoke(
            &create_master_edition_v3(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.master_edition.key(),
                ctx.accounts.mint.key(),
                // update_authority
                ctx.accounts.payer.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.payer.key(),
                Some(0), // max_supply
            ),
            master_edition_infos.as_slice(),
        )?;
        msg!("Master Edition Nft Minted !!!");

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub payer: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_metadata_program: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub rent: AccountInfo<'info>,
}
