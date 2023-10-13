import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Solana6551 } from "../target/types/solana6551";
import { getNftPDA, getNftpdaPDA, getAssociatedAddress } from "../utils/pdas";
import { Keypair } from "@solana/web3.js";
import bs58 from 'bs58';
import { SPL_ATA_PROGRAM_ID, SPL_TOKEN_PROGRAM_ID} from "../utils/CONSTANTS";
import { BN } from "bn.js";
import { PublicKey } from "@metaplex-foundation/js";
import { publicKey } from "@metaplex-foundation/umi";
require('dotenv').config();

describe("solana6551", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Solana6551 as Program<Solana6551>;
  const wallet1 = Keypair.fromSecretKey(bs58.decode(process.env.SECRET_KEY1));
  const wallet2 = Keypair.fromSecretKey(bs58.decode(process.env.SECRET_KEY2));
  
  // it("create nft pda", async() => {
  //   const nftMint = new PublicKey("GC2jtBMLjwfxEXSRrW7HcYBCpKCqPRFr2RDigdY6CVHC")
  //   const nftPda = await getNftPDA(nftMint)
  //   const tokenAddr = await getAssociatedAddress(nftMint,wallet1.publicKey)
  //   try {
  //     const tx = await program.methods.createNftPda()
  //       .accounts({
  //         payer: wallet1.publicKey,
  //         nftMint: nftMint,
  //         nftTokenAccount: tokenAddr,
  //         nftPda: nftPda,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       }).signers([wallet1])
  //       .rpc()
  //       console.log(tx)
  //   } catch (error) {
  //     console.log(error)
  //   }
  // })

  // it("create nftpda pda", async() => {
  //   const nftMint = new PublicKey("GC2jtBMLjwfxEXSRrW7HcYBCpKCqPRFr2RDigdY6CVHC")
  //   const nftPda = await getNftPDA(nftMint)
  //   const nftpdaPda = await getNftpdaPDA(nftMint, new BN(0))
  //   const tokenAddr = await getAssociatedAddress(nftMint,wallet1.publicKey)
  //   try {
  //     const tx = await program.methods.createNftpdaPda("Hello world")
  //       .accounts({
  //         payer: wallet1.publicKey,
  //         nftpdaPda: nftpdaPda,
  //         nftPda: nftPda,
  //         nativeNftMint: nftMint,
  //         nativeNftTokenAccount: tokenAddr,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       }).signers([wallet1])
  //       .rpc()
  //       console.log(tx)
  //   } catch (error) {
  //     console.log(error)
  //   }
  // })

  // it("transfer native nft", async() => {
  //   const nftMint = new PublicKey("J3EU9n3b5m3fh2VEPcmXLtWpAdouBHdhXDgitFrAusqm")
  //   const nftNativeMint = new PublicKey("GC2jtBMLjwfxEXSRrW7HcYBCpKCqPRFr2RDigdY6CVHC")
  //   const nftPda = await getNftPDA(nftNativeMint)
  //   const fromTokenAddr = await getAssociatedAddress(nftMint,wallet1.publicKey)
  //   const toTokenAddr = await getAssociatedAddress(nftMint,nftPda)
  //   try {
  //     const tx = await program.methods.transferNativeNft()
  //       .accounts({
  //         from: wallet1.publicKey,
  //         to: nftPda,
  //         fromNftTokenAccount: fromTokenAddr,
  //         nftMint: nftMint,
  //         toNftTokenAccount: toTokenAddr,
  //         associatedTokenProgram: SPL_ATA_PROGRAM_ID,
  //         tokenProgram: SPL_TOKEN_PROGRAM_ID,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //         rent:anchor.web3.SYSVAR_RENT_PUBKEY
  //       }).signers([wallet1])
  //       .rpc()
  //       console.log(tx)
  //   } catch (error) {
  //     console.log(error)
  //   }
  // })

  // it("transfer native sol", async() => {
  //   try {
  //     const tx = await program.methods.transferNativeSol(new BN(10000000))
  //       .accounts({
  //         from: wallet1.publicKey,
  //         to: wallet2.publicKey,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //         rent:anchor.web3.SYSVAR_RENT_PUBKEY
  //       }).signers([wallet1])
  //       .rpc()
  //       console.log(tx)
  //   } catch (error) {
  //     console.log(error)
  //   }
  // })

  // it("transfer native token", async() => {
  //   const tokenMint = new PublicKey("Hrb6WDZayxRJXio46Y9VV2HJxNHmNMTSjwu9t2JaL6E8")
  //   const fromTokenAddr = await getAssociatedAddress(tokenMint,wallet1.publicKey)
  //   const toTokenAddr = await getAssociatedAddress(tokenMint,wallet2.publicKey)
  //   try {
  //     const tx = await program.methods.transferNativeToken(new BN(10))
  //       .accounts({
  //         from: wallet1.publicKey,
  //         to: wallet2.publicKey,
  //         fromTokenAccount: fromTokenAddr,
  //         tokenMint: tokenMint,
  //         toTokenAccount: toTokenAddr,
  //         associatedTokenProgram: SPL_ATA_PROGRAM_ID,
  //         tokenProgram: SPL_TOKEN_PROGRAM_ID,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //         rent:anchor.web3.SYSVAR_RENT_PUBKEY
  //       }).signers([wallet1])
  //       .rpc()
  //       console.log(tx)
  //   } catch (error) {
  //     console.log(error)
  //   }
  // })

  // it("transfer pda token", async() => {
  //   const tokenMint = new PublicKey("43oBptqvs3g7P4mFyM6cGkVwfXoTSXXuxi563wHt9JMW")
  //   const nftMint = new PublicKey("GC2jtBMLjwfxEXSRrW7HcYBCpKCqPRFr2RDigdY6CVHC")
  //   const nftPda = await getNftPDA(nftMint)
  //   const nativeNftTokenAddr = await getAssociatedAddress(nftMint,wallet1.publicKey)
  //   const fromTokenAddr = await getAssociatedAddress(tokenMint,nftPda)
  //   const toTokenAddr = await getAssociatedAddress(tokenMint,wallet2.publicKey)
  //   try {
  //     const tx = await program.methods.transferPdaToken(new BN(100000000))
  //       .accounts({
  //         payer: wallet1.publicKey,
  //         to: wallet2.publicKey,
  //         fromTokenAccount: fromTokenAddr,
  //         tokenMint: tokenMint,
  //         nftPda: nftPda,
  //         nativeNftMint: nftMint,
  //         nativeNftTokenAccount: nativeNftTokenAddr,
  //         toTokenAccount: toTokenAddr,
  //         associatedTokenProgram: SPL_ATA_PROGRAM_ID,
  //         tokenProgram: SPL_TOKEN_PROGRAM_ID,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //         rent:anchor.web3.SYSVAR_RENT_PUBKEY
  //       }).signers([wallet1])
  //       .rpc()
  //       console.log(tx)
  //   } catch (error) {
  //     console.log(error)
  //   }
  // })

  // it("transfer pda nft", async() => {
  //   const nativeNftMint = new PublicKey("GC2jtBMLjwfxEXSRrW7HcYBCpKCqPRFr2RDigdY6CVHC")
  //   const nftPda = await getNftPDA(nativeNftMint)
  //   const nftMint = new PublicKey("J3EU9n3b5m3fh2VEPcmXLtWpAdouBHdhXDgitFrAusqm")
  //   const nativeNftTokenAddr = await getAssociatedAddress(nativeNftMint,wallet1.publicKey)
  //   const fromTokenAddr = await getAssociatedAddress(nftMint,nftPda)
  //   const toTokenAddr = await getAssociatedAddress(nftMint,wallet2.publicKey)
  //   try {
  //     const tx = await program.methods.transferPdaNft()
  //       .accounts({
  //         payer: wallet1.publicKey,
  //         to: wallet2.publicKey,
  //         fromNftTokenAccount: fromTokenAddr,
  //         nftMint: nftMint,
  //         nftPda: nftPda,
  //         nativeNftMint: nativeNftMint,
  //         nativeNftTokenAccount: nativeNftTokenAddr,
  //         toNftTokenAccount: toTokenAddr,
  //         associatedTokenProgram: SPL_ATA_PROGRAM_ID,
  //         tokenProgram: SPL_TOKEN_PROGRAM_ID,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //         rent:anchor.web3.SYSVAR_RENT_PUBKEY
  //       }).signers([wallet1])
  //       .rpc()
  //       console.log(tx)
  //   } catch (error) {
  //     console.log(error)
  //   }
  // })

  it("transfer pda sol", async() => {
    const nativeNftMint = new PublicKey("GC2jtBMLjwfxEXSRrW7HcYBCpKCqPRFr2RDigdY6CVHC")
    const nftPda = await getNftPDA(nativeNftMint)
    const nativeNftTokenAddr = await getAssociatedAddress(nativeNftMint,wallet1.publicKey)
    try {
      const tx = await program.methods.transferPdaSol(new BN(100000))
        .accounts({
          payer: wallet1.publicKey,
          to: wallet2.publicKey,
          nftPda: nftPda,
          nativeNftMint: nativeNftMint,
          nativeNftTokenAccount: nativeNftTokenAddr,
          tokenProgram: SPL_TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
          rent:anchor.web3.SYSVAR_RENT_PUBKEY
        }).signers([wallet1])
        .rpc()
        console.log(tx)
    } catch (error) {
      console.log(error)
    }
  })
});
