use std::env;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
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

fn find_in_bucket(
    used_words: &[u64],
    n: usize,
    residue: usize,
    best: &AtomicUsize,
) -> Option<usize> {
    let mut x = first_candidate_in_bucket(n, residue);

    loop {
        if x >= best.load(Ordering::Relaxed) {
            return None;
        }

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
                best.fetch_min(x, Ordering::Relaxed);
                return Some(x);
            }
        }
    }
}

#[derive(Clone, Copy)]
struct BucketJob {
    residue: usize,
}

fn find_modular_bucket_term_multithreaded(
    used_words: &[u64],
    n: usize,
    thread_count: usize,
) -> usize {
    let worker_count = thread_count.min(n);
    let best = AtomicUsize::new(usize::MAX);

    thread::scope(|scope| {
        let (job_tx, job_rx) = mpsc::channel::<BucketJob>();
        let (result_tx, result_rx) = mpsc::channel::<Option<usize>>();
        let job_rx = Arc::new(Mutex::new(job_rx));
        let mut next_residue = 0;
        let mut active_jobs = 0;

        for _ in 0..worker_count {
            let job_rx = Arc::clone(&job_rx);
            let result_tx = result_tx.clone();
            let best = &best;

            scope.spawn(move || {
                loop {
                    let job = job_rx
                        .lock()
                        .expect("job receiver mutex must not be poisoned")
                        .recv();

                    match job {
                        Ok(job) => {
                            let result = find_in_bucket(used_words, n, job.residue, best);
                            result_tx
                                .send(result)
                                .expect("main thread must receive bucket result");
                        }
                        Err(_) => break,
                    }
                }
            });
        }

        drop(result_tx);

        while next_residue < n && active_jobs < worker_count {
            job_tx
                .send(BucketJob {
                    residue: next_residue,
                })
                .expect("worker threads must receive bucket jobs");
            next_residue += 1;
            active_jobs += 1;
        }

        while active_jobs > 0 {
            result_rx.recv().expect("worker thread must send result");

            active_jobs -= 1;

            if next_residue < n {
                job_tx
                    .send(BucketJob {
                        residue: next_residue,
                    })
                    .expect("worker threads must receive bucket jobs");
                next_residue += 1;
                active_jobs += 1;
            }
        }

        drop(job_tx);
    });

    let best = best.load(Ordering::Relaxed);
    assert!(best != usize::MAX, "at least one bucket must contain an admissible value");
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

fn generate_dense_modular_buckets_multithreaded(
    num_terms: usize,
    thread_count: usize,
) -> Vec<usize> {
    let mut used_words = Vec::new();
    let mut terms = Vec::new();

    for n in 1..=num_terms {
        let x = find_modular_bucket_term_multithreaded(&used_words, n, thread_count);
        mark_progression(&mut used_words, n, x);
        terms.push(x);
    }

    terms
}

fn time_generator(
    name: &str,
    num_terms: usize,
    generator: impl FnOnce(usize) -> Vec<usize>,
) -> Vec<usize> {
    let start = Instant::now();
    let terms = generator(num_terms);
    let seconds = start.elapsed().as_secs_f64();

    println!("{name}: {seconds:.4}s");
    println!("last term: {}", terms.last().unwrap());

    terms
}

fn parse_positive_usize(value: &str, name: &str) -> usize {
    let number = value
        .parse::<usize>()
        .unwrap_or_else(|_| panic!("{} must be a number", name));
    if number == 0 {
        eprintln!("{name} must be positive");
        std::process::exit(1);
    }
    number
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 || args.len() > 4 {
        eprintln!("Usage: modular_multithread_bench <term count> <thread count> [run|check]");
        std::process::exit(1);
    }

    let num_terms = parse_positive_usize(&args[1], "term count");
    let thread_count = parse_positive_usize(&args[2], "thread count");
    let mode = if args.len() == 4 { &args[3] } else { "run" };

    match mode {
        "run" => {
            time_generator(
                "rust dense modular buckets multithreaded",
                num_terms,
                |terms| generate_dense_modular_buckets_multithreaded(terms, thread_count),
            );
        }
        "check" => {
            let single_threaded = time_generator(
                "rust dense modular buckets",
                num_terms,
                generate_dense_modular_buckets,
            );
            let multithreaded = time_generator(
                "rust dense modular buckets multithreaded",
                num_terms,
                |terms| generate_dense_modular_buckets_multithreaded(terms, thread_count),
            );

            if single_threaded != multithreaded {
                for (index, (a, b)) in single_threaded.iter().zip(multithreaded.iter()).enumerate() {
                    if a != b {
                        eprintln!(
                            "mismatch at term {}: single-threaded={}, multithreaded={}",
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
            eprintln!("mode must be run or check");
            std::process::exit(1);
        }
    }
}
