import {
    Connection,
    PublicKey,
    Transaction,
    SystemProgram,
    Keypair,
    sendAndConfirmTransaction,
  } from '@solana/web3.js';
  
// 设置连接到 Devnet（也可以替换为 mainnet 或 testnet）
const connection = new Connection('https://api.devnet.solana.com', 'confirmed');

// 你的程序的 PublicKey（Rust 程序的 ID）
const programId = new PublicKey('4NB2bAwWHjHHkxUB5p1N2u39xNmEbdn5mEAnkzVd4wYa');

// 创建一个新的 Keypair 作为交易的签署者（payer）
const payer = Keypair.fromSecretKey(new Uint8Array([216,104,91,138,31,220,232,208,49,100,46,250,151,189,12,6,167,247,211,26,125,177,6,74,48,213,135,56,144,167,81,22,183,65,35,212,85,59,3,74,69,129,12,227,27,128,71,179,162,31,154,21,14,126,180,153,114,63,92,250,162,150,169,105]));

// 创建一个简单的指令数据，这里我们可以传递一个空的字节数组或自定义的数据
const instructionData = Buffer.from([]);

// 创建交易指令
const instruction = new Transaction().add({
keys: [
    {
    pubkey: payer.publicKey,  // 使用 payer 账户的公钥
    isSigner: true,           // 该账户需要签署交易
    isWritable: false,        // 如果交易不涉及修改账户数据，设置为 false
    },
],
programId: programId,        // 目标程序的 Program ID
data: instructionData,       // 附带的数据
});

// 创建并发送交易
async function callSolanaProgram() {
try {
    // 将交易发送到网络并等待确认
    const txId = await sendAndConfirmTransaction(connection, instruction, [payer]);
    console.log("Transaction successful with ID:", txId);
} catch (err) {
    console.error("Transaction failed:", err);
}
}

callSolanaProgram();  