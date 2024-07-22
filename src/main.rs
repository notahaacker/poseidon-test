// use light_poseidon::{Poseidon, PoseidonBytesHasher, parameters::bn254_x5};
// use ark_bn254::Fr;
// use ark_ff::{BigInteger, PrimeField};

// use ethnum::U256;

// fn split_u256_to_u8_32(val: U256) -> [u8; 32] {
//     let mut result = [0u8; 32];
//     let bytes = val.to_be_bytes();
//     result.copy_from_slice(&bytes);
//     result
// }

// // [56, 57, 56, 60, 60, 63, 64, 63, 60, 66, 60, 65]*

// fn main() {
//     let mut poseidon = Poseidon::<Fr>::new_circom(1).unwrap();
//     // let val = U256::from_str_radix("08b209cf1122a4e87693e9224895572da2666197ba09269591ce3e4f73b99dd7", 16).unwrap();
//     let val = U256::from_str_radix("0", 16).unwrap();
//     let array1 = split_u256_to_u8_32(val);
//     println!("{:?}", array1);
//     // let array2: [u8; 32] = [1; 32];
//     // let array3: [u8; 32] = [2; 32];
//     // let array4: [u8; 32] = [3; 32];

//     // let hash1 = poseidon.hash_bytes_be(&[&array1, &array2]).unwrap();
//     // let hash2 = poseidon.hash_bytes_be(&[&array3, &array4]).unwrap();
//     // let hash = poseidon.hash_bytes_be(&[&hash1, &hash2]).unwrap();
//     let hash = poseidon.hash_bytes_be(&[&array1]).unwrap();
//     let hex_output: String = hash.iter().map(|byte| format!("{:02x}", byte)).collect();
    
//     println!("{}", hex_output); // Print the hex string
//     // println!("{:x}", hash);
//     //  256d6fc2880684cf6585f26e1a1d52f7bcbad218ff9d2a1604f4fd2c8fb53da
//     // 0256d6fc2880684cf6585f26e1a1d52f7bcbad218ff9d2a1604f4fd2c8fb53da
// }

// use halo2_ecc::bn254::pairing::PairingChip;


// use halo2_ecc::bn254::{Fp12Chip, Fp2Chip, FpChip};
// use halo2_ecc::ecc::EccChip;
// use halo2_ecc::fields::FieldChip;
// use halo2_base::halo2_proofs::halo2curves::bn256::Fq12;

// use halo2_base::utils::{BigPrimeField,ScalarField};
// use halo2_base::{AssignedValue, Context};




// ---------
// use halo2_base::{halo2_proofs::{arithmetic::Field, halo2curves::{bn256::{Fr, G1Affine}, ff::PrimeField, serde::SerdeObject}}, utils::ScalarField};
// use halo2_base::poseidon::PoseidonChip;
// use halo2_base::poseidon::PoseidonInstructions;
// use halo2_base::{
//     gates::{RangeChip, RangeInstructions},
//     poseidon::hasher::PoseidonHasher,
//     safe_types::{FixLenBytes, VarLenBytes, VarLenBytesVec},
//     utils::BigPrimeField,
//     AssignedValue, Context,
// };
// use rand_core::OsRng;
// use light_poseidon::{Poseidon, PoseidonBytesHasher, parameters::bn254_x5};

// fn main(){
//     let g1 = G1Affine::generator();
//     // let sk = Fr::random(OsRng);
//     // let sk = Fr::from_u128(0x08b209cf1122a4e87693e9224895572da2666197ba09269591ce3e4f73b99dd7);
//     let sk = Fr::from_raw([0x91ce3e4f73b99dd7,0xa2666197ba092695, 0x7693e9224895572d ,0x08b209cf1122a4e8]);
//     // let sk = Fr::from_raw([1, 2, 3, 4]);
//     println!("{:?}", sk);
//     let pk = G1Affine::from(g1 * sk);
//     println!("{:?}", pk);
//     let pk_bytes_mid = pk.to_raw_bytes();

//     let mut poseidon1 = Poseidon::<Fr>::new_circom(1).unwrap();


    

//     // let mut poseidon = PoseidonChip::new(
//     //     ctx,
//     //     OptimizedPoseidonSpec::new(2, 8, 55, 5, 8, 2, 0, 0),
//     //     range,
//     // );
//     // let pk = poseidon.hash_fix_len_bytes(ctx, &pk);
// }



use poseidon::Poseidon;
use halo2curves::bn256::Fr;

fn main(){
        let mut hasher = Poseidon::<Fr, 3, 2>::new(8, 57);

        // let input1 = Fr::from(18446744073709551615);
        let input1 = Fr::from_raw([0,1, 0 ,0]);//18446744073709551616
        // let input1 = Fr::from_raw([0x91ce3e4f73b99dd7,0xa2666197ba092695, 0x7693e9224895572d ,0x08b209cf1122a4e8]);
        // let input2 = Fr::from(0x64u64);

        println!("Input1: {:?}", input1);
        // In sake of the example we generate some dummy scalar inputs
        let inputs = vec![input1];

        // Feed inputs to the Absorption line
        hasher.update(&inputs[..]);

        // Yield your challange with squeeze function
        let challenge_alpha = hasher.squeeze();

        println!("Challenge alpha: {:?}", challenge_alpha);
}