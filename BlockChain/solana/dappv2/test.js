import { generateKeyPairSigner } from "@solana/web3.js";
 
const signer = await generateKeyPairSigner();
console.log("address: ", signer.address);