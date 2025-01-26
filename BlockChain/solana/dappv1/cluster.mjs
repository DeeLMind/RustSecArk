import { clusterApiUrl, Connection } from "@solana/web3.js";
 
(async () => {
  // const connection = new Connection(clusterApiUrl("mainnet-beta"), "confirmed");
  // const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
  const connection = new Connection(clusterApiUrl("testnet"), "confirmed");
  console.log(connection);
})();
 
(async () => {
  const connection = new Connection("https://api.devnet.solana.com", "confirmed");
  console.log(connection);
})();