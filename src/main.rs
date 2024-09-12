
use halo2curves::bn256::{Fq, G2Affine};
// use halo2curves::serde::SerdeObject;
use halo2curves::{bn256::{Fr, G1Affine}, ff::Field};
use rand_core::OsRng;
use serde::Serialize;
use serde_json::json;
use hex::encode; 
use std::fs::File;
use std::io::{Read, Write};
use std::time::Instant;
use poseidon::Poseidon;
use rand::Rng;

#[derive(Serialize)]
struct MerklePath {
    sk: String,
    path: Vec<String>,
    index: Vec<bool>,
}

fn fr_to_string(fr: &Fr) -> String {
    let bytes = fr.to_bytes();
    encode(bytes)
}


fn fq_to_string(fq : &Fq) -> String {
    let bytes = fq.to_bytes();
    encode(bytes)
}

fn hash_x_coordinate(pk: &G1Affine) -> Fr {
    let x_bytes = pk.x.to_bytes();
    // println!("x_bytes: {:?}", fr_to_string(&Fr::from_bytes(&x_bytes).unwrap()));
    let mut hasher = Poseidon::<Fr, 3, 2>::new(8, 57);
    hasher.update(&[Fr::from_bytes(&x_bytes).unwrap(  ), Fr::zero()]);
    hasher.squeeze()
}

fn hash_x_coordinate_g2(pk: &G2Affine) -> Fr {
    let x_c0_bytes = pk.x.c0.to_bytes();
    let x_c1_bytes = pk.x.c1.to_bytes();
    // println!("x_bytes: {:?}", fr_to_string(&Fr::from_bytes(&x_bytes).unwrap()));
    let mut hasher = Poseidon::<Fr, 3, 2>::new(8, 57);
    hasher.update(&[Fr::from_bytes(&x_c0_bytes).unwrap(), Fr::from_bytes(&x_c1_bytes).unwrap()]);
    hasher.squeeze()
}

fn generate_random_string(length: usize) -> String {
    let charset: &[u8] = b"0123456789adcdef";
    let mut rng = rand::thread_rng();

    let random_string: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect();

    random_string
}

fn gen_merkle(merkle_depth: usize) {

    // k = 2^depth
    let k = 1 << merkle_depth;
    let inputs_start = Instant::now();
    // 生成k个随机输入
    let mut sk_pks = Vec::new();
    for _ in 0..k {
        let sk = Fr::random(OsRng);
        let pubkey = G1Affine::from(G1Affine::generator() * sk); // Calculate public key from secret key
        sk_pks.push((sk, pubkey));
        // println!("sk: {:?}, pk: {:?}", sk, pubkey);
    }
    let duration = inputs_start.elapsed();
    println!("Time to generate keys: {:?}", duration);

    let merkle_start = Instant::now();
    // 存储Merkle树的叶子节点
    let mut leaves = Vec::new();
    for (_, pk) in &sk_pks {
        let hashed_pk = hash_x_coordinate(pk);
        leaves.push(hashed_pk);
        // println!("hashed_pk: {:?}", hashed_pk);
    }
    let duration = merkle_start.elapsed();
    println!("Time to generate leaves: {:?}", duration);

    // 构建Merkle树并记录路径
    let mut paths: Vec<MerklePath> = Vec::new();
    let mut level = leaves.clone();
    let mut tree = vec![level.clone()];

    let start = Instant::now();
    while level.len() > 1 {
        let mut next_level = Vec::new();
        for chunk in level.chunks(2) {
            let mut hasher = Poseidon::<Fr, 3, 2>::new(8, 57);
            let (left, right) = (chunk[0], chunk.get(1).cloned().unwrap_or(chunk[0]));
            hasher.update(&[left, right]);
            next_level.push(hasher.squeeze());
        }
        level = next_level;
        tree.push(level.clone());
    }
    let duration = start.elapsed();
    println!("Time to build Merkle tree: {:?}", duration);

    let root = tree.last().unwrap()[0];

    let start = Instant::now();
    for (i, (sk, pk)) in sk_pks.iter().enumerate() {
        let mut path = Vec::new();
        let mut index = Vec::new();
        let mut position = i;

        for level in &tree[..tree.len() - 1] {
            let sibling_position = position ^ 1;
            path.push(level.get(sibling_position).cloned().unwrap_or(level[position]));
            index.push(position % 2 == 0); // true 表示左侧，false 表示右侧
            position /= 2;
        }

        paths.push(MerklePath {
            sk: fr_to_string(sk),
            path: path.iter().map(fr_to_string).collect(),
            index,
        });
    }
    let duration = start.elapsed();
    println!("Time to generate paths: {:?}", duration);

    // message: 随机产生的字符串
    // let message = generate_random_string(32);

    // let hash_msg = message.as_bytes().iter().fold(Fr::ZERO, |acc, &byte| {
    //     let mut hasher = Poseidon::<Fr, 3, 2>::new(8, 57);
    //     hasher.update(&[acc, Fr::ZERO]);
    //     hasher.squeeze()
    // });

    let msg_f = Fr::random(OsRng);
    let message = fr_to_string(&msg_f);
    let mut hasher = Poseidon::<Fr, 3, 2>::new(8, 57);
    hasher.update(&[msg_f, Fr::ZERO]);
    let hash_msg = hasher.squeeze();
    // 转换为JSON格式
    let result = serde_json::to_string_pretty(&json!({
        "root": fr_to_string(&root),
        "leaves": paths,
        "message": message,
        "hash_msg": fr_to_string(&hash_msg),
    })).unwrap();
    let path = "data/merkle_tree_from_g1_{k}.json".replace("{k}", &k.to_string());
    // 将结果写入文件
    let mut file = File::create(path).expect("Unable to create file");
    file.write_all(result.as_bytes()).expect("Unable to write data");

    println!("Data has been written to data/merkle_tree_from_g1_{}.json",k);
}

fn gen_merkle2(merkle_depth: usize){
    // k = 2^depth
    let k = 1 << merkle_depth;
    let inputs_start = Instant::now();
    // 生成k个随机输入
    let mut sk_pks = Vec::new();
    for _ in 0..k {
        let sk = Fr::random(OsRng);
        let pubkey = G2Affine::from(G2Affine::generator() * sk); // Calculate public key from secret key
        sk_pks.push((sk, pubkey));
        // println!("sk: {:?}, pk: {:?}", sk, pubkey);
    }
    let duration = inputs_start.elapsed();
    println!("Time to generate keys: {:?}", duration);

    let merkle_start = Instant::now();
    // 存储Merkle树的叶子节点
    let mut leaves = Vec::new();
    for (_, pk) in &sk_pks {
        let hashed_pk = hash_x_coordinate_g2(pk);
        leaves.push(hashed_pk);
        // println!("hashed_pk: {:?}", hashed_pk);
    }
    let duration = merkle_start.elapsed();
    println!("Time to generate leaves: {:?}", duration);

    // 构建Merkle树并记录路径
    let mut paths: Vec<MerklePath>= Vec::new();
    let mut level = leaves.clone();
    let mut tree = vec![level.clone()];

    let start = Instant::now();
    while level.len() > 1 {
        let mut next_level = Vec::new();
        for chunk in level.chunks(2) {
            let mut hasher = Poseidon::<Fr, 3, 2>::new(8, 57);
            let (left, right) = (chunk[0], chunk.get(1).cloned().unwrap_or(chunk[0]));
            hasher.update(&[left, right]);
            next_level.push(hasher.squeeze());
        }
        level = next_level;
        tree.push(level.clone());
    }
    let duration = start.elapsed();
    println!("Time to build Merkle tree: {:?}", duration);

    let root = tree.last().unwrap()[0];

    let start = Instant::now();
    for (i, (sk, pk)) in sk_pks.iter().enumerate() {
        let mut path = Vec::new();
        let mut index = Vec::new();
        let mut position = i;

        for level in &tree[..tree.len() - 1] {
            let sibling_position = position ^ 1;
            path.push(level.get(sibling_position).cloned().unwrap_or(level[position]));
            index.push(position % 2 == 0); // true 表示左侧，false 表示右侧
            position /= 2;
        }

        paths.push(MerklePath {
            sk: fr_to_string(sk),
            path: path.iter().map(fr_to_string).collect(),
            index,
        });
    }
    let duration = start.elapsed();
    println!("Time to generate paths: {:?}", duration);

    // message: 随机产生的字符串
    // let message = generate_random_string(32);

    // let hash_msg = message.as_bytes().iter().fold(Fr::ZERO, |acc, &byte| {
    //     let mut hasher = Poseidon::<Fr, 3, 2>::new(8, 57);
    //     hasher.update(&[acc, Fr::ZERO]);
    //     hasher.squeeze()
    // });

    let msg_f = Fr::random(OsRng);
    let message = fr_to_string(&msg_f);
    let mut hasher = Poseidon::<Fr, 3, 2>::new(8, 57);
    hasher.update(&[msg_f, Fr::ZERO]);
    let hash_msg = hasher.squeeze();
    // 转换为JSON格式
    let result = serde_json::to_string_pretty(&json!({
        "root": fr_to_string(&root),
        "leaves": paths,
        "message": message,
        "hash_msg": fr_to_string(&hash_msg),
    })).unwrap();
    let path = "data/merkle_tree_from_g1_{k}.json".replace("{k}", &k.to_string());
    // 将结果写入文件
    let mut file = File::create(path).expect("Unable to create file");
    file.write_all(result.as_bytes()).expect("Unable to write data");

    println!("Data has been written to data/merkle_tree_from_g1_{}.json",k);
}

fn gen_data_for_msp(degree:usize){
    // k = 2^degree
    let k = 1 << degree;
    let inputs_start = Instant::now();

    // 生成k个随机输入
    let mut sk_pks = Vec::new();
    for _ in 0..k {
        let sk = Fr::random(OsRng);
        let pubkey = G1Affine::from(G1Affine::generator() * sk); // Calculate public key from secret key
        sk_pks.push((sk, pubkey));
    }
    let duration = inputs_start.elapsed();
    println!("Time to generate keys: {:?}", duration);


    let msg_f = Fr::random(OsRng);
    let message = fr_to_string(&msg_f);
    let mut hasher = Poseidon::<Fr, 3, 2>::new(8, 57);
    // hasher.update(&[msg_f, Fr::from("M".as_bytes())]);
    hasher.update(&[msg_f, Fr::ZERO]);
    let hash_msg = hasher.squeeze();

    // 转换为JSON格式
    let result = serde_json::to_string_pretty(&json!({
        "message": message,
        "hash_msg": fr_to_string(&hash_msg),
        "degree": degree,
        "pubkeys": sk_pks.iter().map(|(sk, pk)| {
            (fr_to_string(&sk), fq_to_string(&pk.x), fq_to_string(&pk.y))
        }).collect::<Vec<_>>(),
    })).unwrap();
    let path = "data/data_for_msp_{k}.json".replace("{k}", &k.to_string());
    // 将结果写入文件
    let mut file = File::create(path).expect("Unable to create file");
    file.write_all(result.as_bytes()).expect("Unable to write data");

}
fn main() {
    // 从data/merkle_depth.in文件中读取输入两个数字merkle_depth_1和merkle_depth_2
    // let mut file = File::open("data/merkle_depth.in").expect("Unable to open file");
    // let mut data = String::new();
    // file.read_to_string( &mut data).expect("Unable to read file");
    // let mut lines = data.lines();
    // let merkle_depth_1 = lines.next().unwrap().parse::<usize>().unwrap();
    // let merkle_depth_2 = lines.next().unwrap().parse::<usize>().unwrap();
    // for i in merkle_depth_1..=merkle_depth_2 {
    //     println!("[INFO] Generating Merkle tree with depth: {}", i);
    //     gen_merkle2(i);
    // }

    // MSP
    let mut file = File::open("data/data_for_msp.in").expect("Unable to open file");
    let mut data = String::new();
    file.read_to_string( &mut data).expect("Unable to read file");
    let mut lines = data.lines();
    let low = lines.next().unwrap().parse::<usize>().unwrap();
    let high = lines.next().unwrap().parse::<usize>().unwrap();
    for i in low..=high {
        println!("[INFO] Generating data for MSP with degree: {}", i);
        gen_data_for_msp(i);
    }
}