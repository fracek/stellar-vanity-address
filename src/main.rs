extern crate shuttle_core;
extern crate clap;
extern crate num_cpus;

use std::thread;
use std::sync::mpsc;
use clap::{App, Arg};
use shuttle_core::KeyPair;

fn find_keypair(suffix: &str) -> KeyPair {
    loop {
        if let Ok(kp) = KeyPair::random() {
            if let Ok(pk) = kp.public_key().encode() {
                if pk.ends_with(suffix) {
                    return kp
                }
            }
        }
    }
}

fn spawn_producer(tx: mpsc::Sender<KeyPair>, suffix: String) {
    thread::spawn(move || {
        let keypair = find_keypair(&suffix);
        tx.send(keypair).unwrap();
    });
}

fn main() {
    let matches = App::new("Stellar Vanity Address Generator")
        .version("1.0")
        .author("Francesco Ceccon <francesco@ceccon.me>")
        .about("Generates Vanity Addresses for the Stellar Network")
        .arg(Arg::with_name("num_cpus")
             .short("c")
             .long("num-cpus")
             .help("Number of CPUs to use. Defaults to all available CPUs")
             .takes_value(true))
        .arg(Arg::with_name("SUFFIX")
             .required(true)
             .index(1))
        .get_matches();

    let (tx, rx) = mpsc::channel();

    let num_cpus = match matches.value_of("num_cpus") {
        None => num_cpus::get(),
        Some(n) => n.parse().unwrap(),
    };

    let suffix = matches.value_of("SUFFIX").unwrap().to_string();
    
    println!("Starting {} workers", num_cpus);
    
    for _ in 0..num_cpus {
        spawn_producer(tx.clone(), suffix.clone());
    }
    
    let recv = rx.recv().unwrap();
    let secret_key = recv.secret_key().unwrap().encode().unwrap();
    let public_key = recv.public_key().encode().unwrap();
    println!("SECRET: {}", secret_key);
    println!("PUBLIC: {}", public_key);
}
