import { PublicKey } from "@solana/web3.js";
 
const programId = new PublicKey("11111111111111111111111111111111");
 
const [PDA, bump] = PublicKey.findProgramAddressSync([], programId);
 
console.log(`PDA: ${PDA}`);
console.log(`Bump: ${bump}`);