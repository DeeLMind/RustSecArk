// 导入sol web3.js
import { PublicKey,Connection } from "@solana/web3.js";
import { getMint, TOKEN_2022_PROGRAM_ID } from "@solana/spl-token";

// 配置网络
const connection = new Connection("https://api.devnet.solana.com", "confirmed");


const address = new PublicKey("C33qt1dZGZSsqTrHdtLKXPZNoxs6U1ZBfyDkzmj6mXeR");
const accountInfo = await connection.getAccountInfo(address);
 
console.log(JSON.stringify(accountInfo, null, 2));

const mintData = await getMint(
    connection,
    address,
    "confirmed",
    TOKEN_2022_PROGRAM_ID,
  );
   
  console.log(mintData);