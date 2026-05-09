use ark_bn254::Bn254;
use ark_ff::PrimeField;
use ark_groth16::VerifyingKey;
use ark_r1cs_std::{fields::fp::FpVar, prelude::*};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_serialize::CanonicalSerialize;
use ark_std::marker::PhantomData;

#[derive(Clone)]
pub struct PolyCircuit<F: PrimeField> {
    pub x: Option<F>,
    pub y: Option<F>,
}

impl<F: PrimeField> ConstraintSynthesizer<F> for PolyCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> ark_relations::r1cs::Result<()> {
        let x_var = FpVar::new_witness(cs.clone(), || {
            self.x.ok_or(SynthesisError::AssignmentMissing)
        })?;

        let y_var = FpVar::new_input(cs.clone(), || {
            self.y.ok_or(SynthesisError::AssignmentMissing)
        })?;

        let x_squared = x_var.square()?;

        let three = FpVar::constant(F::from(3u32));

        let three_x = three * &x_var;

        let computed_y = x_squared + three_x;

        computed_y.enforce_equal(&y_var)?;

        Ok(())
    }
}

fn change_endianness(bytes: &[u8]) -> [u8; 32] {
    // let mut vec = Vec::new();
    assert_eq!(bytes.len(), 32);
    let mut array = [0; 32];
    // for b in bytes.chunks(32) {
        // for (i, byte) in bytes.iter().rev().enumerate() {
        //     array[i] = *byte;
        // }
    // }
    array.copy_from_slice(bytes);
    array.reverse();
    array
}
// vkの中身をonchainにハードコードするために出力する関数
fn print_vk_as_bytes(vk: &VerifyingKey<Bn254>) {

    let mut alpha_g1_bytes = Vec::new();
    // vk.serialize_compressed(&mut bytes).unwrap();

    vk.alpha_g1.serialize_uncompressed(&mut alpha_g1_bytes).unwrap();
    // 32 
    println!("length of bytes: {:?}", alpha_g1_bytes.len());
    println!("alpha_g1: {:?}", &alpha_g1_bytes);

    let mut beta_g2_bytes = Vec::new();
    vk.beta_g2.serialize_uncompressed(&mut beta_g2_bytes).unwrap();
    // 64
    println!("length of bytes: {:?}", beta_g2_bytes.len());
    println!("beta_g2: {:?}", &beta_g2_bytes);

    let mut gamma_g2_bytes = Vec::new();
    vk.gamma_g2.serialize_uncompressed(&mut gamma_g2_bytes).unwrap();
    // 64
    println!("length of bytes: {:?}", gamma_g2_bytes.len());
    println!("gamma_g2_bytes: {:?}", &gamma_g2_bytes);

    let mut delta_g2_bytes = Vec::new();
    vk.delta_g2.serialize_uncompressed(&mut delta_g2_bytes).unwrap();
    // 64
    println!("length of bytes: {:?}", delta_g2_bytes.len());
    println!("delta_g2: {:?}", &delta_g2_bytes);

    for (i, ic) in vk.gamma_abc_g1.iter().enumerate() {
        let mut ic_bytes = Vec::new();
        ic.serialize_uncompressed(&mut ic_bytes).unwrap();
        // 32
        println!("length of bytes: {:?}", ic_bytes.len());
        println!("ic: {:?}", &ic_bytes);
    }
    println!("nr_pubinputs: {}", vk.gamma_abc_g1.len() - 1);


}

#[cfg(test)]
mod test {
    use std::ops::Neg;

    use ark_bn254::{Bn254, Fr};
    use ark_ec::AffineRepr;
    use ark_groth16::Groth16;
    use ark_snark::SNARK;
    use ark_std::rand::thread_rng;
    use ark_ff::PrimeField;

    // serializeを使うために必要
    use ark_serialize::CanonicalSerialize;


    use crate::{PolyCircuit, change_endianness, print_vk_as_bytes};

    #[test]
    fn verify_test() {
        let mut rng = thread_rng();
        let x = Fr::from(5u32);
        let y = Fr::from(40u32);

        let circuit = PolyCircuit {
            x: Some(x),
            y: Some(y)
        };

        let (pk, vk) = Groth16::<Bn254>::circuit_specific_setup(circuit.clone(), &mut rng).expect("setup failed");

        // println!("{:?}", vk);
        print_vk_as_bytes(&vk);
        let proof = Groth16::<Bn254>::prove(&pk, circuit, &mut rng).expect("Proving failed");


        let mut proof_bytes = Vec::new();
        // A: 64バイト (Fp座標 x, y)
        // 反転させる　ex)mod7の世界なら、5 -> 2となるような数
        let proof_a_neged = proof.a.neg();

        // [u8]に変換
        let mut a_bytes = [0u8; 64];
        proof_a_neged.serialize_uncompressed(&mut a_bytes[..]).unwrap();

        // aのx座標をbig endianに変更する。
        let a_x_be = change_endianness(&a_bytes[0..32]);
        // aのy座標をbeに変更
        let a_y_be = change_endianness(&a_bytes[32..64]);

        // proof_bytesにまとめる。
        proof_bytes.extend_from_slice(&a_x_be);
        proof_bytes.extend_from_slice(&a_y_be);

        // B: 128バイト (Fp2座標 x0+x1i, y0+y1i)
        // 虚部と実部をバラバラにbeに変更する
        let mut b_bytes = [0u8; 128];

        // proof.b.serialize_uncompressed(&mut proof_bytes).unwrap();
        proof.b.serialize_compressed(&mut b_bytes[..]).unwrap();

        let b_x_be = change_endianness(&b_bytes[0..32]);
        let b_xi_be = change_endianness(&b_bytes[32..64]);
        let b_y_be = change_endianness(&b_bytes[64..96]);
        let b_yi_be = change_endianness(&b_bytes[96..128]);

        proof_bytes.extend_from_slice(&b_x_be);
        proof_bytes.extend_from_slice(&b_xi_be);
        proof_bytes.extend_from_slice(&b_y_be);
        proof_bytes.extend_from_slice(&b_yi_be);


        // C: 64バイト (Fp座標 x, y)
        let mut c_bytes = [0u8; 64];
        // proof.c.serialize_uncompressed(&mut proof_bytes).unwrap();
        proof.c.serialize_compressed(&mut c_bytes[..]).unwrap();

        let c_x_be = change_endianness(&c_bytes[0..32]);
        let c_y_be = change_endianness(&c_bytes[32..64]);

        proof_bytes.extend_from_slice(&c_x_be);
        proof_bytes.extend_from_slice(&c_y_be);
        
        println!("{:?}", proof_bytes.len());

        assert_eq!(256, proof_bytes.len());


        // let public_inputs = vec![y];

        let mut public_inputs_bytes = Vec::new();
        y.serialize_uncompressed(&mut public_inputs_bytes).unwrap();
        assert_eq!(32, public_inputs_bytes.len());

        // ここから下はon chainでの確認になる。
        // let valid = Groth16::<Bn254>::verify(&vk, &public_inputs, &proof).expect("Verification failed");

    }

    use solana_account::Account;
    use solana_sdk::{message::{AccountMeta, Instruction}, msg, pubkey::Pubkey};
    use mollusk_svm::{Mollusk, program::keyed_account_for_system_program};

    const ID: Pubkey = solana_sdk::pubkey!("22222222222222222222222222222222222222222222");
    #[test]
    fn mollusk() {
        let user = Pubkey::new_unique();
        let user_account = Account::default();
        let (system_program, system_program_account) = keyed_account_for_system_program();

        let addresses = vec![
            AccountMeta::new(user, true),
            AccountMeta::new_readonly(system_program, false),
        ];

        let loader = vec![
            (user, user_account),
            (system_program, system_program_account),
        ];

        let mut instruction_data = [0u8; 289];

        let mut mollusk = Mollusk::new(&ID, "target/deploy/pinocchio_with_arkworks");

        
        let res = mollusk.process_instruction(
            &Instruction::new_with_bytes(
                ID, 
                &instruction_data, 
                addresses
            ), 
            &loader
        );
        msg!("{:?}", res.program_result);

    }

}