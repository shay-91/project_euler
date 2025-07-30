use std::collections::HashMap;

pub fn p014(){
    // 'starting_number' contains the number producing the longest chain so far
    let mut starting_number: u64 = 1;
    // 'maximum' is the length of the longest chain so far
    let mut maximum: u32 = 1;
    // using a hashmap to avoid calculating chain lengths multiple times
    let mut chains: HashMap<u64, u32> = HashMap::new();
    // the chain length for the starting number 1 is 1
    chains.insert(1, 1);
    for i in 2..1_000_000 {
        // save the sequence in a vector ...
        let mut sequence = vec![i as u64];
        // until you find a number which is already in the hashmap
        while ! chains.contains_key(&sequence[sequence.len()-1]) {
            let n = sequence[sequence.len()-1];
            // calculate the next number of the sequence
            if n % 2 == 0 {
                sequence.push(n/2);
            }else {
                sequence.push(3 * n + 1);
            }
        }
        // calculate the chain length and add it to the hashmap
        let mut length = *chains.get(&sequence.pop().unwrap()).unwrap();
        while sequence.len() > 1 {
            length += 1;
            chains.insert(sequence.pop().unwrap(), length);
        }
        length += 1;
        // i don't know why i need this if. should never be true...
        if sequence.is_empty() {
            continue;
        }
        // update 'starting_number' and 'maximum' if a larger chain length was found
        let n = sequence.pop().unwrap();
        chains.insert(n, length);
        if length > maximum {
            maximum = length;
            starting_number = n;
        }
    }
    println!("{}", starting_number);
}