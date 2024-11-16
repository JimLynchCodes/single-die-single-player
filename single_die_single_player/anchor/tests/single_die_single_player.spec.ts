import * as anchor from '@coral-xyz/anchor'
import {Program} from '@coral-xyz/anchor'
import {Keypair} from '@solana/web3.js'
import {SingleDieSinglePlayer} from '../target/types/single_die_single_player'

describe('single_die_single_player', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const payer = provider.wallet as anchor.Wallet

  const program = anchor.workspace.SingleDieSinglePlayer as Program<SingleDieSinglePlayer>

  const single_die_single_playerKeypair = Keypair.generate()

  it('Initialize SingleDieSinglePlayer', async () => {
    await program.methods
      .initialize()
      .accounts({
        single_die_single_player: single_die_single_playerKeypair.publicKey,
        payer: payer.publicKey,
      })
      .signers([single_die_single_playerKeypair])
      .rpc()

    const currentCount = await program.account.single_die_single_player.fetch(single_die_single_playerKeypair.publicKey)

    expect(currentCount.count).toEqual(0)
  })

  it('Increment SingleDieSinglePlayer', async () => {
    await program.methods.increment().accounts({ single_die_single_player: single_die_single_playerKeypair.publicKey }).rpc()

    const currentCount = await program.account.single_die_single_player.fetch(single_die_single_playerKeypair.publicKey)

    expect(currentCount.count).toEqual(1)
  })

  it('Increment SingleDieSinglePlayer Again', async () => {
    await program.methods.increment().accounts({ single_die_single_player: single_die_single_playerKeypair.publicKey }).rpc()

    const currentCount = await program.account.single_die_single_player.fetch(single_die_single_playerKeypair.publicKey)

    expect(currentCount.count).toEqual(2)
  })

  it('Decrement SingleDieSinglePlayer', async () => {
    await program.methods.decrement().accounts({ single_die_single_player: single_die_single_playerKeypair.publicKey }).rpc()

    const currentCount = await program.account.single_die_single_player.fetch(single_die_single_playerKeypair.publicKey)

    expect(currentCount.count).toEqual(1)
  })

  it('Set single_die_single_player value', async () => {
    await program.methods.set(42).accounts({ single_die_single_player: single_die_single_playerKeypair.publicKey }).rpc()

    const currentCount = await program.account.single_die_single_player.fetch(single_die_single_playerKeypair.publicKey)

    expect(currentCount.count).toEqual(42)
  })

  it('Set close the single_die_single_player account', async () => {
    await program.methods
      .close()
      .accounts({
        payer: payer.publicKey,
        single_die_single_player: single_die_single_playerKeypair.publicKey,
      })
      .rpc()

    // The account should no longer exist, returning null.
    const userAccount = await program.account.single_die_single_player.fetchNullable(single_die_single_playerKeypair.publicKey)
    expect(userAccount).toBeNull()
  })
})
