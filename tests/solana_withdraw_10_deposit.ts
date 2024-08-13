import * as anchor from "@coral-xyz/anchor";
import { SolanaWithdraw10Deposit } from "../target/types/solana_withdraw_10_deposit";
import { assert } from "chai";

describe("solana_withdraw_10_deposit", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaWithdraw10Deposit as anchor.Program<SolanaWithdraw10Deposit>;

  const TOTAL_DEPOSITS = 1 * anchor.web3.LAMPORTS_PER_SOL; // 1 SOL
  const EXPECTED_WITHDRAW_AMOUNT = TOTAL_DEPOSITS / 10; // 10% of deposited amount

  const [userVaultAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), provider.wallet.publicKey.toBuffer()],
    program.programId
  );

  const [totalInteractionsAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("counter"), provider.wallet.publicKey.toBuffer()],
    program.programId
  );

  const [userBalanceAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("balance"), provider.wallet.publicKey.toBuffer()],
    program.programId
  );

  it("Deposit into Vault", async () => {
    const amount = new anchor.BN(TOTAL_DEPOSITS); // 1 SOL
    const tx = await program.methods
      .deposit(amount)
      .accounts({
        userVaultAccount: userVaultAccount,
        userInteractionsCounter: totalInteractionsAccount,
        signer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        userBalance: userBalanceAccount,
      })
      .rpc();
    console.log("Deposit transaction signature:", tx);
    console.log(`SolScan transaction link: https://solscan.io/tx/${tx}?cluster=devnet`);

    await provider.connection.confirmTransaction({
      signature: tx,
      blockhash: (await provider.connection.getLatestBlockhash()).blockhash,
      lastValidBlockHeight: (await provider.connection.getLatestBlockhash()).lastValidBlockHeight,
    });

    const vaultData = await program.account.userInteractions.fetch(totalInteractionsAccount);
    const balance = await provider.connection.getBalance(userVaultAccount);

    console.log("On-chain data - totalDeposits:", vaultData.totalDeposits.toString());
    console.log("User Vault Account Balance:", balance / anchor.web3.LAMPORTS_PER_SOL, "SOL");

    assert.isTrue(vaultData.totalDeposits.toNumber() > 0, "Total deposits should increase");
    assert.equal(balance, TOTAL_DEPOSITS);
    assert.equal(balance, TOTAL_DEPOSITS, "Deposited amount should match");
  });

  it("Withdraw from Vault", async () => {
    const withdrawTx = await program.methods
      .withdraw()
      .accounts({
        userVaultAccount: userVaultAccount,
        userInteractionsCounter: totalInteractionsAccount,
        signer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        userBalance: userBalanceAccount,
      })
      .rpc();

    await provider.connection.confirmTransaction({
      signature: withdrawTx,
      blockhash: (await provider.connection.getLatestBlockhash()).blockhash,
      lastValidBlockHeight: (await provider.connection.getLatestBlockhash()).lastValidBlockHeight,
    });

    const vaultData = await program.account.userInteractions.fetch(totalInteractionsAccount);
    const balance = await provider.connection.getBalance(userVaultAccount);

    console.log("Total Deposits:", TOTAL_DEPOSITS / anchor.web3.LAMPORTS_PER_SOL, "SOL");
    console.log("Expected Withdraw Amount:", EXPECTED_WITHDRAW_AMOUNT / anchor.web3.LAMPORTS_PER_SOL, "SOL");
    console.log("Vault Data - totalWithdrawals:", vaultData.totalWithdrawals.toString());
    console.log("Expected Balance:", (TOTAL_DEPOSITS - EXPECTED_WITHDRAW_AMOUNT) / anchor.web3.LAMPORTS_PER_SOL, "SOL");
    console.log("Actual Balance:", balance / anchor.web3.LAMPORTS_PER_SOL, "SOL");

    assert.isTrue(vaultData.totalWithdrawals.toNumber() > 0, "Total withdrawals should increase");
    assert.equal(balance, TOTAL_DEPOSITS - EXPECTED_WITHDRAW_AMOUNT);
    assert.equal(
      balance,
      TOTAL_DEPOSITS - EXPECTED_WITHDRAW_AMOUNT,
      "Remaining deposited amount should be correct"
    );
  });
});