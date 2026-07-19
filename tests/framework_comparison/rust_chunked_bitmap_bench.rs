use std::collections::HashMap;
use std::env;
use std::time::Instant;

fn chunked_get(words: &HashMap<usize, u64>, bit: usize) -> bool {
    let word = bit / 64;
    match words.get(&word) {
        Some(bits) => (bits & (1_u64 << (bit % 64))) != 0,
        None => false,
    }
}

fn chunked_set(words: &mut HashMap<usize, u64>, bit: usize) {
    let word = bit / 64;
    let bits = words.entry(word).or_insert(0);
    *bits |= 1_u64 << (bit % 64);
}

fn generate_chunked(num_terms: usize) -> Vec<usize> {
    let mut used_words = HashMap::new();
    let mut terms = Vec::new();

    for n in 1..=num_terms {
        let mut x = 1;
        loop {
            let mut collision = false;

            for k in 0..n {
                if chunked_get(&used_words, x + k * n) {
                    collision = true;
                    break;
                }
            }

            if collision {
                x += 1;
                continue;
            }

            for k in 0..n {
                chunked_set(&mut used_words, x + k * n);
            }
            terms.push(x);
            break;
        }
    }

    terms
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: rust_chunked_bitmap_bench <term count>");
        std::process::exit(1);
    }

    let num_terms = args[1].parse::<usize>().expect("term count must be a number");
    if num_terms == 0 {
        eprintln!("term count must be positive");
        std::process::exit(1);
    }

    let start = Instant::now();
    let terms = generate_chunked(num_terms);
    let seconds = start.elapsed().as_secs_f64();

    println!("rust chunked bitmap: {seconds:.4}s");
    println!("last term: {}", terms.last().unwrap());
}
