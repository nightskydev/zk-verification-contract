import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { ZkCeoVerification } from "../target/types/zk_ceo_verification";
import { assert } from "chai";

describe("zk_ceo_verification", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.ZkCeoVerification as Program<ZkCeoVerification>;

  it("Initialize Company", async () => {
    // Generate a new keypair for the company account
    const company = anchor.web3.Keypair.generate();

    // Generate a new keypair for the user
    const user = provider.wallet;

    // Define the registration number and CEO public key
    const registrationNumber = Array.from(new Uint8Array(32).fill(1));
    const ceoPublicKey = anchor.web3.Keypair.generate().publicKey;

    // Call the initialize_company function
    await program.methods
      .initializeCompany(registrationNumber, ceoPublicKey)
      .accounts({
        company: company.publicKey,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([company])
      .rpc();

    // Fetch the company account
    const companyAccount = await program.account.company.fetch(company.publicKey);

    // Assert the company account fields
    assert.deepEqual(companyAccount.registrationNumber, registrationNumber);
    assert.deepEqual(companyAccount.ceoPublicKey, ceoPublicKey);
    assert.isFalse(companyAccount.verified);
  });

  it("Verify CEO", async () => {
    // Generate a new keypair for the company account
    const company = anchor.web3.Keypair.generate();

    // Generate a new keypair for the user
    const user = provider.wallet;

    // Define the registration number and CEO public key
    const registrationNumber = Array.from(new Uint8Array(32).fill(1));
    const ceoPublicKey = anchor.web3.Keypair.generate().publicKey;

    // Initialize the company
    await program.methods
      .initializeCompany(registrationNumber, ceoPublicKey)
      .accounts({
        company: company.publicKey,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([company])
      .rpc();

    // Define the CEO proof and verifying key
    const proof = {
      proofA: Array.from(new Uint8Array(64).fill(0)),
      proofB: Array.from(new Uint8Array(128).fill(0)),
      proofC: Array.from(new Uint8Array(64).fill(0)),
      publicSignals: Array.from(new Uint8Array(32).fill(0)),
    };
    const verifyingKey = new Uint8Array(128).fill(0);

    // Call the verify_ceo function
    await program.methods
      .verifyCeo(proof, verifyingKey)
      .accounts({
        company: company.publicKey,
        user: user.publicKey,
      })
      .rpc();

    // Fetch the company account
    const companyAccount = await program.account.company.fetch(company.publicKey);

    // Assert the company account fields
    assert.isTrue(companyAccount.verified);
  });
});