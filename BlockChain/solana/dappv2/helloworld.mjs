import {
  Connection,
  PublicKey,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import { Keypair } from "@solana/web3.js";
import { getKeypairFromFile } from "@solana-developers/helpers";
   
  const programId = new PublicKey("CEiJvr96AdH39VVLFC6CCg6mWtng9YBzgHh1Kuv3678w");
   
  // Connect to a solana cluster. Either to your local test validator or to devnet
  const connection = new Connection("https://api.devnet.solana.com", "confirmed");
  //const connection = new Connection("https://api.devnet.solana.com", "confirmed");
   
  // We load the keypair that we created in a previous step
  const parsedFileContents = Uint8Array.from(JSON.parse("[38,250,151,107,121,15,229,21,50,115,219,165,231,26,98,148,181,10,33,49,212,184,139,36,142,239,196,252,102,181,210,235,208,130,89,75,117,7,163,83,150,20,106,78,40,97,139,254,79,146,74,181,212,14,206,8,24,37,31,191,8,38,102,71] "));
  const keyPair =  Keypair.fromSecretKey(parsedFileContents);
  // const keyPair =  getKeypairFromFile("id.json");
   
  // Every transaction requires a blockhash
  const blockhashInfo = await connection.getLatestBlockhash();
   
  // Create a new transaction
  const tx = new Transaction({
    ...blockhashInfo,
  });
   
  // Add our Hello World instruction
  tx.add(
    new TransactionInstruction({
      programId: programId,
      keys: [],
      data: Buffer.from([]),
    }),
  );
   
  // Sign the transaction with your previously created keypair
  tx.sign(keyPair);
   
  // Send the transaction to the Solana network
  const txHash = await connection.sendRawTransaction(tx.serialize());
   
  console.log("Transaction sent with hash:", txHash);
   
  await connection.confirmTransaction({
    blockhash: blockhashInfo.blockhash,
    lastValidBlockHeight: blockhashInfo.lastValidBlockHeight,
    signature: txHash,
  });
   
  console.log(
    `Congratulations! Look at your ‘Hello World' transaction in the Solana Explorer:
    https://explorer.solana.com/tx/${txHash}?cluster=custom`,
  );