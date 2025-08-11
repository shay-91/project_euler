use std::cmp::max;

pub fn p018() {
    let input = "75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23"
        .to_string();
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
