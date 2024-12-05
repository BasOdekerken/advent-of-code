use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),\s*(\d+)\)").unwrap();
    
    let res = re.captures_iter(input)
        .fold((true, 0), |(enabled, total), cap| {
            match cap.get(0).unwrap().as_str() {
                "do()" => (true, total),
                "don't()" => (false, total),
                _ => {
                    let a: u32 = cap[1].parse().unwrap();
                    let b: u32 = cap[2].parse().unwrap();
                    if enabled {
                        (enabled, total + a * b)
                    } else {
                        (enabled, total)
                    }
                }
            }
    });
    return res.1
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn it_works() {
        let result = part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, 48);
    }
}