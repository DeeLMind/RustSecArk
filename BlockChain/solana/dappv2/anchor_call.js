const anchor = require("@coral-xyz/anchor");
const { SystemProgram } = anchor.web3;
const fs = require("fs");
const path = require("path");

async function initializeCounter() {
    const provider = "https://api.devnet.solana.com";
    anchor.setProvider(provider);
    // Load the IDL from a local file
    const idlPath = path.resolve(__dirname, "./idl.json");
    const idl = JSON.parse(fs.readFileSync(idlPath, "utf-8"));
    const program = new anchor.Program(idl, new anchor.web3.PublicKey("4bZEQTxYGh4bjHnEpFyoHQ3gvNuCEe2mpSzkMKzmFGWv"), provider);

    const parsedFileContents = Uint8Array.from(JSON.parse("[38,250,151,107,121,15,229,21,50,115,219,165,231,26,98,148,181,10,33,49,212,184,139,36,142,239,196,252,102,181,210,235,208,130,89,75,117,7,163,83,150,20,106,78,40,97,139,254,79,146,74,181,212,14,206,8,24,37,31,191,8,38,102,71] "));
    const counter =  Keypair.fromSecretKey(parsedFileContents);

    // Call the initialize function
    await program.methods
        .initializeCounter(new anchor.BN(999)) // Initial value
        .accounts({
            counter: counter.publicKey,
            user: provider.wallet.publicKey,
            systemProgram: SystemProgram.programId,
        })
        .signers([counter])
        .rpc();
    console.log("Counter initialized!");
}

initializeCounter().catch(console.error);
