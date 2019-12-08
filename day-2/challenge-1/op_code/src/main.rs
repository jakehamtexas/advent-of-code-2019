use std::fs;

fn main() {
    let inputs_raw = read_from_file();
    let program_instructions = parse_as_vector(inputs_raw);
    let first_value_after_transform = get_first_value_after_transform(program_instructions);
    println!("{}", first_value_after_transform);
}

fn read_from_file() -> String {
    return fs::read_to_string("../../input.txt").expect("input.txt doesn't exist!");
}

fn parse_as_vector(inputs_raw: String) -> Vec<i32> {
    return inputs_raw.split(",").map(|m| m.parse().unwrap()).collect();
}

fn get_first_value_after_transform(program_instructions: Vec<i32>) -> i32 {
    let alarm_state_program_instructions = get_at_alarm_state(program_instructions);
    let program_instructions_after_execution =
        get_program_instructions_after_execution(alarm_state_program_instructions);

    return *program_instructions_after_execution
        .first()
        .expect("Program instructions are empty!");
}

fn get_at_alarm_state(program_instructions: Vec<i32>) -> Vec<i32> {
    let mut alarm_state_program_instructions = program_instructions.clone();
    alarm_state_program_instructions[1] = 12;
    alarm_state_program_instructions[2] = 2;
    return alarm_state_program_instructions;
}

fn get_program_instructions_after_execution(mut program_instructions: Vec<i32>) -> Vec<i32> {
    let mut partitioned_program_instructions =
        get_partitioned_program_instructions(&program_instructions);
    for index in 0..partitioned_program_instructions.len() {
        let current_partition_instructions = &partitioned_program_instructions[index];
        let (op_code, first_op_code_arg_index, second_op_code_arg_index, op_code_change_index) = (
            current_partition_instructions[0],
            current_partition_instructions[1] as usize,
            current_partition_instructions[2] as usize,
            current_partition_instructions[3] as usize,
        );

        let first_op_code_arg = program_instructions[first_op_code_arg_index];
        let second_op_code_arg = program_instructions[second_op_code_arg_index];

        let halt_op_code = 99;
        let add_op_code = 1;
        let multiply_op_code = 2;
        let op_result: i32;
        match op_code {
            halt if halt == halt_op_code => break,
            add if add == add_op_code => op_result = first_op_code_arg + second_op_code_arg,
            multiply if multiply == multiply_op_code => {
                op_result = first_op_code_arg * second_op_code_arg
            }
            _ => panic!("Code is not recognized!"),
        }
        program_instructions[op_code_change_index] = op_result;
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
