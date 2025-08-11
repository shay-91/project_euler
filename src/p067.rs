use std::{cmp::max, fs};

pub fn p067() {
    // removed the last line from the input file to avoid an error
    let input = fs::read_to_string("src/067.txt").unwrap();
    // string => Vec<Vec<i32>>
    // split('\n') to split into rows
    // split(' ') to split into single numbers
    // parse::<i32>().unwrap() to prase from string to integer
    let mut input: Vec<Vec<i32>> = input
        .split('\n')
        .map(|x| x.split(' ').map(|x| x.parse::<i32>().unwrap()).collect())
        .collect();
    // bottom-up solution to reduce the amount of work
    // calculating the max of every 3-number triangle. example:
    //   63
    // 04  62
    // the maximum sum of this triangle is 63 + max(4, 62)
    for i in (0..input.len() - 1).rev() {
        for j in 0..input[i].len() {
            input[i][j] += max(input[i + 1][j], input[i + 1][j + 1]);
        }
    }
    println!("{}", input[0][0]);
}
