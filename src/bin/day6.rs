fn main() {
    let input = aoc2021::read_input(6);

    let mut bins = [0u64; 9];
    for days_left in input.split(',') {
        let days = days_left.parse::<usize>().unwrap();

        bins[days] += 1;
    }

    // NOTE: This is a naive solution -- reinterpreting the indices would remove the need for folding
    for day in 0..256 {
        let mut new = 0;

        bins = bins.into_iter().enumerate().fold(bins, |mut bins, (days_left, count)| {
            if days_left == 0 {
                new = count
            }

            if days_left < 8 {
                bins[days_left] = bins[days_left + 1]
            }
            
            bins
        });

        // The new fish
        bins[8] = new;
        // The fish that just reproduced
        bins[6] += new;

        // PART 1 //
        if day == 79 {
            println!("Part 1: {}", bins.iter().sum::<u64>());
        }
    }
    
    println!("Part 2: {}", bins.iter().sum::<u64>());
}