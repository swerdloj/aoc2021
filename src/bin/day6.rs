fn main() {
    let input = aoc2021::read_input(6);

    let mut bins = [0u64; 9];
    for days_left in input.split(',') {
        let days = days_left.parse::<usize>().unwrap();

        bins[days] += 1;
    }

    for day in 1..=256 {
        let mut new = 0;

        for days_left in 0..=8 {
            if days_left == 0 {
                new = bins[days_left];
            }

            if days_left < 8 {
                bins[days_left] = bins[days_left + 1];
            }
        }

        // The new fish
        bins[8] = new;
        // The fish that just reproduced
        bins[6] += new;

        // PART 1 //
        if day == 80 {
            println!("Part 1: {}", bins.iter().sum::<u64>());
        }
    }
    
    println!("Part 2: {}", bins.iter().sum::<u64>());
}