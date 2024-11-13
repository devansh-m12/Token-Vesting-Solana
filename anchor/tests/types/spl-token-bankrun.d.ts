declare module 'spl-token-bankrun' {
  import { PublicKey, Keypair } from "@solana/web3.js";
  import { BanksClient } from "solana-bankrun";

  export function createMint(
    banksClient: BanksClient,
    payer: Keypair,
    mintAuthority: PublicKey,
    freezeAuthority: PublicKey | null,
    decimals: number
  ): Promise<PublicKey>;

  export function mintTo(
    banksClient: BanksClient,
    payer: Keypair,
    mint: PublicKey,
    destination: PublicKey,
    authority: Keypair,
    amount: number
  ): Promise<string>;
} 