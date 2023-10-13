import { PublicKey } from "@metaplex-foundation/js";
import * as anchor from "@project-serum/anchor";
import { PROGRAM_ID } from "./CONSTANTS";
import * as token from "@solana/spl-token";
import { Connection } from '@solana/web3.js';
import { BN } from "bn.js";

export const getNftPDA = async (
  nftMint: PublicKey,
): Promise<PublicKey> => {
  return (
    await anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("nft_pda"),
        nftMint.toBuffer(),
      ],
      PROGRAM_ID
    )
  )[0];
};

export const getNftpdaPDA = async (
  nftMint: PublicKey,
  num: anchor.BN
): Promise<PublicKey> => {
  return (
    await anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("nftpda_pda"),
        nftMint.toBuffer(),
        new BN(num).toArrayLike(Buffer, "le", 8)
      ],
      PROGRAM_ID
    )
  )[0];
};

export const getAssociatedAddress = async (
  mint: PublicKey,
  owner: PublicKey
) => {
  const tokenAddress = await anchor.utils.token.associatedAddress({
    mint: mint,
    owner: owner
  })
  return tokenAddress
}
