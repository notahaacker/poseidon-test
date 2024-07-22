// use crate::Poseidon;
// use crate::{Spec, State};
// use halo2curves::bn256::Fr;
// use halo2curves::group::ff::{FromUniformBytes, PrimeField};

// const T: usize = 3;
// const RATE: usize = 2;

// fn test(){
//     // Initialize a mutable hasher with constant capacity parameters 
// // and number of rounds arguments. This will also generate matrices 
// // and constants according to the specification
// let mut hasher = Poseidon::<Fr, T, RATE>::new(8, 57);

//     let input1 = Fr::from(6u64);
//     let input2 = Fr::from(100u64);

// // In sake of the example we generate some dummy scalar inputs
// let inputs = vec![input1, input2];

// // Feed inputs to the Absorption line
// hasher.update(&inputs[..]);

// // Yield your challange with squeeze function
// let challenge_alpha = hasher.squeeze();

// println!("Challenge alpha: {:?}", challenge_alpha);

// }