// not the best solution. it should be easy to understand though

pub fn p031() {
    let mut possibilities = 0;
    // amount of 2pound coins
    for p200 in 0..=1 {
        let remnant = 200 - p200 * 200;
        // amount of 1pound coins
        for p100 in 0..=2 {
            let remnant = remnant - p100 * 100;
            // amount of 50p coins
            for p50 in 0..=4 {
                let remnant = remnant - p50 * 50;
                // amount of 20p coins
                for p20 in 0..=10 {
                    let remnant = remnant - p20 * 20;
                    // amount of 10p coins
                    for p10 in 0..=20 {
                        let remnant = remnant - p10 * 10;
                        // amount of 5p coins
                        for p5 in 0..=40 {
                            let remnant = remnant - p5 * 5;
                            // amount of 2p coins
                            for p2 in 0..=100 {
                                let remnant = remnant - p2 * 2;
                                if remnant >= 0 {
                                    possibilities += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{}", possibilities);
}
