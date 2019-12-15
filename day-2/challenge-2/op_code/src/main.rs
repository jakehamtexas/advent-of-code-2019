use std::fs;

mod models;
use models::Instruction;
use models::OpCode;

fn main() {
    let inputs_raw = read_from_file();
    let program_instructions = parse_as_vector(inputs_raw);
    let first_value_after_transform = get_noun_verb_product(program_instructions);
    println!("{}", first_value_after_transform);
}

fn read_from_file() -> String {
    return fs::read_to_string("../../input.txt").expect("input.txt doesn't exist!");
}

fn parse_as_vector(inputs_raw: String) -> Vec<i32> {
    return inputs_raw.split(",").map(|m| m.parse().unwrap()).collect();
}

fn get_noun_verb_product(program_instructions: Vec<i32>) -> i32 {
    let magic_number = 19690720;
    let mut noun_verb_product: i32 = 0;
    for noun in 0..101 {
        for verb in 0..101 {
            let alarm_state_program_instructions =
                get_at_alarm_state(&program_instructions, noun, verb);
            let program_instructions_after_execution =
                get_program_instructions_after_execution(alarm_state_program_instructions);
            let first: i32 = *program_instructions_after_execution
                .first()
                .expect("Program instructions are empty!");
            if first == magic_number {
                noun_verb_product = 100 * noun + verb;
                break;
            }
        }
    }
    return noun_verb_product;
}

fn get_at_alarm_state(program_instructions: &Vec<i32>, noun: i32, verb: i32) -> Vec<i32> {
    let mut alarm_state_program_instructions = program_instructions.clone();
    alarm_state_program_instructions[1] = noun;
    alarm_state_program_instructions[2] = verb;
    return alarm_state_program_instructions;
}

fn get_program_instructions_after_execution(mut program_instructions: Vec<i32>) -> Vec<i32> {
    let mut partitioned_program_instructions =
        get_partitioned_program_instructions(&program_instructions);
    for index in 0..partitioned_program_instructions.len() {
        let current_partition_instructions = &partitioned_program_instructions[index];
        let instruction = Instruction {
            op_code: OpCode::from_i32(current_partition_instructions[0]),
            first_arg_position: current_partition_instructions[1] as usize,
            second_arg_position: current_partition_instructions[2] as usize,
            result_position: current_partition_instructions[3] as usize,
        };

        let first_arg = program_instructions[instruction.first_arg_position];
        let second_arg = program_instructions[instruction.second_arg_position];

        let op_result: i32;
        match instruction.op_code {
            OpCode::Halt => break,
            OpCode::Add => op_result = first_arg + second_arg,
            OpCode::Multiply => op_result = first_arg * second_arg,
        }
        program_instructions[instruction.result_position] = op_result;
        partitioned_program_instructions =
            get_partitioned_program_instructions(&program_instructions);
    }
    return get_flattened_program_instructions(&partitioned_program_instructions);
}

fn get_partitioned_program_instructions(program_instructions: &Vec<i32>) -> Vec<Vec<i32>> {
    let num_instructions_in_a_sequence = 4;
    return program_instructions
        .chunks(num_instructions_in_a_sequence)
        .map(|chunk| chunk.to_vec())
        .collect();
}

fn get_flattened_program_instructions(
    partitioned_program_instructions: &Vec<Vec<i32>>,
) -> Vec<i32> {
    return partitioned_program_instructions
        .iter()
        .flatten()
        .map(|program_instruction| *program_instruction)
        .collect();
}
