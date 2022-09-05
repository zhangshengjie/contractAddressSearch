/*
 * @Description:
 * @Version: 1.0
 * @Autor: z.cejay@gmail.com
 * @Date: 2022-09-02 10:37:47
 * @LastEditors: cejay
 * @LastEditTime: 2022-09-05 17:33:32
 */
use chrono::prelude::*;
use regex::Regex;
use std::env;
use std::thread::{self, JoinHandle};

fn calc_address(
    salt_from: u64,
    salt_to: u64,
    create2_factory_address: &[u8; 20],
    code_hash: &[u8; 32],
    regex: Regex,
) {
    println!("thread started: {} - {}", salt_from, salt_to);
    let mut salt_num = salt_from;
    while salt_num < salt_to {
        let salt = &(hex::decode(format!("{:064x}", salt_num)).unwrap())
            .try_into()
            .unwrap();
        let addr = create2::calc_addr_with_hash(create2_factory_address, salt, code_hash);
        let address_hex = hex::encode(addr);
        // if salt_num % 1000000 == 0 {
        //     print!("{},", salt_num);
        // }
        //if address_hex.starts_with("00000") {
        if regex.is_match(&address_hex) {
            let local: DateTime<Local> = Local::now();
            println!(
                "address: {},salt_num: {},time: {}",
                address_hex,
                salt_num,
                local.format("%Y-%m-%d %H:%M:%S")
            );
        }
        salt_num += 1;
    }
    println!("done");
}

//
pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("please input regex and contract code hash");
        return;
    }

    println!("contract code hash: 【{}】", args[1]);
    println!("regex: 【{}】", args[2]);
    let thread_count = num_cpus::get(); // * 2;
    println!("thread num: {}", thread_count);


    let regex_str = String::from(&args[2]); //let regex_str = String::from(r".*000000.*");
    let regex_rule = Regex::new(&regex_str).unwrap();

    let create2_factory = "ce0042b868300000d44a59004da54a005ffdcf9f";
    let initcode_hash = &args[1]; //"bc36789e7a1e281436464229828f817d6612f7b477d66591ff96a9e064bcc98a";
    let code_hash: &[u8; 32] = &(hex::decode(initcode_hash).unwrap())[..32]
        .try_into()
        .unwrap();
    let create2_factory_address: &[u8; 20] = &(hex::decode(create2_factory).unwrap())[..20]
        .try_into()
        .unwrap();
    //print std::u64::MAX
    println!("{}", std::u64::MAX);
    let salt_step: u64 = (std::u64::MAX / thread_count as u64) - 1;
    let mut salt_from = 0_u64;
    let mut salt_to = salt_step;

    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    for i in 0..thread_count {
        let create2_factory_address_clone = create2_factory_address.clone();
        let code_hash_clone = code_hash.clone();
        let regex_rule_clone = regex_rule.clone();
        let handle = thread::spawn(move || {
            calc_address(
                salt_from,
                salt_to,
                &create2_factory_address_clone,
                &code_hash_clone,
                regex_rule_clone,
            );
        });
        handles.push(handle);
        salt_from = salt_to;
        if i < thread_count - 1 {
            salt_to += salt_step;
        }
    }
    // wait handles
    for handle in handles {
        handle.join().unwrap();
    }
    println!("end");
}
