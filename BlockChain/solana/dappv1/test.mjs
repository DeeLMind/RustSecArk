import { Connection, PublicKey, Keypair } from '@solana/web3.js';
import anchor from '@coral-xyz/anchor';

// Destructure the necessary exports
const { Program, AnchorProvider, Idl, Wallet } = anchor;

// 设置连接
const connection = new Connection('https://api.devnet.solana.com', 'confirmed');

// 加载 IDL 数据
const idl = {
  "address": "6be4ZEKUf8KjoKvsRmSs1Ls4vXqn9mLe3qkZu7GNp15",
  "metadata": {
    "name": "anchro_helloworld",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "initialize",
      "discriminator": [
        175, 175, 109, 31, 13, 152, 155, 237
      ],
      "accounts": [],
      "args": []
    }
  ]
};

// 你的钱包密钥对
const payer = Keypair.fromSecretKey(new Uint8Array([63,64,179,160,70,59,77,222,122,222,196,153,186,208,152,92,188,66,198,229,228,247,160,244,229,80,4,146,18,237,172,109,183,65,36,112,51,141,135,116,101,98,94,24,128,122,32,70,25,18,0,254,94,153,159,203,92,250,103,198,181,44,53,250]));


// 创建 AnchorProvider
const provider = new AnchorProvider(connection, new Wallet(payer), { commitment: 'confirmed' });

// 创建程序实例
const programId = new PublicKey(idl.address);
const program = new Program(idl, programId, provider);

// 发送交易调用 initialize 指令
async function callInitialize() {
  try {
    const tx = await program.methods
      .initialize()  // 指令名称
      .accounts({})   // 账户参数，这里没有账户
      .rpc();         // 执行指令

    console.log("Transaction ID:", tx);
  } catch (error) {
    console.error("Transaction failed:", error);
  }
}

callInitialize();
