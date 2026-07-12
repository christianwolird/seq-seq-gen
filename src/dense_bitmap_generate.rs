use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};
use std::path::Path;
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

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: dense_bitmap_generate <target number of terms>");
        std::process::exit(1);
    }

    let target_terms = args[1]
        .parse::<usize>()
        .expect("target number of terms must be a number");

    if target_terms < 1 {
        eprintln!("target number of terms must be positive");
        std::process::exit(1);
    }

    let repo_root = env::current_dir()?;
    let results_dir = repo_root.join("results");
    let seq_file = results_dir.join("sequence.txt");

    std::fs::create_dir_all(&results_dir)?;

    println!("Loading previous progressions...");

    let mut used_words = Vec::new();
    let loaded_terms = load_existing_terms(&seq_file, &mut used_words)?;

    println!("Loading complete.");

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
            append_term(&mut file, n, x)?;

            let seconds = term_start.elapsed().as_secs_f64();
            println!("Sequoia({n}) = {x}  [{seconds:.1}s]");
            break;
        }
    }

    println!("Done.");

    Ok(())
}
