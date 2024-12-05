fn main() {
    let input = include_str!("input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    let res: u32 = input
        .lines()
        .filter(|line| {
            let nums: Vec<u32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            match is_valid_line(&nums) {
                true => true,
                false => {
                    (0..nums.len()).any(|i| {
                        let mut reduced = nums.clone();
                        reduced.remove(i);
                        is_valid_line(&reduced)
                    })
                },
            }


        })
        .count() as u32;
    return res;
}

fn is_valid_line(nums: &Vec<u32>) -> bool {
    let is_ascending = nums.windows(2).all(|w| {w[0] < w[1] && w[0].abs_diff(w[1]) <= 3});
    let is_descending = nums.windows(2).all(|w| {w[0] > w[1] && w[0].abs_diff(w[1]) <= 3});

    is_ascending || is_descending
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2("7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9");
        assert_eq!(result, 4);
    }
}