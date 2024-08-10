#[macro_use] extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::tokio::sync::Mutex;
use rocket::http::Status;
use rocket::config::{Config, TlsConfig};
use std::sync::Arc;
use chrono::{DateTime, Utc};
use reqwest::Client;
use rand::{Rng, prelude::SliceRandom};
use sha2::{Sha256, Digest};
use std::collections::HashMap;

use ntru::ntru_encrypt::NtruEncrypt;
use ntru::ntru_sign::NtruSign;
use ntru::ntru_param::NtruParam;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Block {
    index: u64,
    timestamp: DateTime<Utc>,
    data: String,
    prev_hash: String,
    hash: String,
    verifiable_credential: String,
    signature: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f64,
    verifiable_credential: String,
    signature: Vec<u8>,
    location: (f64, f64),
    timestamp: DateTime<Utc>,
    proof_of_place: String,
    transaction_id: String,
}

impl Transaction {
    fn new(
        sender: String,
        receiver: String,
        amount: f64,
        verifiable_credential: String,
        location: (f64, f64),
    ) -> Self {
        let mut transaction = Transaction {
            sender,
            receiver,
            amount,
            verifiable_credential,
            signature: vec![],
            location,
            timestamp: Utc::now(),
            proof_of_place: String::new(),
            transaction_id: "0".to_string(),
        };
        transaction.proof_of_place = ProofOfPlace::new(location).generate_proof();
        transaction
    }

    fn generate_proof_of_history(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{:?}{:?}", self.sender, self.timestamp).as_bytes());
        hex::encode(hasher.finalize())
    }

    fn verify_signature(&self, public_key: &str) -> bool {
        let computed_signature = hex::encode(Sha256::digest(format!("{:?}", self).as_bytes()));
        self.signature == computed_signature.into_bytes()
    }
}

type Blockchain = Arc<Mutex<Vec<Block>>>;

struct DPoS {
    municipalities: Vec<String>,
    approved_representative: Option<String>,
}

impl DPoS {
    fn new(municipalities: Vec<String>) -> Self {
        Self {
            municipalities,
            approved_representative: None,
        }
    }

    fn elect_representative(&mut self) -> String {
        let representative = self.municipalities.choose(&mut rand::thread_rng()).unwrap().clone();
        self.approved_representative = Some(representative.clone());
        representative
    }

    fn approve_transaction(&self, transaction: &mut Transaction) -> Result<&str, &str> {
        if let Some(representative) = &self.approved_representative {
            transaction.signature = format!("approved_by_{}", representative).into_bytes();
            Ok("Transaction approved")
        } else {
            Err("No representative elected")
        }
    }
}

struct ProofOfPlace {
    location: (f64, f64),
    timestamp: DateTime<Utc>,
}

impl ProofOfPlace {
    fn new(location: (f64, f64)) -> Self {
        ProofOfPlace {
            location,
            timestamp: Utc::now(),
        }
    }

    fn generate_proof(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{:?}{:?}", self.location, self.timestamp).as_bytes());
        hex::encode(hasher.finalize())
    }

    fn verify_proof(proof: &str, location: (f64, f64), timestamp: DateTime<Utc>) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(format!("{:?}{:?}", location, timestamp).as_bytes());
        let computed_proof = hex::encode(hasher.finalize());
        computed_proof == proof
    }
}

struct ProofOfHistory {
    sequence: Vec<String>,
}

impl ProofOfHistory {
    fn new() -> Self {
        ProofOfHistory { sequence: Vec::new() }
    }

    fn add_event(&mut self, event: &str) {
        self.sequence.push(event.to_string());
    }

    fn generate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        for event in &self.sequence {
            hasher.update(event.as_bytes());
        }
        hex::encode(hasher.finalize())
    }
}

struct Consensus {
    dpos: DPoS,
    poh: ProofOfHistory,
    transactions: Vec<Transaction>,
}

impl Consensus {
    fn new(municipalities: Vec<String>) -> Self {
        Consensus {
            dpos: DPoS::new(municipalities),
            poh: ProofOfHistory::new(),
            transactions: Vec::new(),
        }
    }

    fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    fn process_transactions(&mut self) {
        for transaction in &mut self.transactions {
            if self.dpos.approve_transaction(transaction).is_ok() {
                self.poh.add_event(&transaction.generate_proof_of_history());
                println!("Transaction processed: {:?}", transaction);
            } else {
                println!("Transaction failed: {:?}", transaction);
            }
        }
    }

    fn generate_poh_hash(&self) -> String {
        self.poh.generate_hash()
    }
}

#[get("/")]
fn index() -> &'static str {
    "Welcome to the Global Main Chain!"
}

#[post("/transaction", format = "json", data = "<transaction>")]
async fn create_transaction(transaction: Json<Transaction>, client: &rocket::State<Client>) -> Json<Transaction> {
    let mut consensus = Consensus::new(vec!["Municipality1".to_string(), "Municipality2".to_string()]);
    consensus.add_transaction(transaction.into_inner());
    consensus.process_transactions();

    let global_chain_url = "https://127.0.0.1:8000/transaction";
    let res = client.post(global_chain_url)
                    .json(&consensus.transactions.last().unwrap())
                    .send()
                    .await;

    match res {
        Ok(_) => Json(consensus.transactions.last().unwrap().clone()),
        Err(_) => Json(Transaction {
            sender: "error".to_string(),
            receiver: "error".to_string(),
            amount: 0.0,
            verifiable_credential: "error".to_string(),
            signature: vec![],
            location: (0.0, 0.0),
            timestamp: Utc::now(),
            proof_of_place: "error".to_string(),
            transaction_id: "error".to_string(),
        }),
    }
}

#[post("/add_block", format = "json", data = "<block>")]
async fn add_block(block: Json<Block>, chain: &rocket::State<Blockchain>, client: &rocket::State<Client>) -> Status {
    println!("Received block: {:?}", block);
    let mut chain = chain.lock().await;
    let block_inner = block.into_inner();
    let block_clone = block_inner.clone();
    chain.push(block_inner);

    println!("Block added to local chain. Attempting to send to global chain.");

    let global_chain_url = "https://127.0.0.1:8000/add_block";
    let res = client.post(global_chain_url)
                    .json(&block_clone)
                    .send()
                    .await;

    match res {
        Ok(_) => {
            println!("Block successfully sent to global chain.");
            Status::Accepted
        },
        Err(e) => {
            println!("Failed to send block to global chain: {:?}", e);
            Status::InternalServerError
        }
    }
}

#[get("/chain")]
async fn get_chain(chain: &rocket::State<Blockchain>) -> Json<Vec<Block>> {
    let chain = chain.lock().await;
    Json(chain.clone())
}

#[rocket::main]
async fn main() {
    let chain = Arc::new(Mutex::new(Vec::<Block>::new()));

    let tls_config = TlsConfig::from_paths("d:\\city_chain_project-3\\cert.crt", "d:\\city_chain_project-3\\key.pem");
    let config = Config::figment()
        .merge(("tls.certs", "d:\\city_chain_project-3\\cert.crt"))
        .merge(("tls.key", "d:\\city_chain_project-3\\key.pem"));

    rocket::custom(config)
        .manage(chain)
        .manage(Client::new())
        .mount("/", routes![index, create_transaction, add_block, get_chain])
        .launch()
        .await
        .unwrap();
}
