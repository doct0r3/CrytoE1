use sha1::{Sha1, Digest};
use itertools::Itertools;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();
    let hash1 = "67ae1a64661ac8b4494666f58c4822408dd0a3e4";
    let str2 = vec![
        vec!['Q', 'q'], vec!['W', 'w'], vec!['%', '5'], vec!['8', '('],
        vec!['=', '0'], vec!['I', 'i'], vec!['*', '+'], vec!['n', 'N']
    ];

    for a in 0..2 {
        for b in 0..2 {
            for c in 0..2 {
                for d in 0..2 {
                    for e in 0..2 {
                        for f in 0..2 {
                            for g in 0..2 {
                                for h in 0..2 {
                                    let new_s = vec![
                                        str2[0][a], str2[1][b], str2[2][c], str2[3][d],
                                        str2[4][e], str2[5][f], str2[6][g], str2[7][h]
                                    ];

                                    for perm in new_s.iter().permutations(8) {
                                        let perm_string: String = perm.into_iter().collect();
                                        let encrypted = sha1_encrypt(&perm_string);
                                        if encrypted == hash1 {
                                            println!("{}", perm_string);
                                            let duration = start_time.elapsed();
                                            println!("Time elapsed: {} seconds", duration.as_secs());
                                            return;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn sha1_encrypt(input: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}