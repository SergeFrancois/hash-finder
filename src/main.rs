use clap::Parser;
use sha2::{Digest, Sha256};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Write;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;
use std::thread::available_parallelism;
use threadpool::ThreadPool;


#[derive(Parser, Debug)]
#[command(version, about = "A utility for searching for SHA-256 hashes with a specified number of 16-bit trailing zeros.", long_about = None)]
struct Args {
    #[arg(short = 'N', value_parser = clap::value_parser!(u8).range(1..=16), help = "Number of trailing zeros")]
    zero_count: u8,
    #[arg(short = 'F', default_value_t = 1, value_parser = clap::value_parser!(u32).range(1..), help = "Number of hashes")]
    hash_count: u32
}

struct FailedSearchResult {
    number: usize,
    thread_number: usize
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct SuccessfulSearchResult {
    number: usize,
    hash: String,
    thread_number: usize
}

enum SearchResult {
    Successful(SuccessfulSearchResult),
    Failed(FailedSearchResult)
}


fn has_trailing_zeros(data: &[u8], zero_count: usize) -> bool {
    let mut count = 0;
    for byte in data.iter().rev() {
        let count2 = if *byte == 0 { 2 } else { if *byte % 16 == 0 { 1 } else { 0 } };
        count += count2;
        if count2 < 2 {
            break;
        }
    }
    count == zero_count
}

fn main() {
    let args = Args::parse();
    let thread_count = available_parallelism().unwrap().get();
    let thread_pool = ThreadPool::new(thread_count);
    let need_stop_search = Arc::new(AtomicBool::new(false));
    let (tx, rx) = channel();
    for i in 0..thread_count {
        let tx = tx.clone();
        let need_stop_search = Arc::clone(&need_stop_search);
        let zero_count: usize = args.zero_count.into();
        thread_pool.execute(move || {
            let thread_number = i;
            let mut number: usize = i;
            while !need_stop_search.load(Ordering::Relaxed) {                
                let hash = Sha256::digest(number.to_le_bytes());
                if has_trailing_zeros(&hash, zero_count) {
                    let mut hash_as_hex = String::new();
                    let _ = write!(hash_as_hex, "{:02x}", hash);
                    let _ = tx.send(SearchResult::Successful(
                        SuccessfulSearchResult{
                            number: number,
                            hash: hash_as_hex,
                            thread_number: thread_number
                        }
                    ));
                } else {
                    let _ = tx.send(SearchResult::Failed(
                        FailedSearchResult{
                            number: number,
                            thread_number: thread_number
                        }
                    ));
                }
                number += thread_count;
            }
        });
    }
    drop(tx);
    let mut numbers: Vec<usize> = vec![0; thread_count];
    let mut results: BinaryHeap<Reverse<SuccessfulSearchResult>> = BinaryHeap::new();
    let mut count = 0;
    'main_loop: while let Ok(result) = rx.recv() {
        match result {
            SearchResult::Successful(r) => {
                numbers[r.thread_number] = r.number;
                results.push(Reverse(r));
            },
            SearchResult::Failed(r) => numbers[r.thread_number] = r.number
        }
        let min_number = *numbers.iter().min().unwrap();
        while !results.is_empty() &&
              min_number >= results.peek().unwrap().0.number
        {
            let result = results.pop().unwrap().0;
            println!("{0}, {1}", result.number, result.hash);
            count += 1;
            if count >= args.hash_count {
                need_stop_search.store(true, Ordering::Relaxed);
                break 'main_loop;
            }
        }
    }
    thread_pool.join();
}