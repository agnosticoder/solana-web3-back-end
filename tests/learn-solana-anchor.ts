import * as anchor from '@project-serum/anchor';

const {SystemProgram}  = anchor.web3;

const main = async () => {
  console.log("🚀 Starting Test...");

  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.LearnSolanaAnchor;

  //* Create an account key-pair for our prgram to use
  const baseAccount = anchor.web3.Keypair.generate();

  //* Call start_stuff_off, pass the params it needs
  const tx = await program.rpc.startStuffOff({
    accounts:{
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId
    },
    signers: [baseAccount]
  });
  console.log('Your Transaction Signature is: ', tx);

  //Update totalGif
  const tx2 = await program.rpc.addGif('link first',{accounts:{baseAccount: baseAccount.publicKey, user: provider.wallet.publicKey}});
  console.log('🏴‍☠️ Your Second Transaction Signature is: ', tx2);


  //Fetch data from the account 
  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 Gif Count: ', account.totalGifs.toString());
  console.log('🧊 I want to see some struct', account.gifList);
}

main().catch(err => {
  console.error({err});
  process.exit(1);
});


// import * as anchor from "@project-serum/anchor";
// import { Program } from "@project-serum/anchor";
// import { LearnSolanaAnchor } from "../target/types/learn_solana_anchor";

// const { SystemProgram } = anchor.web3;

// describe("learn-solana-anchor", () => {

//   // Configure the client to use the local cluster.
//   const provider = anchor.Provider.env();
//   anchor.setProvider(provider);

//   const program = anchor.workspace.LearnSolanaAnchor; 

//    // Create an account keypair for our program to use.
//   const baseAccount = anchor.web3.Keypair.generate();

//   it("Is initialized!", async () => {
//     // Add your test here.
//     const tx = await program.rpc.initialize({
//       accounts: {
//         baseAccount: baseAccount.publicKey,
//         user: provider.wallet.publicKey,
//         systemProgram: SystemProgram.programId
//       },
//       signers: [baseAccount]
//     });
//     console.log("Your transaction signature", tx);

//     //* Fetch data from the account
//     let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
//     console.log('Gif Count', account.totalGifs.toString());

//     await program.rpc.addGif({
//       accounts:{
//         baseAccount: baseAccount.publicKey
//       }
//     })


//     let account2 = await program.account.baseAccount.fetch(baseAccount.publicKey);
//     console.log('Gif Count', account2.totalGifs.toString());
//   });
// });
