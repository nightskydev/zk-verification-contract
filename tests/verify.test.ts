import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import NodeWallet from "@project-serum/anchor/dist/cjs/nodewallet";
import {
  PublicKey,
  SystemProgram,
  Transaction,
  Connection,
  Commitment,
} from "@solana/web3.js";
import { assert } from "chai";
import * as snarkjs from "snarkjs";
import path from "path";
import { buildBn128, utils } from "ffjavascript";
import { parseProofToBytesArray, parseToBytesArray } from "../src/utils";
const { unstringifyBigInts } = utils;

import { IDL } from "./zk_ceo_verification";

const wasmPath = "tests/bday.wasm";
const zkeyPath = "tests/bday_0001.zkey";

function g1Uncompressed(curve, p1Raw) {
  let p1 = curve.G1.fromObject(p1Raw);

  let buff = new Uint8Array(64); // 64 bytes for G1 uncompressed
  curve.G1.toRprUncompressed(buff, 0, p1);

  return Buffer.from(buff);
}

function g2Uncompressed(curve, p2Raw) {
  let p2 = curve.G2.fromObject(p2Raw);

  let buff = new Uint8Array(128); // 128 bytes for G2 uncompressed
  curve.G2.toRprUncompressed(buff, 0, p2);

  return Buffer.from(buff);
}

function to32ByteBuffer(bigInt) {
  const hexString = bigInt.toString(16).padStart(64, "0"); // Pad to 64 hex characters (32 bytes)
  const buffer = Buffer.from(hexString, "hex");
  return buffer;
}

describe("zk_ceo_verification", () => {
  // Configure the client to use the local cluster.
  const commitment: Commitment = "confirmed";
  const connection = new Connection("https://api.devnet.solana.com", {
    commitment,
    // wsEndpoint: "wss://api.devnet.solana.com/",
  });
  const options = anchor.AnchorProvider.defaultOptions();
  const wallet = NodeWallet.local();
  const provider = new anchor.AnchorProvider(connection, wallet, options);

  anchor.setProvider(provider);

  // CAUTTION: if you are intended to use the program that is deployed by yourself,
  // please make sure that the programIDs are consistent
  const programId = new PublicKey(
    "tku5mcPJXMt9mxLChczNSsGBtUn3KLBhafHSVX8iZVh"
  );
  const program = new anchor.Program(IDL, programId, provider);

  // it("Test simple contract", async () => {
  //   const wasmPath = "tests/multiplier.wasm";
  //   const zkeyPath = "tests/multiplier_0001.zkey";
  //   let input = { a: 4, b: 11 };
  //   let { proof, publicSignals } = await snarkjs.groth16.fullProve(
  //     input,
  //     wasmPath,
  //     zkeyPath
  //   );
  //   const parsed_proof = parseProofToBytesArray(JSON.stringify(proof));
  //   console.log({ parsed_proof, publicSignals });

  //   let curve = await buildBn128();
  //   let proofProc = unstringifyBigInts(proof);

  //   let pi_a = g1Uncompressed(curve, proofProc.pi_a);
  //   let pi_a_0_u8_array = Array.from(pi_a);
  //   console.log(pi_a_0_u8_array);

  //   const pi_b = g2Uncompressed(curve, proofProc.pi_b);
  //   let pi_b_0_u8_array = Array.from(pi_b);
  //   console.log(pi_b_0_u8_array.slice(0, 64));
  //   console.log(pi_b_0_u8_array.slice(64, 128));

  //   const pi_c = g1Uncompressed(curve, proofProc.pi_c);
  //   let pi_c_0_u8_array = Array.from(pi_c);
  //   console.log(pi_c_0_u8_array);

  //   // Assuming publicSignals has only one element
  //   const publicSignalsBuffer = to32ByteBuffer(BigInt(publicSignals[0]));
  //   let public_signal_0_u8_array = Array.from(publicSignalsBuffer);
  //   console.log(public_signal_0_u8_array);
  // });

  it("Verify CEO", async () => {
    // let input = {
    //   name: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    //   role: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    //   birthday: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    //   companyName: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    // };
    // let { proof, publicSignals } = await snarkjs.groth16.fullProve(
    //   input,
    //   wasmPath,
    //   zkeyPath
    // );

    // const parsed_proof = parseProofToBytesArray(JSON.stringify(proof));
    // const parsed_public_signal = parseToBytesArray(publicSignals);

    // let final_public_signal = [];
    // for (let i = 0; i < parsed_public_signal.length; i++) {
    //   final_public_signal = final_public_signal.concat(parsed_public_signal[i]);
    // }
    // console.log(
    //   JSON.stringify(parsed_proof),
    //   JSON.stringify(final_public_signal)
    // );

    // let curve = await buildBn128();
    // let proofProc = unstringifyBigInts(proof);
    // // publicSignals = unstringifyBigInts(publicSignals);

    // let pi_a = g1Uncompressed(curve, proofProc.pi_a);
    // let pi_a_0_u8_array = Array.from(pi_a);
    // console.log(pi_a_0_u8_array);

    // const pi_b = g2Uncompressed(curve, proofProc.pi_b);
    // let pi_b_0_u8_array = Array.from(pi_b);
    // console.log(pi_b_0_u8_array.slice(0, 64));
    // console.log(pi_b_0_u8_array.slice(64, 128));

    // const pi_c = g1Uncompressed(curve, proofProc.pi_c);
    // let pi_c_0_u8_array = Array.from(pi_c);
    // console.log(pi_c_0_u8_array);

    // // Assuming publicSignals has only one element
    // const publicSignalsBuffer = to32ByteBuffer(BigInt(publicSignals[0]));
    // let public_signal_0_u8_array = Array.from(publicSignalsBuffer);
    // console.log(public_signal_0_u8_array);

    const test_proof = {
      proofA: [
        28, 28, 62, 132, 84, 160, 127, 110, 250, 247, 79, 162, 132, 239, 135,
        52, 62, 15, 214, 204, 201, 66, 13, 229, 144, 126, 218, 141, 156, 88,
        248, 240, 27, 240, 32, 177, 187, 45, 5, 27, 235, 255, 74, 92, 146, 147,
        192, 245, 219, 24, 34, 172, 226, 48, 85, 38, 4, 130, 148, 129, 65, 63,
        11, 37,
      ],
      proofB: [
        4, 159, 19, 6, 51, 222, 15, 219, 17, 194, 235, 201, 18, 68, 114, 105,
        100, 208, 164, 211, 135, 223, 232, 70, 119, 21, 144, 90, 221, 181, 139,
        131, 15, 51, 150, 88, 76, 165, 179, 114, 59, 97, 246, 8, 105, 73, 136,
        173, 195, 146, 46, 2, 247, 207, 103, 160, 169, 81, 238, 18, 137, 95, 54,
        110, 3, 200, 173, 231, 218, 205, 34, 220, 147, 73, 172, 58, 107, 85,
        149, 140, 27, 70, 16, 145, 36, 116, 202, 139, 56, 254, 245, 254, 134,
        73, 213, 186, 25, 161, 134, 162, 77, 37, 212, 226, 180, 149, 185, 132,
        168, 136, 178, 113, 10, 191, 178, 168, 197, 76, 89, 191, 193, 166, 152,
        67, 94, 237, 19, 229,
      ],
      proofC: [
        38, 62, 10, 176, 161, 77, 67, 248, 47, 73, 71, 155, 41, 161, 226, 165,
        171, 33, 232, 181, 64, 48, 125, 46, 174, 126, 201, 161, 194, 185, 29,
        111, 44, 40, 182, 245, 146, 166, 140, 56, 31, 63, 6, 82, 211, 183, 177,
        98, 81, 24, 125, 210, 162, 90, 160, 134, 6, 73, 240, 53, 123, 254, 85,
        49,
      ],
    };

    const test_public_signal = [
      9, 74, 227, 59, 103, 168, 69, 153, 138, 187, 85, 233, 23, 100, 45, 64, 34,
      208, 120, 217, 111, 124, 54, 234, 17, 218, 66, 115, 236, 242, 15, 80, 9,
      74, 227, 59, 103, 168, 69, 153, 138, 187, 85, 233, 23, 100, 45, 64, 34,
      208, 120, 217, 111, 124, 54, 234, 17, 218, 66, 115, 236, 242, 15, 80, 9,
      74, 227, 59, 103, 168, 69, 153, 138, 187, 85, 233, 23, 100, 45, 64, 34,
      208, 120, 217, 111, 124, 54, 234, 17, 218, 66, 115, 236, 242, 15, 80, 9,
      74, 227, 59, 103, 168, 69, 153, 138, 187, 85, 233, 23, 100, 45, 64, 34,
      208, 120, 217, 111, 124, 54, 234, 17, 218, 66, 115, 236, 242, 15, 80,
    ];

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
    const proofInfo = {
      proofA: test_proof.proofA,
      proofB: test_proof.proofB,
      proofC: test_proof.proofC,
      publicSignals: test_public_signal,
    };

    // Call the verify_ceo function
    await program.methods
      .verifyCeo(proofInfo)
      .accounts({
        company: company.publicKey,
        user: user.publicKey,
      })
      .rpc();

    // Fetch the company account
    const companyAccount = await program.account.company.fetch(
      company.publicKey
    );

    // Assert the company account fields
    assert.isTrue(companyAccount.verified);
  });
});
