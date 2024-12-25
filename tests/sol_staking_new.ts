import * as anchor from "@project-serum/anchor";
import { SystemProgram } from "@solana/web3.js";
import { assert } from "chai";
import { PublicKey } from "@solana/web3.js";
import { StakingData, UserStake } from "../target/types/sol_staking_new";

describe("sol_staking_new", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolStakingNew;

  let authority: anchor.web3.Keypair;
  let stakingDataAccount: anchor.web3.Keypair;
  let staker: anchor.web3.Keypair;
  let stakerTokenAccount: anchor.web3.Keypair;
  let stakingPoolAccount: anchor.web3.Keypair;

  before(async () => {
    authority = anchor.web3.Keypair.generate();
    stakingDataAccount = anchor.web3.Keypair.generate();
    staker = anchor.web3.Keypair.generate();
    stakerTokenAccount = anchor.web3.Keypair.generate();
    stakingPoolAccount = anchor.web3.Keypair.generate();

    // Fund the staker account with some SOL for testing
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        staker.publicKey,
        1000000000 // Amount in lamports (1 SOL = 1 billion lamports)
      ),
      "confirmed"
    );

    // Create the staking data account
    await program.rpc.setStakingParameters(new anchor.BN(10_000), new anchor.BN(0), new anchor.BN(100000), new anchor.BN(1000), {
      accounts: {
        stakingData: stakingDataAccount.publicKey,
        authority: authority.publicKey,
      },
      signers: [authority, stakingDataAccount],
    });
  });

  it("Sets staking parameters", async () => {
    const stakingData: StakingData = await program.account.stakingData.fetch(stakingDataAccount.publicKey);

    assert.equal(stakingData.apy.toString(), "10000"); // Expected APY (10% APY)
    assert.equal(stakingData.stakingStart.toString(), "0"); // Expected staking start time
    assert.equal(stakingData.stakingEnd.toString(), "100000"); // Expected staking end time
    assert.equal(stakingData.lockDuration.toString(), "1000"); // Expected lock duration
  });

  it("Adds rewards to the staking pool", async () => {
    const amount = 1000;

    // Add rewards
    await program.rpc.addRewards(new anchor.BN(amount), {
      accounts: {
        stakingData: stakingDataAccount.publicKey,
        stakingPoolAccount: stakingPoolAccount.publicKey,
        authority: authority.publicKey,
        authorityTokenAccount: stakerTokenAccount.publicKey,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      },
      signers: [authority],
    });

    const stakingData: StakingData = await program.account.stakingData.fetch(stakingDataAccount.publicKey);

    assert.equal(stakingData.rewardPool.toString(), amount.toString()); // Ensure the reward pool is updated
  });

  it("Claims rewards", async () => {
    const stakedAmount = 1000;
    const apy = 10_000; // 10% APY
    const timeStaked = 10000; // Time staked in seconds
    const rewards = calculateRewards(stakedAmount, apy, timeStaked);

    // Claim rewards
    await program.rpc.claimRewards({
      accounts: {
        stakingData: stakingDataAccount.publicKey,
        userStake: staker.publicKey,
        staker: staker.publicKey,
        stakerTokenAccount: stakerTokenAccount.publicKey,
        stakingPoolAccount: stakingPoolAccount.publicKey,
        authority: authority.publicKey,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      },
      signers: [staker],
    });

    const userStake: UserStake = await program.account.userStake.fetch(staker.publicKey);
    assert.equal(userStake.claimedRewards.toString(), rewards.toString()); // Verify rewards were claimed
  });

  // Utility function to calculate rewards
  function calculateRewards(amount: number, apy: number, timeStaked: number): number {
    const secondsInYear = 365 * 24 * 60 * 60;
    const ratePerSecond = apy / 100 / secondsInYear;
    return Math.floor(amount * ratePerSecond * timeStaked);
  }
});
