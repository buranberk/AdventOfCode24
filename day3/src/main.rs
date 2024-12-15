
fn main() {
    // reat the memort file and store it in a string
    let memory = std::fs::read_to_string("data/input.txt").unwrap();

    // parse the memory string to get the total result
    let total_result = parse_mul_instructions(&memory);
    println!("The total result is: {}", total_result);

    // parse the memory string with the conditional instructions
    let total_result_conditional = parse_mul_instructions_conditional(&memory);
    println!("The total result with conditional instructions is: {}", total_result_conditional);

}

// Function to detect all substrings in the form mul(x,y) where x and y are integers
fn parse_mul_instructions(instructions: &str) -> i32 {
    let mut total_instruction_result = 0;

    // define the values to be used during the parsing
    let mut x_str = String::new();
    let mut y_str = String::new();
    let numbers_set = ['0','1','2','3','4','5','6','7','8','9'];
    let regex: Vec<char> = "mul(X*,Y*)".chars().collect();
    let chars: Vec<char> = instructions.chars().collect();
    // iterate over the memory string to find the mul instructions
    let mut i = 0;
    let mut regex_index = 0;
    while i<instructions.len() {
        //println!("i: {}, regex_index: {}", i, regex_index);
        //println!("char: {}", chars.clone().clone().nth(i).unwrap());
        if regex_index>=regex.len() {
            let x = x_str.parse::<i32>().unwrap();
            let y = y_str.parse::<i32>().unwrap();
            total_instruction_result += x*y;
            regex_index = 0;
            x_str.clear();
            y_str.clear();
        }
        else if regex.get(regex_index).unwrap() == &char::from('*') {
            if numbers_set.contains(&chars.get(i).unwrap()) {
                if regex.get(regex_index-1).unwrap() == &char::from('X')  {
                    x_str.push(*chars.get(i).unwrap());
                }
                else {
                    y_str.push(*chars.get(i).unwrap());
                }
                i += 1;
            }
            else {
                regex_index += 1;
            }
        }
        else if regex.get(regex_index).unwrap() == &char::from('X') {
            regex_index += 1;
        }
        else if regex.get(regex_index).unwrap() == &char::from('Y') {
            regex_index += 1;
        }
        else if regex.get(regex_index).unwrap() == chars.get(i).unwrap() {
            i += 1;
            regex_index += 1;
        }
        else {
            regex_index = 0;
            i += 1;
            x_str.clear();
            y_str.clear();
        }
    }
    total_instruction_result

}

fn parse_mul_instructions_conditional(instructions: &str) -> i32 {
    let mut total_instruction_result = 0;

    // define the values to be used during the parsing
    let mut x_str = String::new();
    let mut y_str = String::new();
    let numbers_set = ['0','1','2','3','4','5','6','7','8','9'];
    let regex: Vec<char> = "mul(X*,Y*)".chars().collect();
    let chars: Vec<char> = instructions.chars().collect();
    // iterate over the memory string to find the mul instructions
    let mut i = 0;
    let mut regex_index = 0;
    let mut enabled= true;
    while i<instructions.len() {
        if enabled {
            if regex_index>=regex.len() {
                let x = x_str.parse::<i32>().unwrap();
                let y = y_str.parse::<i32>().unwrap();
                
                
                total_instruction_result += x*y;
                regex_index = 0;
                x_str.clear();
                y_str.clear();
            }
            else if regex.get(regex_index).unwrap() == &char::from('*') {
                if numbers_set.contains(&chars.get(i).unwrap()) {
                    if regex.get(regex_index-1).unwrap() == &char::from('X')  {
                        x_str.push(*chars.get(i).unwrap());
                    }
                    else {
                        y_str.push(*chars.get(i).unwrap());
                    }
                    i += 1;
                }
                else {
                    regex_index += 1;
                }
            }
            else if regex.get(regex_index).unwrap() == &char::from('X') {
                regex_index += 1;
            }
            else if regex.get(regex_index).unwrap() == &char::from('Y') {
                regex_index += 1;
            }
            else if regex.get(regex_index).unwrap() == chars.get(i).unwrap() {
                i += 1;
                regex_index += 1;
            }
            else {
                regex_index = 0;
                i += 1;
                x_str.clear();
                y_str.clear();
            }
        }
        else {
            i += 1;
        }
        
        // check for the enable instruction
        // get the next 4 characters
        let enable_instruction: Vec<char> = chars.clone().into_iter().skip(i-1).take(4).collect();
        if enable_instruction == ['d','o','(',')'] && enabled == false {
            enabled = true;
            regex_index = 0;
            x_str.clear();
            y_str.clear();
        }
        // check for the disable instruction
        // get the next 7 characters
        let disable_instruction: Vec<char> = chars.clone().into_iter().skip(i-1).take(7).collect();
        if disable_instruction == ['d','o','n','\'','t','(',')'] && enabled == true {
            enabled = false;
        }
    }
    total_instruction_result

}