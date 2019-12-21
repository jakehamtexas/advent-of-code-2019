use std::fs;

fn main() {
    let range = parse_as_range(&read_from_file());
    let password_possibilities = get_password_possibilities(&range);
    let even_harder_password_possibilities = get_even_harder_password_possibilities(&range);
    println!("{}", password_possibilities);
    println!("{}", even_harder_password_possibilities);
}

fn read_from_file() -> String {
    String::from(fs::read_to_string("./input.txt").expect("input.txt doesn't exist!"))
}

fn parse_as_range(input: &str) -> (u32, u32) {
    let parts = input.split('-').collect::<Vec<&str>>();
    (parse_as_u32(parts[0]), parse_as_u32(parts[1]))
}

fn parse_as_u32(input: &str) -> u32 {
    input.parse::<u32>().unwrap()
}

fn get_password_possibilities(range: &(u32, u32)) -> u32 {
    let mut count = 0;
    for iter in range.0..range.1 {
        count += if get_has_adjacent_same_digits(iter) && !get_has_decreasing(iter) {
            1
        } else {
            0
        };
    }
    count
}

fn get_even_harder_password_possibilities(range: &(u32, u32)) -> u32 {
    let mut count = 0;
    for iter in range.0..range.1 + 1 {
        count += if get_has_adjacent_same_digits(iter)
            && get_has_even_amount_of_grouped_digits(iter)
            && !get_has_decreasing(iter)
        {
            1
        } else {
            0
        };
    }
    count
}

fn get_has_adjacent_same_digits(iter: u32) -> bool {
    let nums = get_as_nums(iter);
    for (i, num) in nums.iter().enumerate() {
        if i > 0 && *num == nums[i - 1] {
            return true;
        }
    }
    false
}

fn get_has_decreasing(iter: u32) -> bool {
    let nums = get_as_nums(iter);
    for (i, num) in nums.iter().enumerate() {
        if i > 0 && *num < nums[i - 1] {
            return true;
        }
    }
    false
}

fn get_has_even_amount_of_grouped_digits(iter: u32) -> bool {
    let nums = get_as_nums(iter);
    get_contiguous_slices(nums)
        .into_iter()
        .map(|slice| slice.len())
        .any(|len| len == 2)
}

fn get_contiguous_slices(nums: Vec<u8>) -> Vec<Vec<u8>> {
    let mut slices: Vec<Vec<u8>> = Vec::new();
    let mut slice: Vec<u8> = Vec::new();
    let mut current_number = 0;
    for num in nums.into_iter() {
        if num != current_number {
            current_number = num;
            slices.push(slice);
            slice = Vec::new();
        }
        slice.push(num);
    }
    slices.push(slice);
    slices
}

fn get_as_nums(iter: u32) -> Vec<u8> {
    iter.to_string()
        .chars()
        .map(|digit| digit.to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>()
}
