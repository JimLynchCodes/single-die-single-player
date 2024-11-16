// Here we export some useful types and functions for interacting with the Anchor program.
import { AnchorProvider, Program } from '@coral-xyz/anchor'
import { Cluster, PublicKey } from '@solana/web3.js'
import SingleDieSinglePlayerIDL from '../target/idl/single_die_single_player.json'
import type { SingleDieSinglePlayer } from '../target/types/single_die_single_player'

// Re-export the generated IDL and type
export { SingleDieSinglePlayer, SingleDieSinglePlayerIDL }

// The programId is imported from the program IDL.
export const SINGLE_DIE_SINGLE_PLAYER_PROGRAM_ID = new PublicKey(SingleDieSinglePlayerIDL.address)

// This is a helper function to get the SingleDieSinglePlayer Anchor program.
export function getSingleDieSinglePlayerProgram(provider: AnchorProvider) {
  return new Program(SingleDieSinglePlayerIDL as SingleDieSinglePlayer, provider)
}

// This is a helper function to get the program ID for the SingleDieSinglePlayer program depending on the cluster.
export function getSingleDieSinglePlayerProgramId(cluster: Cluster) {
  switch (cluster) {
    case 'devnet':
    case 'testnet':
      // This is the program ID for the SingleDieSinglePlayer program on devnet and testnet.
      return new PublicKey('CounNZdmsQmWh7uVngV9FXW2dZ6zAgbJyYsvBpqbykg')
    case 'mainnet-beta':
    default:
      return SINGLE_DIE_SINGLE_PLAYER_PROGRAM_ID
  }
}
