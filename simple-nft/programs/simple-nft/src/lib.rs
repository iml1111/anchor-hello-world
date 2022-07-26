use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token;
use anchor_spl::token::{MintTo, Token};
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v2};

declare_id!("4jvMtVYUSyWoVvaxwAnWzSNdqaZHJArKG3zyFQGZjuty");

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
            authority: _ctx.accounts.payer.to_account_info(),
        };
        msg!("CPI(Token Program) Account Assigned");
        let cpi_program = _ctx.accounts.token_program.to_account_info();
        msg!("Token Program Assigned");
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        msg!("Token Program Context Assigned");
        token::mint_to(cpi_ctx, 1)?;
        msg!("Token Minted !!!");

        // Token Metadata 삽입 과정
        let account_info = vec![
            _ctx.accounts.metadata.to_account_info(),
            _ctx.accounts.mint.to_account_info(),
            _ctx.accounts.mint_authority.to_account_info(),
            _ctx.accounts.payer.to_account_info(),
            _ctx.accounts.token_metadata_program.to_account_info(),
            _ctx.accounts.token_program.to_account_info(),
            _ctx.accounts.system_program.to_account_info(),
            _ctx.accounts.rent.to_account_info(),
        ];
        msg!("Account Info Assgined");
        let creator = vec![
            mpl_token_metadata::state::Creator {
                address: creator_key, // Who is Creator > Example?
                verified: false, // Why False ??
                share: 100,
            },
            mpl_token_metadata::state::Creator {
                // Is diff creator_key ??
                address: _ctx.accounts.mint_authority.key(), 
                verified: false,
                share: 0,
            }
        ];
        msg!("Creator Assigned");
        invoke(
            &create_metadata_accounts_v2(
                _ctx.accounts.token_metadata_program.key(),
                _ctx.accounts.metadata.key(), // what is this ??
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
            _ctx.accounts.master_edition.to_account_info(),
            _ctx.accounts.mint.to_account_info(),
            _ctx.accounts.mint_authority.to_account_info(),
            _ctx.accounts.payer.to_account_info(),
            // 방금 메타데이터 계정을 만들었기 때문에 존재할 것임.
            _ctx.accounts.metadata.to_account_info(),
            _ctx.accounts.token_metadata_program.to_account_info(),
            _ctx.accounts.token_program.to_account_info(),
            _ctx.accounts.system_program.to_account_info(),
            _ctx.accounts.rent.to_account_info(),
        ];
        msg!("Master Edition Account Infos Assigned");
        invoke(
            &create_master_edition_v3(
                _ctx.accounts.token_metadata_program.key(),
                _ctx.accounts.master_edition.key(), // what is this ??
                _ctx.accounts.mint.key(),
                // update_authority
                _ctx.accounts.payer.key(),
                _ctx.accounts.mint_authority.key(),
                _ctx.accounts.metadata.key(),
                _ctx.accounts.payer.key(),
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
    // 민팅을 할 권한을 가진 사람: 이 사람이 반드시 민팅을 하지 않을 수도 있음.
    pub mint_authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,

    // 민트 어카운트(공장)은 미리 생성이 되어 있어야 한다. 
    // 토큰 프로그램으로 생성할 수 있음 -> 미리 여기서 만들어서 와야 함.
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    
    // 메타데이터 어카운트가 미리 생성되어 있어야 한다.
    // 토큰 메타데이터 프로그램에서 생성할 수 있음. -> 미리 여기서 만들어서 와야 함.
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    
    // NFT의 발행자, 돈 내는 사람
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub payer: AccountInfo<'info>,
    
    // 토큰 어카운트
    // (민트 어카운트) <---(토큰 어카운트)---> (유저 어카운트)
    // 토큰 어카운트도 미리 만들고 와야 함.
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    
    // 마스터 에디션 어카운트
    // 메타플렉스에서 토큰을 만드는 방법을 따라야 함. (기존 솔라나에서 만드는 것과 살짝 다름)
    // 얘도 토큰 메타데이터 프로그램에서 미리 생성해야 와야 함.
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_metadata_program: UncheckedAccount<'info>,
    
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub rent: AccountInfo<'info>,
}
