import { PublicKey } from "@solana/web3.js";
 
const programId = new PublicKey("11111111111111111111111111111111");
const string = "helloWorld";
const bump = 254;
 
const PDA = PublicKey.createProgramAddressSync(
  [Buffer.from(string), Buffer.from([bump])],
  programId,
);

console.log(`PDA: ${PDA}`);