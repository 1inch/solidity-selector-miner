use tiny_keccak::{Keccak, Hasher};
use std::thread;
use std::time::Instant;
use structopt::StructOpt;

extern crate hex;

#[derive(StructOpt)]
struct Args {
    selector: String,
    params: String,
    #[structopt(default_value = "1")]
    threads: usize,
}

fn main() {
    let args = Args::from_args();

    let alphabet: [u8; 62] = [
        '0' as u8, '1' as u8, '2' as u8, '3' as u8, '4' as u8, '5' as u8, '6' as u8, '7' as u8,
        '8' as u8, '9' as u8, 'a' as u8, 'b' as u8, 'c' as u8, 'd' as u8, 'e' as u8, 'f' as u8,
        'g' as u8, 'h' as u8, 'i' as u8, 'j' as u8, 'k' as u8, 'l' as u8, 'm' as u8, 'n' as u8,
        'o' as u8, 'p' as u8, 'q' as u8, 'r' as u8, 's' as u8, 't' as u8, 'u' as u8, 'v' as u8,
        'w' as u8, 'x' as u8, 'y' as u8, 'z' as u8, 'A' as u8, 'B' as u8, 'C' as u8, 'D' as u8,
        'E' as u8, 'F' as u8, 'G' as u8, 'H' as u8, 'I' as u8, 'J' as u8, 'K' as u8, 'L' as u8,
        'M' as u8, 'N' as u8, 'O' as u8, 'P' as u8, 'Q' as u8, 'R' as u8, 'S' as u8, 'T' as u8,
        'U' as u8, 'V' as u8, 'W' as u8, 'X' as u8, 'Y' as u8, 'Z' as u8,
    ];

    let mut target = [0u8; 4];
    target.copy_from_slice(
        &hex::decode(args.selector)
            .unwrap()
            .drain(0..4)
            .collect::<Vec<_>>(),
    );
    
    let params_length = args.params.len();
    let mut params = [[0u8; 32]; 100];
    for i in 0..params_length/32 {
        params[i].copy_from_slice(&args.params.as_bytes()[i*32..(i+1)*32]);
    }
    if params_length % 32 > 0 {
        params[params_length/32][0..params_length%32].copy_from_slice(
            &args.params.as_bytes()[params_length/32*32..params_length]
        );
    }

    let args_threads = args.threads;
    let mut handles = vec![];
    for ti in 0..args.threads {
        handles.push(Some(thread::spawn(move || {
            let mut index = 0;
            let mut reported_index = 0;
            let mut last = Instant::now();
            let first = last;
            
            for i1 in 0..alphabet.len() {
                for i2 in 0..alphabet.len() {
                    for i3 in 0..alphabet.len() {
                        for i4 in 0..alphabet.len() {
                            for i5 in 0..alphabet.len() {
                                for i6 in 0..alphabet.len() {
                                    index += 1;
                                    let ms = last.elapsed().as_millis() as u64;
                                    if ms > 3000 {
                                        println!(
                                            "Thread #{:x}: iteration {}M ({} MH/s)\r",
                                            ti,
                                            (index / 1000) as f64 / 1000.0,
                                            ((index - reported_index) / (1 + ms)) as f64 / 1000.0
                                        );
                                        last = Instant::now();
                                        reported_index = index
                                    }

                                    let mut hasher = Keccak::v256();
                                    hasher.update(&[
                                        'f' as u8,
                                        'u' as u8,
                                        'n' as u8,
                                        'c' as u8,
                                        '_' as u8,
                                        alphabet[ti],
                                        alphabet[i1],
                                        alphabet[i2],
                                        alphabet[i3],
                                        alphabet[i4],
                                        alphabet[i5],
                                        alphabet[i6],
                                    ]);
                                    for i in 0..params_length/32 {
                                        hasher.update(&params[i]);
                                    }
                                    for i in 0..params_length%32 {
                                        hasher.update(&[params[params_length/32][i]]);
                                    }

                                    let mut res = [0u8; 4];
                                    hasher.finalize(&mut res);
                                    if &res[0..4] == &target[0..4] {
                                        println!(
                                            "Found signature func_{} in {} seconds after {}M iterations",
                                            String::from_utf8(vec![
                                                alphabet[ti],
                                                alphabet[i1],
                                                alphabet[i2],
                                                alphabet[i3],
                                                alphabet[i4],
                                                alphabet[i5],
                                                alphabet[i6],
                                            ])
                                            .unwrap(),
                                            first.elapsed().as_secs(),
                                            (index * args_threads as u64 / 1000) as f64 / 1000.0
                                        );
                                        std::process::exit(0);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })));
    }

    for i in 0..handles.len() {
        handles[i].take().map(std::thread::JoinHandle::join);
    }
}
