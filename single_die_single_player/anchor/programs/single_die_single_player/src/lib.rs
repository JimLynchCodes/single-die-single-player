#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use anchor_spl::token;
use std::collections::BTreeMap;


declare_id!("DLw5fJNrrBCWoy75aukoDApBZm4MEvaWCvPJoqtLSg1p");

const BET_AMOUNT: u64 = 10; // Adjust the bet amount as needed

#[program]
pub mod single_die_single_player {
    use super::*;

    use anchor_lang::prelude::*;
    use anchor_spl::token;

    pub fn choose_number(ctx: Context<ChooseNumber>, guess: u8) -> Result<()> {
        // Program account that stores the game state
        let game_account = &mut ctx.accounts.game_account;
        let player_account = &ctx.accounts.player;

        // Check if the number is valid
        require!(guess >= 1 && guess <= 6, CustomError::InvalidNumber);

        require!(
            player_account.lamports() >= BET_AMOUNT,
            CustomError::InsufficientFunds
        );

        // Transfers bet from player to game
        token::transfer(
            ctx.accounts.into_context(token::Transfer {
                from: player_account.to_account_info(),
                to: game_account.to_account_info(),
                authority: player_account.to_account_info(),
            })?,
            BET_AMOUNT,
        )?;

        emit!(PlayerChoseNumber {
            player: *ctx.accounts.player.key,
            guess,
            bet_amount: BET_AMOUNT,
            bet_currency: "SOL".to_string(),
            timestamp: Clock::get()?.unix_timestamp,
        });

        // If this is the sixth player, reveal the winner
        // if game_account.players.len() == 6 {

        let oracle_queue_account = &ctx.accounts.oracle_queue_account;

        let vrf_key = make_orao_vrf_request(
            ctx.program_id,
            ctx.accounts.game_account.key(),
            ctx.accounts.oracle_account.to_account_info(),
            // settle_game
        )?;

        game_account.rolls.insert(
            vrf_key,
            Player {
                // address: *player_account.to_account_info().key,
                account: *player_account.to_account_info(),
                guess,
                BET_AMOUNT,
            },
        );

        // Request VRF from Switchboard (adapt to your specific Switchboard setup)
        // let vrf_key = switchboard_program::instruction::request_randomness(
        //     ctx.accounts.oracle_queue_account.to_account_info(),
        //     ctx.accounts.game_account.to_account_info(), // Callback address
        //     // ... other parameters as needed
        // )?;

        // Store the vrf_key in the game account for later retrieval
        // game_account.vrf_key = vrf_key;
        // }

        Ok(())
    }

    pub fn settle_game(ctx: Context<SettleGame>) -> Result<()> {
        let game_account = &mut ctx.accounts.game_account;
        let vrf_account = &ctx.accounts.vrf_account;

        let result = vrf_account
            .result
            .ok_or(CustomError::RandomnessNotAvailable)?;

        let winning_number = (result % 6) + 1;

        let roll_data = game_account
            .rolls
            .get(vrf_account.key)
            .ok_or(CustomError::CouldntReadRoll)?;

        if roll_data.guess == winning_number {
            let prize_amount = BET_AMOUNT + BET_AMOUNT * 42 / 10; // 4.2x multiplier + original bet
            token::transfer(
                ctx.accounts.into_context(token::Transfer {
                    from: game_account.to_account_info(),
                    to: roll_data.account,
                    authority: game_account.to_account_info(),
                })?,
                prize_amount,
            )?;

            emit!(PlayerWon {
                player: *ctx.accounts.player.key,
                winning_number,
                prize_amount,
                prize_currency: "SOL".to_string(),
                timestamp: Clock::get()?.unix_timestamp,
            });
        }

        if let Some(value) = game_account.rolls.remove(vrf_account.key) {
            msg!("Removed: {} => {}", vrf_account.key, value);
        } else {
            msg!("Key {} not found!", vrf_account.key);
        }

        Ok(())
    }
}

#[account]
pub struct ChooseNumber {
    pub rolls: BTreeMap<Pubkey, Player>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Player {
    pub address: Pubkey,
    pub guess: u8,
    pub bet_size: u64,
}

#[error]
pub enum CustomError {
    #[msg("Invalid number- expecting integer between 1 and 6")]
    InvalidNumber,
    #[msg("Invalid bet amount")]
    InvalidBetAmount,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Error reading player's roll")]
    CouldntReadRoll,
}

#[account]
pub struct SettleGame<'info> {
    pub game_account: Account<'info, ChooseNumber>,

    pub oracle_queue_account: Account<'info, VrfAccountData>,
}

#[event]
pub struct PlayerChoseNumber {
    pub player: Pubkey,
    pub guess: u8,
    pub bet_amount: u64,
    pub bet_currency: String,
    pub timestamp: i64,
}

#[event]
pub struct PlayerWon {
    pub player: Pubkey,
    pub winning_number: u8,
    pub prize_amount: u64,
    pub prize_currency: String,
    pub timestamp: i64,
}

/// Helper function to make an ORAO VRF request.
fn make_orao_vrf_request(
    program_id: &Pubkey,
    game_account_key: &Pubkey,
    vrf_account_info: AccountInfo<'_>,
) -> Result<Pubkey> {
    let callback_ix = Instruction {
        program_id: *program_id,
        accounts: vec![AccountMeta::new(*game_account_key, false)],
        // data: number_guessing_game::instruction::SettleGame {},
        // data: SettleGame {},
        // maybe this?

        // Get the function discriminator for "settle_game"
        // let settle_game_discriminator = anchor_lang::sighash("global", "settle_game");

        // // Prepare callback data (8-byte discriminator for "settle_game" and additional parameters)
        // let callback_data = settle_game_discriminator.to_vec();
        // data: callback_data,
    };

    orao_vrf_solana::instruction::request_randomness(
        vrf_account_info,
        game_account_key, // Callback address
        callback_ix,
    )?;

    // Return the vrf_key (you may need to adapt based on the ORAO setup).
    Ok(vrf_account_info.key.clone())
}
