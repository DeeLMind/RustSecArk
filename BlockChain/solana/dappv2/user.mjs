// 生成用户公钥私钥
// import { generateKeyPairSigner } from "@solana/web3.js";
// import { generateKeyPair } from "@solana/web3.js";
// async function generateUser(){
//     const keypair = generateKeyPair();
//     console.log('Public Key:', keypair.publicKey);
// }
// await generateUser();

// 加载用户
// import { Keypair } from "@solana/web3.js";
// import fs from 'fs';
// async function loadUser(){
//     const secretKey = Uint8Array.from(JSON.parse(fs.readFileSync('C:\\Users\\Administrator\\.config\\solana\\id.json', 'utf-8')));
//     const keypair = Keypair.fromSecretKey(secretKey);
//     console.log(keypair.publicKey);
//     console.log(keypair.secretKey);
// }
// await loadUser();

// 恢复密钥
// import { Keypair } from "@solana/web3.js";
// async function restoreUser(){
//     const keypairBytes = Uint8Array.from([
//         88, 143, 140,  41,  18, 253, 151, 249,  50,  63, 195,
//        219, 105,  32, 144, 105, 252, 140, 250, 108, 230, 237,
//        196, 112,  96,  47,  93, 172, 139, 210, 113, 252, 233,
//        107,  48, 140,  68, 156, 230,  12, 226,  65, 238,  41,
//         79, 223, 151,  30,  88, 238, 130, 189, 103, 105, 184,
//        211,  27, 159,  38,  58, 167, 163, 159,  73
//      ]);
     
//     const keypair = Keypair.fromSecretKey(keypairBytes);
//     console.log(keypair.publicKey);
//     console.log(keypair.secretKey);
// }
// await restoreUser();

import { isAddress } from "@solana/web3.js";
 
// Note that generateKeyPair() will always give a public key that is valid for users
 
// Valid public key
const key = "5oNDL3swdJJF1g9DzJiZ4ynHXgszjAEpUkxVYejchzrY";
 
// Lies on the ed25519 curve and is suitable for users
console.log("Valid Address: ", isAddress(key));
 
// // Valid public key
const offCurveAddress = "4BJXYkfvg37zEmBbsacZjeQDpTNx91KppxFJxRqrz48e";
 
// // Not on the ed25519 curve, therefore not suitable for users
console.log("Valid Off Curve Address: ", isAddress(offCurveAddress));
 
// // Not a valid public key
const errorPubkey = "testPubkey";
console.log("Invalid Address: ", isAddress(errorPubkey));

// 助记词生成


// 助记词恢复