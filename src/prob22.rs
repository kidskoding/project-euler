use std::fs;

pub fn prob22() -> u32 {
    let file_path = "./input/names.txt";
    let file = fs::read_to_string(file_path).unwrap();

    let mut names: Vec<String> = file
        .split(",")
        .map(|s| s.trim_matches('"').to_string())
        .collect();

    names.sort();

    let mut sum = 0;
    let mut count = 1;
    for name in names {
        let mut score: u32 = name
            .chars()
            .map(|c| (c as u32) - ('A' as u32) + 1)
            .sum();

        score *= count;
        count += 1;
        sum += score;
    }

    sum
}
