import {
    Connection,
    Keypair,
    SystemProgram,
    LAMPORTS_PER_SOL,
    Transaction,
    sendAndConfirmTransaction,
  } from "@solana/web3.js";
   
import fs from 'fs';

let fromKeypair;
let toKeypair;

async function loadUser(){
  const secretKey = Uint8Array.from(JSON.parse(fs.readFileSync('C:\\Users\\Administrator\\.config\\solana\\id.json', 'utf-8')));
  fromKeypair = Keypair.fromSecretKey(secretKey);
  console.log(fromKeypair.publicKey);
  console.log(fromKeypair.secretKey);
}
await loadUser();

async function restoreUser(){
  const keypairBytes = Uint8Array.from([177,206,210,43,242,41,232,20,193,233,235,14,217,102,198,99,55,87,243,90,46,201,230,227,43,122,60,45,173,4,136,81,183,64,237,220,178,220,22,14,251,181,215,175,191,34,55,51,251,112,77,29,176,167,181,168,176,216,100,255,109,32,81,33]);
  toKeypair = Keypair.fromSecretKey(keypairBytes);
  console.log(toKeypair.publicKey);
  console.log(toKeypair.secretKey);
}
await restoreUser();

async function getAccountBalance() {
  try {
      const connection = new Connection("https://api.devnet.solana.com","confirmed",);
      const balance = await connection.getBalance(toKeypair.publicKey);
      // 将 lamports 转换为 SOL
      const balanceInSOL = balance / Math.pow(10, 9);
      console.log(`账户余额: ${balanceInSOL} SOL`);
  } catch (error) {
      console.error('获取账户余额时出错:', error);
  }
}

await getAccountBalance();

async function sendSol(){
  const connection = new Connection("https://api.devnet.solana.com","confirmed",);
  
  const lamportsToSend = 1_000_000;

  const transferTransaction = new Transaction().add(
    SystemProgram.transfer({
      fromPubkey: fromKeypair.publicKey,
      toPubkey: toKeypair.publicKey,
      lamports: lamportsToSend,
    }),
  );

  await sendAndConfirmTransaction(connection, transferTransaction, [
    fromKeypair,
  ]);
}
await sendSol();
await getAccountBalance();