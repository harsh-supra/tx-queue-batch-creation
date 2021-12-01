#[derive(Debug)]
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use chrono::{DateTime, FixedOffset, Local, Utc};


// struct Block {
//     timestamp: i64,
//     batch_hash: String,
//     previous_block_hash: String,
//     transaction: Vec<Transaction>,  
//     batch_hashes: Vec<Batch_Hash>,
// }

//type Batch_Hash = Vec<String>;

pub struct Transaction {
    transaction_id: u32,
    transaction_sender: String,
    transaction_receiver: String,
    amount: f32,
    partial_signature: String,
}

struct Batch {
    previous_batch_hash: String,
    batch_transactions: transaction,
    batch_index: u32,
    batch_hash: String,
    batch_timestamp: i32,
}


fn add_transactions(_transaction_id: u32,_transaction_sender: String,_transaction_receiver: String,
                        _amount: f32,_partial_signature: String) -> transaction {
    let transactions = Transaction {
        transaction_id : _transaction_id,
        transaction_sender : _transaction_sender,
        transaction_receiver : _transaction_receiver,
        amount : _amount,
        partial_signature: _partial_signature,
    };
    transaction.push(transactions);
    return transaction;
    if transaction.len() == batch_capacity {
        create_batch(previousbatchhash, transaction)
    }
    
}


fn create_batch(_previous_batch_hash: String, _transactions: & mut [Transaction]) -> Vec<Batch> {
    let local_time = Local::now();
    let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
    
    let batch = Batch {

        previous_batch_hash: _previous_batch_hash,
        batch_transactions: _transactions, //only need 5000 transaction so how do you extract.?
        batch_index: batch_index++,
        batch_hash: calculate_hash(&previous_batch_hash),
        batch_timestamp: utc_time,
    };
    batches.push(batch);
    batch_hashes.insert(batch_index, String::from("previous_batch_hash"));
    return batches;

}

fn calculate_hash<T: Hash>(t: &T) -> u64 {      //How can we create Alphanumeric value.?
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn main() {    //Is it required to name the main function in a file with the filename.?
    
    // if batches.len() == 0 {
        //Where and how should we keep this condition.?
    // }


}


//Create batch out of 5000 transactions
//Add account based sharding--Assign that particular transactions to a tribe based on range
