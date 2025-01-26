const { Connection, PublicKey, TransactionInstruction,SystemProgram, Transaction, Keypair } = require('@solana/web3.js');
const { serialize, deserialize } = require('borsh');
const BN = require('bn.js');

// 定义Borsh schema
class CounterInstruction {
  static schema = new Map([
    [CounterInstruction, { kind: 'struct', fields: [['variant', 'u8'], ['initialValue', 'u64']] }]
  ]);

  constructor(variant, initialValue) {
    this.variant = variant;
    this.initialValue = new BN(initialValue);
  }
}

// 定义CounterAccount结构
class CounterAccount {
  static schema = new Map([
    [CounterAccount, { kind: 'struct', fields: [['count', 'u64']] }]
  ]);

  constructor({ count }) {
    this.count = new BN(count);
  }
}

// 初始化计数器函数
async function initializeCounter(connection, payer, counterPublicKey, programId, initialValue) {
  const instructionData = new CounterInstruction(0, initialValue); // 0表示InitializeCounter指令
  const serializedData = serialize(CounterInstruction.schema, instructionData);

  const instruction = new TransactionInstruction({
    keys: [
      { pubkey: counterPublicKey, isSigner: true, isWritable: true },
      { pubkey: payer.publicKey, isSigner: true, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId,
    data: Buffer.from(serializedData),
  });

  const transaction = new Transaction().add(instruction);
  await connection.sendTransaction(transaction, [payer]);
  console.log("计数器已初始化");
}



const connection = new Connection("https://api.devnet.solana.com", "confirmed");

const parsedFileContents = Uint8Array.from(JSON.parse("[38,250,151,107,121,15,229,21,50,115,219,165,231,26,98,148,181,10,33,49,212,184,139,36,142,239,196,252,102,181,210,235,208,130,89,75,117,7,163,83,150,20,106,78,40,97,139,254,79,146,74,181,212,14,206,8,24,37,31,191,8,38,102,71] "));
const payer =  Keypair.fromSecretKey(parsedFileContents);
const counter = Keypair.generate();

// 调用时的代码
initializeCounter(connection, payer, counter.publicKey, new PublicKey("7fn1RAgQNPb9kTFE4VfwfbobZXYUc2DFJj7Y5VKACkjG"), 999);

