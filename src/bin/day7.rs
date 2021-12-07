fn main() {
    println!("Part 1: {}", solve(true));
    println!("Part 2: {}", solve(false));
}

fn solve(part1: bool) -> i32 {
    let input = aoc2021::read_input(7);

    // (Position -> Count)
    let mut positions = std::collections::HashMap::new();

    let mut max = 0;
    let mut min = i32::MAX;

    for position in input.split(',') {
        let position = position.parse::<i32>().unwrap();

        max = max.max(position);
        min = min.min(position);

        let count = positions.entry(position).or_insert(0);
        *count += 1;
    }

    // TODO: Use a search instead of naive iteration
    (min..=max).map(|target| {
        positions.iter().map(|(position, count)| {
            let distance = (target - *position).abs();

            let fuel_needed = if part1 {
                distance
            } else {
                // Costs one more each time -> series simplificiation
                distance * (distance + 1) / 2
            };

            *count * fuel_needed
        }).sum::<i32>()
    })
    .min()
    .unwrap()
}