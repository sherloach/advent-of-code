advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .map(filter(|nums| {
            let mut sorted = nums.clone();
            sorted.sort();
            sorted[0] + sorted[1] > sorted[2]
        }));

    // for line in lines {
    //     println!("{:?}", line);
    // }

    None
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
