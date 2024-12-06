advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let count = input
        .lines()
        .filter(|line| {
            let nums: Vec<u32> = line
                .split(" ")
                .map(|num| num.parse::<u32>().unwrap())
                .collect();

            is_safe(&nums)
        })
        .count();

    Some(count as u32)
}

fn is_safe(nums: &[u32]) -> bool {
    let is_increasing = nums.windows(2).all(|w| w[1] > w[0]);
    let is_decreasing = nums.windows(2).all(|w| w[1] < w[0]);

    let valid_adjacent = nums.windows(2).all(|w| {
        let diff = w[1].abs_diff(w[0]);
        diff >= 1 && diff <= 3
    });

    (is_increasing || is_decreasing) && valid_adjacent
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
