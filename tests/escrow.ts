import * as anchor from "@anchor-lang/core";
import { Program } from "@anchor-lang/core";
import { Escrow } from "../target/types/escrow";
import NodeWallet from "@anchor-lang/core/dist/cjs/nodewallet";
import { Keypair, PublicKey } from "@solana/web3.js";
import BN from "bn.js";
import { randomBytes } from "crypto";
import { ASSOCIATED_TOKEN_PROGRAM_ID, createMint, getAssociatedTokenAddressSync, getOrCreateAssociatedTokenAccount, mintTo, TOKEN_PROGRAM_ID} from "@solana/spl-token";
import { SYSTEM_PROGRAM_ID } from "@anchor-lang/core/dist/cjs/native/system";
import { assert } from "console";
import { expect } from "chai";

const commitment = "confirmed";

describe("escrow", () => {
    const confirmTx = async (tx: string) => {
    const connection = anchor.getProvider().connection;
    const latestBlockHash = await connection.getLatestBlockhash();
    await connection.confirmTransaction(
      {
        signature: tx,
        ...latestBlockHash,
      },
      commitment
    );
  };
  const confirmTxs = async (signatures: string[]) => {
    await Promise.all(signatures.map(confirmTx));
  };
  const getTokenBalance = async (account: PublicKey) => {
    const info = await connection.getAccountInfo(account);
    if (!info) return 0;
    const balance = await connection.getTokenAccountBalance(account);
    return balance.value.uiAmount ?? 0;
  };
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
 
  const program = anchor.workspace.Escrow as Program<Escrow>;
  const connection = anchor.getProvider().connection;

  const payer = provider.wallet as NodeWallet;

  const taker = Keypair.generate();

  let mintA: PublicKey;
  let mintB: PublicKey;
  let userAtaA: PublicKey;
  let takerAtaA: PublicKey;
  
  let takerAtaB: PublicKey;
  let makerAtaB: PublicKey;
  let userAtaB: PublicKey;

  let vault: PublicKey;
  const seed = new BN(randomBytes(8));
  const escrow = PublicKey.findProgramAddressSync(
    [Buffer.from("escrow"), payer.publicKey.toBuffer() , seed.toBuffer('le', 8)], program.programId
  )[0];
  it("Requesting Airdrop", async () => {

    await Promise.all([ payer , taker].map(async (k) => {
        return await anchor.getProvider().connection.requestAirdrop(k.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
      })
    ).then(confirmTxs);
    
  });

  it("Mint Tokens", async () => {
    mintA = await createMint(
      connection,
      payer.payer,
      provider.publicKey,
      provider.publicKey,
      6
    );
    console.log("Mint A:", mintA.toBase58());
    mintB = await createMint(
      connection,
      payer.payer,
      provider.publicKey,
      provider.publicKey,
      6
    );
    console.log("Mint B:", mintB.toBase58());

    vault = getAssociatedTokenAddressSync(mintA, escrow, true);
    userAtaA = (await getOrCreateAssociatedTokenAccount(
      connection,
      payer.payer,
      mintA,
      payer.publicKey
    )).address;
   
    takerAtaA = (await getOrCreateAssociatedTokenAccount(
      connection,
      payer.payer,
      mintA,
      taker.publicKey
    )).address;
    
    userAtaB = (await getOrCreateAssociatedTokenAccount(
      connection,
      payer.payer,
      mintB,
      payer.publicKey
    )).address;
    
    takerAtaB = (await getOrCreateAssociatedTokenAccount(
      connection,
      payer.payer,
      mintB,
      taker.publicKey
    )).address;
    
   await mintTo(
      connection,
      payer.payer,
      mintA,
      userAtaA,
      provider.publicKey,
      1000 * 10 ** 6
    );
    console.log("Minted 1000 tokens to maker's ATA for mint A", userAtaA.toBase58());
    await mintTo(
      connection,
      payer.payer,
      mintB,
      takerAtaB,
      provider.publicKey,
      1000 * 10 ** 6
   )
   console.log("Minted 1000 tokens to taker for mint B", takerAtaB.toBase58());
  });
  it("Initialize Escrow", async () => {
    const initialVaultBalance = await getTokenBalance(vault);
    const initialUserAtaABalance = await getTokenBalance(userAtaA);
    console.log("Initial Vault Balance:", initialVaultBalance);
    console.log("Initial User Token A Balance:", initialUserAtaABalance);
    const receive = new BN(100 * 10 ** 6);
    const deposit = new BN(10 * 10 ** 6);
    const tx = await program.methods.initialize(seed, receive, deposit)
      .accountsStrict({
        user: payer.publicKey,
        minta: mintA,
        mintb: mintB,
        vault,
        escrow,
        userAtaA: userAtaA,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SYSTEM_PROGRAM_ID  
      })
      .rpc();
      await confirmTx(tx);
      console.log("Escrow initialized with tx:", tx);
        const postInitVaultBalance = await getTokenBalance(vault);
        const postInitUserAtaABalance = await getTokenBalance(userAtaA);
        console.log("Post-Initialization Vault Balance:", postInitVaultBalance);
        console.log("Post-Initialization User Token A Balance:", postInitUserAtaABalance);
  });

  it("Take from vault", async () => {
    const initialVaultBalance = await getTokenBalance(vault);
    const initialTakerAtaBBalance = await getTokenBalance(takerAtaB);
    console.log("Initial Vault Balance:", initialVaultBalance);
    console.log("Initial Taker ATA B Balance:", initialTakerAtaBBalance);
    const tx = await program.methods.take()
      .accountsStrict({
        taker: taker.publicKey,
        user: payer.publicKey,
        minta: mintA,
        mintb: mintB,
        vault,
        escrow,
        takerAtaA: takerAtaA,
        takerAtaB: takerAtaB,
        userAtaB: userAtaB,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SYSTEM_PROGRAM_ID  
      })
      .signers([taker])
      .rpc();
      await confirmTx(tx);
      console.log("Escrow take tx:", tx);
      const postTakeVaultBalance = await getTokenBalance(vault);
      const postTakeTakerAtaBBalance = await getTokenBalance(takerAtaB);
      const postTakeUserAtaBBalance = await getTokenBalance(userAtaB);
      const postTakeTakerAtaABalance = await getTokenBalance(takerAtaA);
      console.log("Post-Take Vault Balance:", postTakeVaultBalance);
      console.log("Post-Take Taker Token B Balance:", postTakeTakerAtaBBalance);
      console.log("Post-Take User Token B Balance:", postTakeUserAtaBBalance);
      console.log("Post-Take Taker Token A Balance:", postTakeTakerAtaABalance);
      const escrowInfo = await provider.connection.getAccountInfo(escrow);
      expect(escrowInfo).to.be.null;
      const vaultInfo = await provider.connection.getAccountInfo(vault);
      expect(vaultInfo).to.be.null;
  });
});
