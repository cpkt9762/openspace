use base58::ToBase58;
use rand::rngs::{OsRng, StdRng};
use rand::SeedableRng;
use rand_core::RngCore;
use rsa::{PaddingScheme, PublicKey, RSAPrivateKey, RSAPublicKey};
use sha2::{Digest, Sha256};
use std::time::Instant; // Add this import
fn generate_keys() -> (RSAPrivateKey, RSAPublicKey) {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let private_key = RSAPrivateKey::new(&mut rng, bits).expect("生成私钥失败");
    let public_key = RSAPublicKey::from(&private_key);
    (private_key, public_key)
}
// PoW 计算函数
fn perform_pow(nickname: &str, leading_zeros: usize) -> (String, u64) {
    let start_time = Instant::now();
    let target = "0".repeat(leading_zeros);
    let mut nonce = 0;
    let mut hash = String::new();

    loop {
        let input = format!("{}{}", nickname, nonce);
        let mut hasher = Sha256::new();
        hasher.update(input);
        let result = hasher.finalize();
        hash = format!("{:x}", result);

        if hash.starts_with(&target) {
            break;
        }

        nonce += 1;
    }

    let duration = start_time.elapsed();
    println!("满足 {} 个0开头的哈希: {}", leading_zeros, hash);
    println!("使用昵称 \"{}\" 和 nonce {}", nickname, nonce);
    println!("花费时间: {:.2?}", duration);

    (hash, nonce)
}

// 使用私钥对昵称+nonce 进行签名
fn sign_data(private_key: &RSAPrivateKey, data: &str) -> Vec<u8> {
    let mut rng = OsRng;
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hashed_data = hasher.finalize();

    private_key
        .sign(
            PaddingScheme::PKCS1v15Sign {
                hash: Some(rsa::Hash::SHA2_256),
            },
            &hashed_data,
        )
        .expect("签名失败")
}

// 使用公钥验证签名
fn verify_signature(public_key: &RSAPublicKey, data: &str, signature: &[u8]) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hashed_data = hasher.finalize();

    public_key
        .verify(
            PaddingScheme::PKCS1v15Sign {
                hash: Some(rsa::Hash::SHA2_256),
            },
            &hashed_data,
            signature,
        )
        .is_ok()
}

fn main() {
    let nickname = "myname";

    // 生成公私钥对
    let (private_key, public_key) = generate_keys();

    // 执行 PoW 并获得哈希和 nonce
    let (pow_hash, nonce) = perform_pow(nickname, 4);

    // 准备签名的数据 (昵称 + nonce)
    let data_to_sign = format!("{}{}", nickname, nonce);

    // 使用私钥签名数据
    let signature: Vec<u8> = sign_data(&private_key, &data_to_sign);
    let hex_string = hex::encode(signature.clone());
    println!("签名成功，签名: {:?}", hex_string);

    // 使用公钥验证签名
    let is_valid = verify_signature(&public_key, &data_to_sign, &signature);
    println!("签名验证结果: {}", if is_valid { "有效" } else { "无效" });
}
