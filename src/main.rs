mod transaction;
use transaction::Transaction;
use std::collections::HashMap;
#[derive(Debug)]
struct Tribes {
   
    tribe_id : u32,
    from_address_index: u32,
    to_address_index : u32,
}


fn return_tribes_info(chunksize: u32) -> Vec<Tribes> {
    let mut tribes = Vec::new();   //Shouldn't this be created in main.?
    for i in 1..=5 {
        

        let tribe_name = Tribes {        
            tribe_id : i,
            from_address_index : (i-1)*chunksize,
            to_address_index : i*chunksize - 1,
        };
        tribes.push(tribe_name);
    }
    return tribes;
}





fn main() {
        //let tribe1, tribe2, tribe3, tribe4, tribe5;
    let totaltribes: u32 = 5;
    let totalaccounts: u32 = 200000;
    let chunksize = totalaccounts / totaltribes;
    let batch_index = 0;
    let mut batches = Vec::new();
    //Can we create this transaction type as mutable
    type transaction = Vec<Transaction>; //This will be the queue where we will add all our transaction 
    let mut batch_hashes = HashMap::new();
    let mut batch_capacity = 5000;
    
    let choose_tribe = return_tribes_info(chunksize);
        
    println!(" {:?}" , choose_tribe);
}


