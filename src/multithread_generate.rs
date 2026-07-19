use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};
use std::path::Path;
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

fn load_existing_terms(seq_file: &Path, words: &mut Vec<u64>) -> std::io::Result<usize> {
    if !seq_file.exists() {
        return Ok(0);
    }

    let file = File::open(seq_file)?;
    let mut reader = BufReader::new(file);
    let mut last_good_offset = 0_u64;
    let mut current_offset = 0_u64;
    let mut expected_n = 1_usize;
    let mut line = String::new();

    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }

        current_offset += bytes_read as u64;
        let trimmed = line.trim_end();
        if trimmed.is_empty() {
            break;
        }

        let mut parts = trimmed.split_whitespace();
        let Some(n_text) = parts.next() else { break };
        let Some(x_text) = parts.next() else { break };

        if parts.next().is_some() {
            break;
        }

        let Ok(n) = n_text.parse::<usize>() else {
            break;
        };
        let Ok(x) = x_text.parse::<usize>() else {
            break;
        };

        if n != expected_n {
            break;
        }

        mark_progression(words, n, x);
        expected_n += 1;
        last_good_offset = current_offset;
    }

    let loaded_terms = expected_n - 1;
    let mut file = OpenOptions::new().write(true).open(seq_file)?;
    file.set_len(last_good_offset)?;
    file.seek(SeekFrom::End(0))?;

    Ok(loaded_terms)
}

fn append_term(file: &mut File, n: usize, x: usize) -> std::io::Result<()> {
    writeln!(file, "{n} {x}")?;
    file.flush()?;
    file.sync_all()?;
    Ok(())
}

fn first_candidate_in_bucket(n: usize, residue: usize) -> usize {
    if residue == 0 {
        n
    } else {
        residue
    }
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

fn find_modular_term(used_words: &[u64], n: usize, thread_count: usize) -> usize {
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

            scope.spawn(move || loop {
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
    assert!(
        best != usize::MAX,
        "at least one bucket must contain an admissible value"
    );
    best
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: multithread_generate <target number of terms>");
        std::process::exit(1);
    }

    let target_terms = args[1]
        .parse::<usize>()
        .expect("target number of terms must be a number");

    if target_terms < 1 {
        eprintln!("target number of terms must be positive");
        std::process::exit(1);
    }

    let thread_count = thread::available_parallelism().map_or(1, usize::from);
    let repo_root = env::current_dir()?;
    let results_dir = repo_root.join("results");
    let seq_file = results_dir.join("sequence.txt");

    std::fs::create_dir_all(&results_dir)?;

    println!("Loading previous progressions...");
    let mut used_words = Vec::new();
    let loaded_terms = load_existing_terms(&seq_file, &mut used_words)?;
    println!("Loading complete. Using {thread_count} threads.");

    if target_terms <= loaded_terms {
        println!("Sequence already has {loaded_terms} terms.");
        return Ok(());
    }

    println!(
        "Generating terms {} through {}...",
        loaded_terms + 1,
        target_terms
    );

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&seq_file)?;

    for n in (loaded_terms + 1)..=target_terms {
        let term_start = Instant::now();
        let x = find_modular_term(&used_words, n, thread_count);

        mark_progression(&mut used_words, n, x);
        append_term(&mut file, n, x)?;

        let seconds = term_start.elapsed().as_secs_f64();
        println!("Sequoia({n}) = {x}  [{seconds:.1}s]");
    }

    println!("Done.");
    Ok(())
}
