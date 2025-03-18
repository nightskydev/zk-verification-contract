use anchor_lang::prelude::*;
use ark_ff::bytes::{FromBytes, ToBytes};
use groth16_solana::groth16::{Groth16Verifier, Groth16Verifyingkey};
use std::ops::Neg;
pub mod errors;
use errors::CustomError; 

declare_id!("FHUi6sy6jK1dQfZVBnkqsmLoomWAb1METmRwpCx4sbS1");

pub const VERIFYINGKEY: Groth16Verifyingkey =  Groth16Verifyingkey {
	nr_pubinputs: 1,

	vk_alpha_g1: [
		45,77,154,167,227,2,217,223,65,116,157,85,7,148,157,5,219,234,51,251,177,108,100,59,34,245,153,162,190,109,242,226,
		20,190,221,80,60,55,206,176,97,216,236,96,32,159,227,69,206,137,131,10,25,35,3,1,240,118,202,255,0,77,25,38,
	],

	vk_beta_g2: [
		9,103,3,47,203,247,118,209,175,201,133,248,136,119,241,130,211,132,128,166,83,242,222,202,169,121,76,188,59,243,6,12,
		14,24,120,71,173,76,121,131,116,208,214,115,43,245,1,132,125,214,139,192,224,113,36,30,2,19,188,127,193,61,183,171,
		48,76,251,209,224,138,112,74,153,245,232,71,217,63,140,60,170,253,222,196,107,122,13,55,157,166,154,77,17,35,70,167,
		23,57,193,177,164,87,168,199,49,49,35,210,77,47,145,146,248,150,183,198,62,234,5,169,213,127,6,84,122,208,206,200,
	],

	vk_gamme_g2: [
		25,142,147,147,146,13,72,58,114,96,191,183,49,251,93,37,241,170,73,51,53,169,231,18,151,228,133,183,174,243,18,194,
		24,0,222,239,18,31,30,118,66,106,0,102,94,92,68,121,103,67,34,212,247,94,218,221,70,222,189,92,217,146,246,237,
		9,6,137,208,88,95,240,117,236,158,153,173,105,12,51,149,188,75,49,51,112,179,142,243,85,172,218,220,209,34,151,91,
		18,200,94,165,219,140,109,235,74,171,113,128,141,203,64,143,227,209,231,105,12,67,211,123,76,230,204,1,102,250,125,170,
	],

	vk_delta_g2: [
		5,98,127,48,71,155,185,134,216,234,34,60,191,233,83,152,130,3,247,105,170,119,68,170,88,118,206,222,243,213,91,5,
		8,22,155,5,153,76,226,230,158,148,255,103,179,139,220,65,223,148,166,183,220,138,165,251,155,17,32,120,126,71,205,228,
		12,79,181,151,91,5,46,76,146,106,65,244,51,18,250,125,114,248,13,87,182,26,214,148,130,34,227,248,62,139,40,122,
		13,226,76,61,72,3,106,46,111,80,146,159,14,207,35,211,243,64,68,71,27,167,173,47,112,246,212,233,99,19,188,2,
	],

	vk_ic: &[
		[
			17,76,64,176,181,229,186,49,136,92,222,169,134,224,113,231,113,67,142,147,245,154,161,109,76,120,21,4,205,176,89,211,
			37,182,42,152,7,102,59,157,93,216,127,177,49,158,96,165,47,105,243,91,178,252,143,98,110,185,91,66,115,38,233,145,
		],
	]
};
#[program]
pub mod zk_ceo_verification {
    use super::*;

    pub fn initialize_company(
        ctx: Context<InitializeCompany>,
        registration_number: [u8; 32],
        ceo_public_key: Pubkey,
    ) -> Result<()> {
        let company = &mut ctx.accounts.company;
        company.registration_number = registration_number;
        company.ceo_public_key = ceo_public_key;
        company.verified = false;
        Ok(())
    }

    pub fn verify_ceo(
        ctx: Context<VerifyCEO>,
        proof: CEOProof,
    ) -> Result<()> {
        let company = &mut ctx.accounts.company;
        require!(
            company.verify_ceo(proof),
            CustomError::VerificationFailed
        );
        company.verified = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCompany<'info> {
    #[account(init, payer = user, space = 8 + 32 + 32 + 1)]
    pub company: Account<'info, Company>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VerifyCEO<'info> {
    #[account(mut)]
    pub company: Account<'info, Company>,
    pub user: Signer<'info>,
}

#[account]
pub struct Company {
    registration_number: [u8; 32],
    ceo_public_key: Pubkey,
    verified: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub struct CEOProof {
    proof_a: [u8; 64],
    proof_b: [u8; 128],
    proof_c: [u8; 64],
    public_signals: [u8; 32],
}

impl Company {
    pub fn verify_ceo(
        &self,
        proof: CEOProof,
    ) -> bool {
        type G1 = ark_ec::short_weierstrass_jacobian::GroupAffine<ark_bn254::g1::Parameters>;

        fn change_endianness(bytes: &[u8]) -> Vec<u8> {
            let mut vec = Vec::new();
            for b in bytes.chunks(32) {
                for byte in b.iter().rev() {
                    vec.push(*byte);
                }
            }
            vec
        }

        let proof_a_neg_g1: G1 = <G1 as FromBytes>::read(
            &*[&change_endianness(proof.proof_a.as_slice())[..], &[0u8][..]].concat(),
        )
        .unwrap();
        let mut proof_a_neg = [0u8; 65];
        <G1 as ToBytes>::write(&proof_a_neg_g1.neg(), &mut proof_a_neg[..]).unwrap();
        let proof_a_neg = change_endianness(&proof_a_neg[..64]).try_into().unwrap();

        let mut public_inputs_vec = Vec::new();
        for input in proof.public_signals.chunks(32) {
            public_inputs_vec.push(input);
        }

        // Convert the byte slice into a Groth16Verifyingkey
        let mut verifier = Groth16Verifier::new(
            &proof_a_neg,
            &proof.proof_b,
            &proof.proof_c,
            &public_inputs_vec.as_slice(),
            &VERIFYINGKEY,
        )
        .unwrap();
        verifier.verify().unwrap()
    }
}