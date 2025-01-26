import {
  LAMPORTS_PER_SOL,
  Connection,
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction,
  Keypair,
  PublicKey
} from "@solana/web3.js";

import base58 from "bs58";

const connection = new Connection("https://api.devnet.solana.com", "confirmed");

const secret = base58.decode("5m4kqjcNVcdiEzreA4AtXuiVBdwQ4dLNiSTe8oSKqBSRkspHj8zdRdUbLfupU1cabNXr6EYtmU8pUKAYW4uGEB6D");
const sender = Keypair.fromSecretKey(secret);
const receiver = new PublicKey("F2w5mnw5HKSWjBEho8po9iGmAcKdmfi6AoYA9osAneSJ");

// Check and log balance before transfer
const preBalance1 = await connection.getBalance(sender.publicKey);
const preBalance2 = await connection.getBalance(receiver);
console.log("Sender pre-balance:", preBalance1 / LAMPORTS_PER_SOL);
console.log("Receiver pre-balance:", preBalance2 / LAMPORTS_PER_SOL);
console.log("\n");

// Define the amount to transfer
const transferAmount = 0.01; // 0.01 SOL

// Create a transfer instruction for transferring SOL from sender to receiver
const transferInstruction = SystemProgram.transfer({
  fromPubkey: sender.publicKey,
  toPubkey: receiver,
  lamports: transferAmount * LAMPORTS_PER_SOL, // Convert transferAmount to lamports
});

// Add the transfer instruction to a new transaction
const transaction = new Transaction().add(transferInstruction);

console.log(transaction);

// Send the transaction to the network
try {
  const transactionSignature = await sendAndConfirmTransaction(
    connection,
    transaction,
    [sender] // signer
  );

  // Check and log balance after transfer
  const postBalance1 = await connection.getBalance(sender.publicKey);
  const postBalance2 = await connection.getBalance(receiver);

  console.log("Sender post-balance:", postBalance1 / LAMPORTS_PER_SOL);
  console.log("Receiver post-balance:", postBalance2 / LAMPORTS_PER_SOL);
  console.log("\n");

  console.log(
    "Transaction Signature:",
    `https://explorer.solana.com/tx/${transactionSignature}?cluster=devnet`
  );
} catch (error) {
  console.error("Failed to send transaction:", error);
}