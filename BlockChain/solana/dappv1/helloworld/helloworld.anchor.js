const anchor = require('@coral-xyz/anchor');
const { Connection, Keypair } = require('@solana/web3.js');

// Define the IDL for the Anchor program
const idl = {
  "address": "HHpyXUa97M6v9i5C5qK375JiYjckMEEDBaCLAA1SMSmv",
  "metadata": {
    "name": "hello",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "initialize",
      "discriminator": [
        175,
        175,
        109,
        31,
        13,
        152,
        155,
        237
      ],
      "accounts": [],
      "args": []
    }
  ]
};

// Set up the connection to the Solana network (use devnet for testing)
const connection = new Connection("https://api.devnet.solana.com", 'confirmed'); // Use 'devnet' or another network URL

// Load your wallet (make sure the key is correct)
const wallet = Keypair.fromSecretKey(new Uint8Array([216,104,91,138,31,220,232,208,49,100,46,250,151,189,12,6,167,247,211,26,125,177,6,74,48,213,135,56,144,167,81,22,183,65,35,212,85,59,3,74,69,129,12,227,27,128,71,179,162,31,154,21,14,126,180,153,114,63,92,250,162,150,169,105]));

// Set up the Anchor provider
// const provider =  new anchor.AnchorProvider(connection, wallet);
const provider =  anchor.AnchorProvider.local();
anchor.setProvider(provider);

const program = new anchor.Program(idl);

// Function to initialize the program
async function initializeProgram() {
  console.log("Attempting to initialize the program...");

  try {
    // Execute the 'initialize' instruction
    const tx = await program.methods.initialize().accounts([]).signers([wallet]).rpc();
    console.log("Transaction signature:", tx);
  } catch (error) {
    console.error("Failed to initialize program with error:", error);
  }
}

// Call the function to initialize the program
initializeProgram();