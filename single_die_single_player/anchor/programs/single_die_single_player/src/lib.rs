#![allow(clippy::result_large_err)]
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
// use orao_solana_vrf::state::VrfAccountData;
// use orao_solana_vrf::*;
use std::collections::BTreeMap;

declare_id!("DLw5fJNrrBCWoy75aukoDApBZm4MEvaWCvPJoqtLSg1p");

const BET_AMOUNT: u64 = 10_000_000; // 0.01 SOL in lamports

#[program]
pub mod single_die_game {
    use super::*;
    // use orao_solana_vrf::*;
    // use orao_solana_vrf::{VrfRequestParams, get_randomness};

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let game_account = &mut ctx.accounts.game_account;
        game_account.rolls = BTreeMap::new();
        game_account.authority = ctx.accounts.authority.key();
        Ok(())
    }

    pub fn choose_number(ctx: Context<ChooseNumber>, guess: u8) -> Result<()> {
        // Validate the guess
        require!(guess >= 1 && guess <= 6, ErrorCode::InvalidNumber);

        // Validate funds
        let player = &ctx.accounts.player;
        require!(
            ctx.accounts.player_token_account.amount >= BET_AMOUNT,
            ErrorCode::InsufficientFunds
        );

        // Transfer tokens from player to game vault
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.player_token_account.to_account_info(),
                    to: ctx.accounts.game_vault.to_account_info(),
                    authority: player.to_account_info(),
                },
            ),
            BET_AMOUNT,
        )?;

        // Create VRF request
        // let vrf_request = VrfRequestParams {
        //     callback_program_id: ctx.program_id,
        //     callback_accounts: vec![
        //         AccountMeta::new(ctx.accounts.game_account.key(), false),
        //         AccountMeta::new(ctx.accounts.player.key(), false),
        //         AccountMeta::new(ctx.accounts.player_token_account.key(), false),
        //         AccountMeta::new(ctx.accounts.game_vault.key(), false),
        //     ],
        //     callback_data: guess.to_le_bytes().to_vec(),
        // };

        // let vrf_program = ctx.accounts.vrf.to_account_info();
        // let request_accounts = orao_solana_vrf::cpi::accounts::Request {
        //     payer: ctx.accounts.player.to_account_info(),
        //     network_state: ctx.accounts.config.to_account_info(),
        //     treasury: ctx.accounts.treasury.to_account_info(),
        //     request: ctx.accounts.request.to_account_info(),
        //     system_program: ctx.accounts.system_program.to_account_info(),
        // };
        // let cpi_ctx = CpiContext::new(vrf_program, request_accounts);
        // let vrf_key = orao_solana_vrf::cpi::request(cpi_ctx, seed)?;

        // // Request randomness from ORAO
        // let vrf_key = orao_solana_vrf::get_randomness(
        //     &ctx.accounts.vrf_program.to_account_info(),
        //     &ctx.accounts.vrf_account.to_account_info(),
        //     &vrf_request,
        // )?;

        // The VRF key is the public key of the request account
        // let vrf_key = ctx.accounts.request.key();

        // // Make the VRF request
        // let vrf_program = ctx.accounts.vrf.to_account_info();
        // let request_accounts = orao_solana_vrf::cpi::accounts::Request {
        //     payer: ctx.accounts.player.to_account_info(),
        //     network_state: ctx.accounts.config.to_account_info(),
        //     treasury: ctx.accounts.treasury.to_account_info(),
        //     request: ctx.accounts.request.to_account_info(),
        //     system_program: ctx.accounts.system_program.to_account_info(),
        // };
        // let cpi_ctx = CpiContext::new(vrf_program, request_accounts);

        // // Generate a seed for randomness (could be based on timestamp or other factors)
        // let seed = Clock::get()?.slot.to_le_bytes();
        // orao_solana_vrf::cpi::request(cpi_ctx, seed)
        //     .map_err(|e| anchor_lang::error!(ErrorCode::RandomnessRequestFailed))?;

        // // // Store roll data
        // let game_account = &mut ctx.accounts.game_account;
        // game_account.rolls.insert(
        //     vrf_key,
        //     RollData {
        //         player: *player.key,
        //         player_token_account: ctx.accounts.player_token_account.key(),
        //         guess,
        //         bet_amount: BET_AMOUNT,
        //         timestamp: Clock::get()?.unix_timestamp,
        //     },
        // );

        emit!(PlayerChoseNumber {
            player: *player.key,
            guess,
            bet_amount: BET_AMOUNT,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    pub fn settle_game(ctx: Context<SettleGame>, vrf_result: [u8; 32]) -> Result<()> {
        let game_account = &mut ctx.accounts.game_account;

        // Get roll data
        // let roll_data = game_account
        //     .rolls
        //     .get(&ctx.accounts.vrf_account.key())
        //     .ok_or(ErrorCode::InvalidRoll)?;

        // // Calculate winning number (1-6)
        // let winning_number = (u8::from_le_bytes([vrf_result[0]]) % 6) + 1;

        // if roll_data.guess == winning_number {
        //     // Calculate prize (4.2x multiplier)
        //     let prize_amount = BET_AMOUNT + (BET_AMOUNT * 42 / 10);

        //     // Transfer prize
        //     token::transfer(
        //         CpiContext::new_with_signer(
        //             ctx.accounts.token_program.to_account_info(),
        //             token::Transfer {
        //                 from: ctx.accounts.game_vault.to_account_info(),
        //                 to: ctx.accounts.player_token_account.to_account_info(),
        //                 authority: ctx.accounts.game_account.to_account_info(),
        //             },
        //             &[&[
        //                 b"game",
        //                 &[*ctx.bumps.get("game_account").unwrap()],
        //             ]],
        //         ),
        //         prize_amount,
        //     )?;

        // emit!(PlayerWon {
        //     player: roll_data.player,
        //     winning_number,
        //     prize_amount,
        //     timestamp: Clock::get()?.unix_timestamp,
        // });
        // }

        // Clean up
        game_account.rolls.remove(&ctx.accounts.vrf_account.key());

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 1000 // Adjust space as needed
    )]
    pub game_account: Account<'info, GameState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ChooseNumber<'info> {
    #[account(mut)]
    pub game_account: Account<'info, GameState>,
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut)]
    pub player_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub game_vault: Account<'info, TokenAccount>,
    /// CHECK: ORAO VRF Program
    pub vrf_program: AccountInfo<'info>,
    // pub config: Account<'info, ConfigAccount>,  // Network state/config for VRF
    /// CHECK: ORAO VRF Account
    #[account(mut)]
    pub vrf_account: AccountInfo<'info>,
    pub request: Account<'info, RequestAccount>, // VRF Request Account
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct RequestAccount {
    pub randomness: Option<[u8; 32]>, // Randomness result, if fulfilled.
    pub requester: Pubkey,           // Account that initiated the request.
    pub status: u8,                  // Status of the request (e.g., pending, fulfilled).
    // Other fields specific to the ORAO VRF program.
}

#[derive(Accounts)]
pub struct SettleGame<'info> {
    #[account(mut)]
    pub game_account: Account<'info, GameState>,
    #[account(mut)]
    pub player_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub game_vault: Account<'info, TokenAccount>,
    /// CHECK: ORAO VRF Account
    pub vrf_account: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct GameState {
    pub authority: Pubkey,
    pub rolls: BTreeMap<Pubkey, RollData>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RollData {
    pub player: Pubkey,
    pub player_token_account: Pubkey,
    pub guess: u8,
    pub bet_amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct PlayerChoseNumber {
    pub player: Pubkey,
    pub guess: u8,
    pub bet_amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct PlayerWon {
    pub player: Pubkey,
    pub winning_number: u8,
    pub prize_amount: u64,
    pub timestamp: i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid number - must be between 1 and 6")]
    InvalidNumber,
    #[msg("Randomness request failed")]
    RandomnessRequestFailed,
    #[msg("Insufficient funds for bet")]
    InsufficientFunds,
    #[msg("Invalid roll data")]
    InvalidRoll,
}
