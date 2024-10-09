use sha2::{Digest, Sha256};
use std::time::Instant;

// PoW 计算函数
fn perform_pow(nickname: &str, leading_zeros: usize) {
    let start_time = Instant::now(); // 记录开始时间
    let target = "0".repeat(leading_zeros); // 目标哈希前缀
    let mut nonce = 0;
    let mut hash = String::new();

    // 循环计算，直到哈希值满足条件
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

    let duration = start_time.elapsed(); // 记录结束时间
    println!("满足 {} 个0开头的哈希: {}", leading_zeros, hash);
    println!("使用昵称 \"{}\" 和 nonce {}", nickname, nonce);
    println!("花费时间: {:.2?}", duration);
}

fn main() {
    let nickname = "myname";

    // 4 个0开头
    perform_pow(nickname, 4);

    // 5 个0开头
    perform_pow(nickname, 5);
}
