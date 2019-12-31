extern crate input;
extern crate intcode;

use intcode::{get_partitioned_codes, Instruction, OpCode};

fn main() {
    let inputs_raw = input::read_from_file("./input.txt");
    let codes = input::parse_as_vector_from_delimiter::<i32>(inputs_raw, ',');
    let first_value_after_transform = get_first_value_after_transform(&codes);
    println!(
        "First value after transform: {}",
        first_value_after_transform
    );
    let noun_verb_product = get_noun_verb_product(&codes);
    println!("Noun-verb product: {}", noun_verb_product);
}

fn get_first_value_after_transform(codes: &[i32]) -> i32 {
    let alarm_state_codes = get_at_alarm_state(&codes.to_vec(), 12, 2);
    let codes_after_execution = get_codes_after_execution(alarm_state_codes.to_vec());

    *codes_after_execution.first().expect("No codes!")
}

fn get_at_alarm_state(codes: &[i32], noun: i32, verb: i32) -> Vec<i32> {
    let mut alarm_state_codes = codes.to_vec();
    alarm_state_codes[1] = noun;
    alarm_state_codes[2] = verb;
    alarm_state_codes
}

fn get_codes_after_execution(mut codes: Vec<i32>) -> Vec<i32> {
    let num_in_a_sequence = 4;
    let mut partitioned_codes = get_partitioned_codes(&codes, num_in_a_sequence);
    for index in 0..partitioned_codes.len() {
        let partition = &partitioned_codes[index];

        let instruction = Instruction::new(partition);
        let first = codes[instruction.first_arg_position];
        let second = codes[instruction.second_arg_position];

        let op_result = match instruction.op_code {
            OpCode::Halt => None,
            OpCode::Add => Some(first + second),
            OpCode::Multiply => Some(first * second),
        };
        match op_result {
            Some(result) => {
                codes[instruction.result_position] = result;
                partitioned_codes = get_partitioned_codes(&codes, num_in_a_sequence);
            }
            None => break,
        };
    }
    get_flattened_codes(&partitioned_codes)
}

fn get_flattened_codes(partitioned_codes: &[Vec<i32>]) -> Vec<i32> {
    partitioned_codes.iter().flatten().copied().collect()
}

fn get_noun_verb_product(codes: &[i32]) -> i32 {
    let magic_number = 19_690_720;
    let mut noun_verb_product: i32 = 0;
    for noun in 0..101 {
        for verb in 0..101 {
            let alarm_state_codes = get_at_alarm_state(&codes, noun, verb);
            let codes_after_execution = get_codes_after_execution(alarm_state_codes.to_vec());
            let first: i32 = *codes_after_execution.first().expect("No codes!");
            if first == magic_number {
                noun_verb_product = 100 * noun + verb;
                break;
            }
        }
    }
    noun_verb_product
}
