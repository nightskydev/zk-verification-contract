use anchor_lang::prelude::*;
use ark_ff::bytes::{FromBytes, ToBytes};
use groth16_solana::groth16::{Groth16Verifier, Groth16Verifyingkey};
use std::ops::Neg;
pub mod errors;
use errors::CustomError;

declare_id!("tku5mcPJXMt9mxLChczNSsGBtUn3KLBhafHSVX8iZVh");

pub const VERIFYINGKEY: Groth16Verifyingkey = Groth16Verifyingkey {
    nr_pubinputs: 5,

    vk_alpha_g1: [
        45, 77, 154, 167, 227, 2, 217, 223, 65, 116, 157, 85, 7, 148, 157, 5, 219, 234, 51, 251,
        177, 108, 100, 59, 34, 245, 153, 162, 190, 109, 242, 226, 20, 190, 221, 80, 60, 55, 206,
        176, 97, 216, 236, 96, 32, 159, 227, 69, 206, 137, 131, 10, 25, 35, 3, 1, 240, 118, 202,
        255, 0, 77, 25, 38,
    ],

    vk_beta_g2: [
        9, 103, 3, 47, 203, 247, 118, 209, 175, 201, 133, 248, 136, 119, 241, 130, 211, 132, 128,
        166, 83, 242, 222, 202, 169, 121, 76, 188, 59, 243, 6, 12, 14, 24, 120, 71, 173, 76, 121,
        131, 116, 208, 214, 115, 43, 245, 1, 132, 125, 214, 139, 192, 224, 113, 36, 30, 2, 19, 188,
        127, 193, 61, 183, 171, 48, 76, 251, 209, 224, 138, 112, 74, 153, 245, 232, 71, 217, 63,
        140, 60, 170, 253, 222, 196, 107, 122, 13, 55, 157, 166, 154, 77, 17, 35, 70, 167, 23, 57,
        193, 177, 164, 87, 168, 199, 49, 49, 35, 210, 77, 47, 145, 146, 248, 150, 183, 198, 62,
        234, 5, 169, 213, 127, 6, 84, 122, 208, 206, 200,
    ],

    vk_gamme_g2: [
        25, 142, 147, 147, 146, 13, 72, 58, 114, 96, 191, 183, 49, 251, 93, 37, 241, 170, 73, 51,
        53, 169, 231, 18, 151, 228, 133, 183, 174, 243, 18, 194, 24, 0, 222, 239, 18, 31, 30, 118,
        66, 106, 0, 102, 94, 92, 68, 121, 103, 67, 34, 212, 247, 94, 218, 221, 70, 222, 189, 92,
        217, 146, 246, 237, 9, 6, 137, 208, 88, 95, 240, 117, 236, 158, 153, 173, 105, 12, 51, 149,
        188, 75, 49, 51, 112, 179, 142, 243, 85, 172, 218, 220, 209, 34, 151, 91, 18, 200, 94, 165,
        219, 140, 109, 235, 74, 171, 113, 128, 141, 203, 64, 143, 227, 209, 231, 105, 12, 67, 211,
        123, 76, 230, 204, 1, 102, 250, 125, 170,
    ],

    vk_delta_g2: [
        26, 203, 92, 171, 171, 11, 181, 174, 149, 27, 39, 138, 152, 201, 64, 68, 58, 196, 226, 111,
        72, 25, 156, 65, 19, 43, 74, 205, 177, 178, 156, 241, 17, 235, 49, 68, 78, 118, 42, 13,
        135, 183, 115, 24, 101, 138, 116, 81, 201, 49, 190, 110, 10, 164, 201, 43, 227, 89, 11, 63,
        156, 104, 89, 186, 20, 96, 77, 92, 160, 19, 241, 168, 117, 110, 49, 74, 45, 234, 200, 213,
        103, 225, 7, 187, 170, 202, 177, 87, 160, 42, 42, 222, 182, 46, 30, 7, 11, 142, 96, 181,
        189, 233, 138, 104, 156, 66, 245, 243, 124, 122, 70, 187, 199, 205, 4, 102, 176, 252, 195,
        61, 249, 164, 52, 78, 69, 26, 170, 163,
    ],

    vk_ic: &[
        [
            29, 52, 223, 78, 66, 81, 17, 132, 16, 253, 160, 136, 132, 205, 74, 47, 234, 252, 58,
            222, 70, 68, 191, 236, 109, 241, 215, 220, 39, 2, 22, 100, 18, 77, 141, 202, 140, 99,
            53, 210, 231, 232, 153, 63, 25, 146, 90, 64, 68, 22, 73, 128, 30, 37, 122, 200, 183,
            63, 46, 220, 175, 137, 198, 51,
        ],
        [
            11, 61, 40, 100, 209, 193, 208, 163, 237, 68, 97, 109, 214, 61, 239, 253, 229, 249,
            122, 207, 18, 246, 153, 129, 76, 66, 221, 186, 222, 187, 249, 254, 16, 76, 166, 239,
            124, 228, 210, 215, 151, 75, 229, 119, 246, 45, 158, 209, 250, 240, 152, 8, 190, 124,
            145, 87, 117, 252, 241, 40, 147, 198, 123, 209,
        ],
        [
            23, 233, 167, 177, 126, 76, 252, 36, 96, 67, 11, 230, 186, 188, 24, 146, 183, 92, 170,
            217, 179, 242, 162, 212, 84, 253, 212, 56, 226, 72, 0, 170, 10, 19, 81, 194, 19, 18,
            250, 36, 115, 206, 230, 7, 113, 125, 19, 198, 222, 143, 115, 225, 241, 199, 230, 255,
            9, 80, 210, 17, 172, 79, 160, 26,
        ],
        [
            44, 74, 243, 213, 98, 27, 195, 245, 79, 121, 177, 142, 84, 70, 119, 136, 149, 229, 182,
            87, 143, 234, 174, 69, 55, 62, 212, 216, 50, 237, 210, 4, 33, 115, 36, 103, 103, 19,
            122, 90, 79, 246, 245, 21, 255, 47, 26, 125, 230, 179, 53, 234, 209, 9, 212, 186, 71,
            219, 64, 255, 175, 214, 248, 36,
        ],
        [
            26, 60, 19, 146, 129, 254, 183, 48, 42, 253, 232, 118, 65, 108, 24, 97, 162, 62, 88,
            97, 148, 21, 106, 5, 12, 222, 86, 111, 239, 58, 125, 202, 38, 25, 70, 112, 127, 197,
            76, 239, 184, 85, 206, 241, 45, 7, 249, 240, 193, 197, 130, 30, 161, 107, 152, 36, 31,
            76, 239, 74, 219, 221, 141, 218,
        ],
    ],
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

    pub fn verify_ceo(ctx: Context<VerifyCEO>, proof: CEOProof) -> Result<()> {
        let company = &mut ctx.accounts.company;
        require!(company.verify_ceo(proof), CustomError::VerificationFailed);
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
    public_signals: [u8; 128], // 3 public signals, each 256 bytes
}

impl Company {
    pub fn verify_ceo(&self, proof: CEOProof) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::solana_program::pubkey::Pubkey;

    #[test]
    fn test_initialize_company() {
        let mut company = Company {
            registration_number: [0u8; 32],
            ceo_public_key: Pubkey::default(),
            verified: false,
        };

        let registration_number = [1u8; 32];
        let ceo_public_key = Pubkey::new_unique();

        // Simulate the initialization
        company.registration_number = registration_number;
        company.ceo_public_key = ceo_public_key;
        company.verified = false;

        // Assertions
        assert_eq!(company.registration_number, registration_number);
        assert_eq!(company.ceo_public_key, ceo_public_key);
        assert!(!company.verified);
    }

    #[test]
    fn test_verify_ceo() {
        let company = Company {
            registration_number: [1u8; 32],
            ceo_public_key: Pubkey::new_unique(),
            verified: false,
        };

        // Mock CEOProof
        let proof = CEOProof {
            proof_a: [
                28, 28, 62, 132, 84, 160, 127, 110, 250, 247, 79, 162, 132, 239, 135, 52, 62, 15,
                214, 204, 201, 66, 13, 229, 144, 126, 218, 141, 156, 88, 248, 240, 27, 240, 32,
                177, 187, 45, 5, 27, 235, 255, 74, 92, 146, 147, 192, 245, 219, 24, 34, 172, 226,
                48, 85, 38, 4, 130, 148, 129, 65, 63, 11, 37,
            ],
            proof_b: [
                4, 159, 19, 6, 51, 222, 15, 219, 17, 194, 235, 201, 18, 68, 114, 105, 100, 208,
                164, 211, 135, 223, 232, 70, 119, 21, 144, 90, 221, 181, 139, 131, 15, 51, 150, 88,
                76, 165, 179, 114, 59, 97, 246, 8, 105, 73, 136, 173, 195, 146, 46, 2, 247, 207,
                103, 160, 169, 81, 238, 18, 137, 95, 54, 110, 3, 200, 173, 231, 218, 205, 34, 220,
                147, 73, 172, 58, 107, 85, 149, 140, 27, 70, 16, 145, 36, 116, 202, 139, 56, 254,
                245, 254, 134, 73, 213, 186, 25, 161, 134, 162, 77, 37, 212, 226, 180, 149, 185,
                132, 168, 136, 178, 113, 10, 191, 178, 168, 197, 76, 89, 191, 193, 166, 152, 67,
                94, 237, 19, 229,
            ],
            proof_c: [
                38, 62, 10, 176, 161, 77, 67, 248, 47, 73, 71, 155, 41, 161, 226, 165, 171, 33,
                232, 181, 64, 48, 125, 46, 174, 126, 201, 161, 194, 185, 29, 111, 44, 40, 182, 245,
                146, 166, 140, 56, 31, 63, 6, 82, 211, 183, 177, 98, 81, 24, 125, 210, 162, 90,
                160, 134, 6, 73, 240, 53, 123, 254, 85, 49,
            ],
            public_signals: [
                9, 74, 227, 59, 103, 168, 69, 153, 138, 187, 85, 233, 23, 100, 45, 64, 34, 208,
                120, 217, 111, 124, 54, 234, 17, 218, 66, 115, 236, 242, 15, 80, 9, 74, 227, 59,
                103, 168, 69, 153, 138, 187, 85, 233, 23, 100, 45, 64, 34, 208, 120, 217, 111, 124,
                54, 234, 17, 218, 66, 115, 236, 242, 15, 80, 9, 74, 227, 59, 103, 168, 69, 153,
                138, 187, 85, 233, 23, 100, 45, 64, 34, 208, 120, 217, 111, 124, 54, 234, 17, 218,
                66, 115, 236, 242, 15, 80, 9, 74, 227, 59, 103, 168, 69, 153, 138, 187, 85, 233,
                23, 100, 45, 64, 34, 208, 120, 217, 111, 124, 54, 234, 17, 218, 66, 115, 236, 242,
                15, 80,
            ],
        };

        // Simulate verification
        let result = company.verify_ceo(proof);

        // Assertions
        assert!(result, "CEO verification failed");
    }
}
