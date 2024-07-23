use poseidon::Poseidon;
use halo2curves::{bn256::Fr, ff::Field};
use rand_core::OsRng;
use serde::Serialize;
use serde_json::json;
use hex::encode; 
use halo2_base::halo2_proofs::halo2curves::bn256::G1Affine;

use std::fs::File;
use std::io::Write;
use std::time::Instant;

#[derive(Serialize)]
struct MerklePath {
    input: String,
    path: Vec<String>,
    index: Vec<bool>,
}

fn fr_to_string(fr: &Fr) -> String {
    let bytes = fr.to_bytes();
    encode(bytes)
}

fn main() {

    let inputs_start = Instant::now();
    // 生成1024个随机输入
    let inputs: Vec<Fr> = (0..16).map(|_| Fr::random(OsRng)).collect();
    let duration = inputs_start.elapsed();
    println!("Time to generate inputs: {:?}", duration);

    let merkle_start = Instant::now();
    // 存储Merkle树的叶子节点
    let mut leaves = Vec::new();
    for input in &inputs {
        let mut hasher = Poseidon::<Fr, 3, 2>::new(8, 57);
        hasher.update(&[*input]);
        leaves.push(hasher.squeeze());
    }
    let duration = merkle_start.elapsed();
    println!("Time to generate leaves: {:?}", duration);


    // 构建Merkle树并记录路径
    let mut paths = Vec::new();
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
    for (i, input) in inputs.iter().enumerate() {
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
            input: fr_to_string(input),
            path: path.iter().map(fr_to_string).collect(),
            index,
        });
    }
    let duration = start.elapsed();
    println!("Time to generate paths: {:?}", duration);

    // 转换为JSON格式
    let result = serde_json::to_string_pretty(&json!({
        "root": fr_to_string(&root),
        "leaves": paths,
    })).unwrap();

        // 将结果写入文件
    let mut file = File::create("merkle_tree_16.json").expect("Unable to create file");
    file.write_all(result.as_bytes()).expect("Unable to write data");

    println!("Data has been written to merkle_tree.json");
//     println!("{}", result);
}
