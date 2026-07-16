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

fn mark_progression(words: &mut Vec<u64>, n: usize, x: usize) {
    for k in 0..n {
        dense_set(words, x + k * n);
    }
}

fn generate_dense_bruteforce(num_terms: usize) -> Vec<usize> {
    let mut used_words = Vec::new();
    let mut terms = Vec::new();

    for n in 1..=num_terms {
        let mut x = 1_usize;

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

            mark_progression(&mut used_words, n, x);
            terms.push(x);
            break;
        }
    }

    terms
}

fn first_candidate_in_bucket(n: usize, residue: usize) -> usize {
    if residue == 0 {
        n
    } else {
        residue
    }
}

fn find_modular_bucket_term(used_words: &[u64], n: usize) -> usize {
    let mut best = usize::MAX;

    for residue in 0..n {
        let mut x = first_candidate_in_bucket(n, residue);

        while x < best {
            let mut colliding_k = None;

            for k in (0..n).rev() {
                if dense_get(used_words, x + k * n) {
                    colliding_k = Some(k);
                    break;
                }
            }

            match colliding_k {
                Some(k) => x += (k + 1) * n,
                None => {
                    best = x;
                    break;
                }
            }
        }
    }

    best
}

fn generate_dense_modular_buckets(num_terms: usize) -> Vec<usize> {
    let mut used_words = Vec::new();
    let mut terms = Vec::new();

    for n in 1..=num_terms {
        let x = find_modular_bucket_term(&used_words, n);
        mark_progression(&mut used_words, n, x);
        terms.push(x);
    }

    terms
}

fn time_generator(name: &str, num_terms: usize, generator: fn(usize) -> Vec<usize>) -> Vec<usize> {
    let start = Instant::now();
    let terms = generator(num_terms);
    let seconds = start.elapsed().as_secs_f64();

    println!("{name}: {seconds:.4}s");
    println!("last term: {}", terms.last().unwrap());

    terms
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        eprintln!("Usage: rust_modular_bucket_bench <term count> [all|bruteforce|modular|check]");
        std::process::exit(1);
    }

    let num_terms = args[1]
        .parse::<usize>()
        .expect("term count must be a number");
    if num_terms == 0 {
        eprintln!("term count must be positive");
        std::process::exit(1);
    }

    let which = if args.len() == 3 { &args[2] } else { "all" };

    match which {
        "all" => {
            time_generator(
                "rust dense bruteforce",
                num_terms,
                generate_dense_bruteforce,
            );
            time_generator(
                "rust dense modular buckets",
                num_terms,
                generate_dense_modular_buckets,
            );
        }
        "bruteforce" => {
            time_generator(
                "rust dense bruteforce",
                num_terms,
                generate_dense_bruteforce,
            );
        }
        "modular" => {
            time_generator(
                "rust dense modular buckets",
                num_terms,
                generate_dense_modular_buckets,
            );
        }
        "check" => {
            let brute = time_generator(
                "rust dense bruteforce",
                num_terms,
                generate_dense_bruteforce,
            );
            let modular = time_generator(
                "rust dense modular buckets",
                num_terms,
                generate_dense_modular_buckets,
            );

            if brute != modular {
                for (index, (a, b)) in brute.iter().zip(modular.iter()).enumerate() {
                    if a != b {
                        eprintln!(
                            "mismatch at term {}: bruteforce={}, modular={}",
                            index + 1,
                            a,
                            b
                        );
                        break;
                    }
                }
                std::process::exit(1);
            }

            println!("terms match");
        }
        _ => {
            eprintln!("algorithm choice must be all, bruteforce, modular, or check");
            std::process::exit(1);
        }
    }
}
