use std::collections::HashSet;

use num_bigint::BigInt;

pub fn p029(){
    // using BigInt to calculate big integers ;)
    // save results in a hashset to avoid duplicates
    let mut sequence: HashSet<BigInt> = HashSet::new();
    for a in 2..=100 {
        for b in 2..=100 {
            sequence.insert(BigInt::from(a).pow(b));
        }
    }
    println!("{}", sequence.len());
}