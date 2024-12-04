use std::{
    collections::{BinaryHeap, HashMap},
    ops::Mul,
};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (left_heap, right_heap) = input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(l, r)| (l.parse::<u32>().unwrap(), r.parse().unwrap()))
        .fold(
            (BinaryHeap::new(), BinaryHeap::new()),
            |(mut acc_l, mut acc_r), (l, r)| {
                acc_l.push(l);
                acc_r.push(r);
                (acc_l, acc_r)
            },
        );

    Some(
        left_heap
            .into_sorted_vec()
            .iter()
            .zip(right_heap.into_sorted_vec().iter())
            .map(|(l, r)| l.abs_diff(*r))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(l, r)| (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap()))
        .fold(
            (Vec::new(), HashMap::new()),
            |(mut vec, mut hashmap), (l, r)| {
                vec.push(l);
                hashmap.insert(r, hashmap.get(&r).unwrap_or(&0u32) + 1);
                (vec, hashmap)
            },
        );
    left.iter()
        .map(|l| l.mul(right.get(l).unwrap_or(&0)))
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
