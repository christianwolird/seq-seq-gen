use std::env;
use std::time::Instant;

fn dense_get(words: &[u64], bit: usize) -> bool {
    let word = bit / 64;
    word < words.len() && (words[word] & (1_u64 << (bit % 64))) != 0
}

fn dense_set(words: &mut Vec<u64>, bit: usize) {
    let word = bit / 64;
    if word >= words.len() {
        words.resize(word + 1, 0);
    }
    words[word] |= 1_u64 << (bit % 64);
}

fn generate_dense(num_terms: usize) -> Vec<usize> {
    let mut used_words = Vec::new();
    let mut terms = Vec::new();

    for n in 1..=num_terms {
        let mut x = 1;
        loop {
            let mut collision = false;

            for k in 0..n {
                if dense_get(&used_words, x + k * n) {
                    collision = true;
                    break;
                }
            }

            if collision {
                x += 1;
                continue;
            }

            for k in 0..n {
                dense_set(&mut used_words, x + k * n);
            }
            terms.push(x);
            break;
        }
    }

    terms
}

fn time_generator(name: &str, num_terms: usize, generator: fn(usize) -> Vec<usize>) {
    let start = Instant::now();
    let terms = generator(num_terms);
    let seconds = start.elapsed().as_secs_f64();

    println!("{name}: {seconds:.4}s");
    println!("last term: {}", terms.last().unwrap());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: rust_dense_bitmap_bench <term count>");
        std::process::exit(1);
    }

    let num_terms = args[1].parse::<usize>().expect("term count must be a number");
    if num_terms == 0 {
        eprintln!("term count must be positive");
        std::process::exit(1);
    }

    time_generator("rust dense bitmap", num_terms, generate_dense);
}
