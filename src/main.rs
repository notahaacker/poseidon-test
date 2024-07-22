use poseidon::Poseidon;
use halo2curves::bn256::Fr;

fn main(){
        let mut hasher = Poseidon::<Fr, 3, 2>::new(8, 57);

        // let input1 = Fr::from_raw([0,1, 0 ,0]);//18446744073709551616
        // let input1 = Fr::from_raw([0x91ce3e4f73b99dd7,0xa2666197ba092695, 0x7693e9224895572d ,0x08b209cf1122a4e8]);
        let input1 = Fr::from(0);
        let input2 = Fr::from(1);

        println!("Input1: {:?}, Input2: {:?}", input1, input2);
        // In sake of the example we generate some dummy scalar inputs
        let inputs = vec![input1];

        // Feed inputs to the Absorption line
        hasher.update(&inputs[..]);

        // Yield your challange with squeeze function
        let hash1 = hasher.squeeze();
        println!("Hash1: {:?}", hash1);

        let mut hasher = Poseidon::<Fr, 3, 2>::new(8, 57);
        let inputs = vec![input2];
        hasher.update(&inputs[..]);
        let hash2 = hasher.squeeze();
        println!("Hash2: {:?}", hash2);

        let mut hasher = Poseidon::<Fr, 3, 2>::new(8, 57);
        let inputs = vec![hash1,hash2];
        hasher.update(&inputs[..]);
        let root = hasher.squeeze();
        println!("Root: {:?}", root);
}
