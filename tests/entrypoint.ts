import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import type { Entrypoint } from "../target/types/entrypoint";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, sendAndConfirmTransaction, SystemProgram, Transaction } from "@solana/web3.js";
import { createInitializeMint2Instruction, getAccount, getMinimumBalanceForRentExemptMint, MINT_SIZE, TOKEN_PROGRAM_ID } from "@solana/spl-token";

// 
// 
// 
// 
// 
// 
// ----------

anchor.setProvider(anchor.AnchorProvider.env());
const program = anchor.workspace.entrypoint as Program<Entrypoint>;
const connection = program.provider.connection;

// 
// 
// 
// 
// 
describe("entrypoint", () => {
  const payer = program.provider.wallet.payer;
  console.log(payer.publicKey.toBase58());
  const mint = Keypair.generate();

  // 
  // 
  // 
  // 

  // it("create mint account", async () => {
  //   const createAccIx = SystemProgram.createAccount({
  //     fromPubkey: payer.publicKey,
  //     newAccountPubkey: mint.publicKey,
  //     space: MINT_SIZE,
  //     lamports: await getMinimumBalanceForRentExemptMint(connection, "processed"),
  //     programId: TOKEN_PROGRAM_ID
  //   });

  //   const initMintIx = createInitializeMint2Instruction(
  //     mint.publicKey, 2, payer.publicKey, null
  //   );

  //   const tx = new Transaction().add(createAccIx, initMintIx);
  //   const signature = await sendAndConfirmTransaction(
  //     connection, tx, [payer, mint]
  //   );
  //   // printLogs(signature);
  // })

  // 
  // 
  // 
  // 

  it("start auction", async () => {

    const info = Keypair.generate();

    const auctionInfo = {
      startPrice: new anchor.BN(LAMPORTS_PER_SOL * 100),
      endPrice: new anchor.BN(LAMPORTS_PER_SOL * 50),
      duration: new anchor.BN(2 * 60 * 60 * 1000),
      dropInterval: new anchor.BN(5 * 60 * 1000),
      dropStep: new anchor.BN(LAMPORTS_PER_SOL * 10)
    };

    const tx = await program.methods.startAuction(
      auctionInfo.startPrice, auctionInfo.endPrice, auctionInfo.duration, auctionInfo.dropInterval, auctionInfo.dropStep
    )
      .rpc({ commitment: "confirmed" });
    printLogs(tx);
  })

  // 
  // 
  // 
  // 

  it("auction", async () => {
    const tx = await program.methods
      .auction(new anchor.BN(LAMPORTS_PER_SOL * 120))
      .rpc({ commitment: "confirmed" });
    
    printLogs(tx);
  })

});

// 
// 
// 
// 

const printLogs = (signature: string) => {
  connection.getParsedTransaction(signature, "confirmed").then(res => {
    const logMessages = res.meta.logMessages;
    console.log(logMessages);
  });
}