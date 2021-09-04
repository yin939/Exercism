use std::{collections::HashMap, thread};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut rel: HashMap<char, usize> = HashMap::new();

    let chunk_size = if !input.is_empty() && input.len() % worker_count == 0 {
        input.len() / worker_count
    } else {
        input.len() / worker_count + 1
    };

    let chunks: Vec<Vec<String>> = input
        .chunks(chunk_size)
        .map(|chunk| chunk.iter().map(|s| s.to_lowercase()).collect())
        .collect();

    let mut workers = vec![];
    for chunk in chunks {
        workers.push(thread::spawn(move || handler(&chunk)));
    }


    for worker in workers {
        let worker_rel = worker.join().unwrap();
        for (k, v) in worker_rel {
            *rel.entry(k).or_default() += v;
        }
    }

    rel
}

pub fn handler(lines: &[String]) -> HashMap<char, usize> {
    let mut rel = HashMap::new();

    for line in lines {
        for c in line.chars().filter(|c| c.is_alphabetic()) {
            *rel.entry(c).or_default() += 1;
        }
    }

    rel
}
