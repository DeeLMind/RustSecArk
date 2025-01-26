import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import fs from 'fs';

let keypair = null;

async function loadUser(){
    const secretKey = Uint8Array.from(JSON.parse(fs.readFileSync('C:\\Users\\Administrator\\.config\\solana\\id.json', 'utf-8')));
    keypair = Keypair.fromSecretKey(secretKey);
    console.log(keypair.publicKey);
    console.log(keypair.secretKey);
}
await loadUser();

async function airdrop5(){
    const connection = new Connection("https://api.devnet.solana.com", "confirmed");
 
    const signature = await connection.requestAirdrop(
        new PublicKey(keypair.publicKey),
        5 * LAMPORTS_PER_SOL,
    );

    const { blockhash, lastValidBlockHeight } = await connection.getLatestBlockhash();
    
    await connection.confirmTransaction({
        blockhash,
        lastValidBlockHeight,
        signature,
    });
}
await airdrop5();