pub fn read_input(day: u32) -> String {
    std::fs::read_to_string(format!("./inputs/day{}.txt", day))
        .expect("No input file found")
}