use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

// 区块结构
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Block {
    index: u64,
    timestamp: u128,
    transactions: Vec<Transaction>,
    proof: u64,
    previous_hash: String,
}

// 交易结构
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: u64,
}

// 区块链结构
#[derive(Clone)]
struct Blockchain {
    chain: Vec<Block>,
    pending_transactions: Vec<Transaction>,
}

impl Blockchain {
    fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            pending_transactions: Vec::new(),
        };
        blockchain.create_genesis_block();
        blockchain
    }

    // 创世区块
    fn create_genesis_block(&mut self) {
        let genesis_block = Block {
            index: 0,
            timestamp: now(),
            transactions: Vec::new(),
            proof: 100,
            previous_hash: "0".to_string(),
        };
        self.chain.push(genesis_block);
    }

    // 获取最后一个区块
    fn get_last_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    // 添加新交易
    fn add_transaction(&mut self, sender: String, receiver: String, amount: u64) {
        let transaction = Transaction {
            sender,
            receiver,
            amount,
        };
        self.pending_transactions.push(transaction);
    }

    // 工作量证明
    fn proof_of_work(&self, last_proof: u64) -> u64 {
        let mut proof = 0;
        while !self.valid_proof(last_proof, proof) {
            proof += 1;
        }
        proof
    }

    // 验证工作量证明
    fn valid_proof(&self, last_proof: u64, proof: u64) -> bool {
        let guess = format!("{}{}", last_proof, proof);
        let mut hasher = Sha256::new();
        hasher.update(guess.as_bytes());
        let result = hasher.finalize();
        let hash = format!("{:x}", result);
        hash.starts_with("0000")
    }

    // 将待处理的交易打包到新区块
    fn add_block(&mut self, proof: u64) -> &Block {
        let previous_hash = self.hash_block(self.get_last_block());
        let block = Block {
            index: self.chain.len() as u64,
            timestamp: now(),
            transactions: self.pending_transactions.clone(),
            proof,
            previous_hash,
        };

        self.pending_transactions.clear();
        self.chain.push(block);
        self.chain.last().unwrap()
    }

    // 计算区块的哈希值
    fn hash_block(&self, block: &Block) -> String {
        let block_data = serde_json::to_string(block).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(block_data.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    // 校验区块链是否有效
    fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.previous_hash != self.hash_block(previous_block) {
                return false;
            }

            if !self.valid_proof(previous_block.proof, current_block.proof) {
                return false;
            }
        }
        true
    }
}

// 节点结构
#[derive(Clone)]
struct Node {
    blockchain: Blockchain,
    neighbor_nodes: Vec<Node>,
}

impl Node {
    fn new() -> Self {
        Node {
            blockchain: Blockchain::new(),
            neighbor_nodes: Vec::new(),
        }
    }

    fn add_neighbor(&mut self, neighbor: Node) {
        self.neighbor_nodes.push(neighbor);
    }

    fn synchronize_chain(&mut self) {
        for neighbor in &self.neighbor_nodes {
            if neighbor.blockchain.chain.len() > self.blockchain.chain.len() {
                if neighbor.blockchain.is_chain_valid() {
                    self.blockchain.chain = neighbor.blockchain.chain.clone();
                    println!("同步到更长的链");
                }
            }
        }
    }
}

// 获取当前时间戳
fn now() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("时间不可能倒流");
    since_the_epoch.as_millis()
}

fn main() {
    // 创建两个节点
    let mut node1 = Node::new();
    let mut node2 = Node::new();

    // 添加邻居节点
    node1.add_neighbor(node2.clone());
    node2.add_neighbor(node1.clone());

    // 节点1添加交易
    node1
        .blockchain
        .add_transaction("Alice".to_string(), "Bob".to_string(), 50);
    node1
        .blockchain
        .add_transaction("Bob".to_string(), "Charlie".to_string(), 25);

    // 节点1进行工作量证明并打包区块
    let last_proof = node1.blockchain.get_last_block().proof;
    let proof = node1.blockchain.proof_of_work(last_proof);
    let new_block = node1.blockchain.add_block(proof);

    println!("节点1打包了区块: {:?}", new_block);

    // 节点2同步区块
    node2.synchronize_chain();
    println!("节点2链的长度: {}", node2.blockchain.chain.len());
}
