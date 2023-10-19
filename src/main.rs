struct Instruction {
    // data structure to represent a single instruction
    // (Current state, Current char) -> (Next state, New char, Move direction)
    current_state: String,
    current_char: char,
    next_state: String,
    new_char: char,
    move_direction: i32,
}

impl Instruction {
    // constructor implementation
    fn new(current_state: String, current_char: char, next_state: String, new_char: char, move_direction: i32) -> Self {
        Self {
            current_state,
            current_char,
            next_state,
            new_char,
            move_direction,
        }
    }
}

struct TuringMachine {
    // data structure to represent a Turing machine with a simple tape
    description: String,
    instructions: Vec<Instruction>,
    initial_state: String,
    final_states: Vec<String>,
}

impl TuringMachine {
    // constructor implementation
    fn new(description: String, instructions: Vec<Instruction>, initial_state: String, final_states: Vec<String>) -> Self {
        Self {
            description,
            instructions,
            initial_state,
            final_states,
        }
    }

    // simulate the Turing machine
    fn simulate(&self, input: &str) {
        let mut tape: Vec<char> = vec!['b'];
        tape.extend(input.chars());
        tape.push('b');

        let mut current_state = self.initial_state.clone();
        let mut head_pos = 1;

        // print the description
        println!("====================");
        println!("Description of the machine: {}\n", self.description);  

        println!("Step:\tTape State\t(Current) -> (Next)");

        let mut step = 0;

        while !self.final_states.contains(&current_state) {
            let current_char = tape[head_pos];

            let instruction = match self.instructions.iter().find(|i| i.current_state == current_state && i.current_char == current_char) {
                Some(inst) => inst,
                None => {
                    println!("No instruction found for state {} and char {}", current_state, current_char);
                    break;
                }
            };
            // print the current tape with the instruction
            println!("{}:\t{}\t\t({}, {}) -> ({}, {}, {})", step, tape.iter().collect::<String>(), current_state, current_char, instruction.next_state, instruction.new_char, instruction.move_direction);

            tape[head_pos] = instruction.new_char;
            current_state = instruction.next_state.clone();
            head_pos = (head_pos as i32 + instruction.move_direction) as usize;

            // if the head is at the end of the tape, add a new blank char
            if head_pos == tape.len() {
                tape.push('b');
            }

            step += 1;
        }

        // print the final tape
        println!("------");

        // print and strip all the b's from start and end of the tape
        let mut output = tape.iter().collect::<String>();
        output = output.trim_matches('b').to_string();
        output = output.trim_start_matches('b').to_string();

        println!("Derivation: {} ⊢* {}\n", input, output);
    }
}

// examples
fn main() {
    //
    // EXAMPLE 1
    //  

    // make a machine that flips the bits of a binary input
    let instructions = vec![
        Instruction::new("Q0".to_string(), '0', "Q0".to_string(), '1', 1),
        Instruction::new("Q0".to_string(), '1', "Q0".to_string(), '0', 1),
        Instruction::new("Q0".to_string(), 'b', "Q1".to_string(), 'b', 1),
        Instruction::new("Q1".to_string(), 'b', "Q2".to_string(), 'b', -1),
        Instruction::new("Q1".to_string(), '1', "Q1".to_string(), '1', -1),
        Instruction::new("Q1".to_string(), '0', "Q1".to_string(), '0', -1),
    ];

    let description = "A Turing machine that flips the bits of a binary input".to_string();
    let initial_state = "Q0".to_string();
    let final_states = vec!["Q2".to_string()];

    let turing_machine = TuringMachine::new(description, instructions, initial_state, final_states);

    let input = "1101";
    turing_machine.simulate(input);

    let input2 = "101";
    turing_machine.simulate(input2);

    //
    // EXAMPLE 2
    //  

    // make a machine that does "For word w return ww"
    // more compact version of declaring the machine
    let tm2 = TuringMachine::new(
        "For word w returns ww".to_string(),
        vec![
            Instruction::new("q1".to_string(), 'a', "q2".to_string(), 'x', 1),
            Instruction::new("q1".to_string(), 'b', "qf".to_string(), 'b', 0),
            Instruction::new("q2".to_string(), 'a', "q2".to_string(), 'a', 1),
            Instruction::new("q2".to_string(), 'x', "q2".to_string(), 'x', 1),
            Instruction::new("q2".to_string(), 'b', "q3".to_string(), 'x', -1),
            Instruction::new("q3".to_string(), 'a', "q4".to_string(), 'a', -1),
            Instruction::new("q3".to_string(), 'x', "q3".to_string(), 'x', -1),
            Instruction::new("q3".to_string(), 'b', "q5".to_string(), 'b', 1),
            Instruction::new("q4".to_string(), 'a', "q4".to_string(), 'a', -1),
            Instruction::new("q4".to_string(), 'x', "q1".to_string(), 'x', 1),
            Instruction::new("q5".to_string(), 'x', "q5".to_string(), 'a', 1),
            Instruction::new("q5".to_string(), 'b', "qf".to_string(), 'b', 0),
        ],
        "q1".to_string(),
        vec!["qf".to_string()],
    );
    
    tm2.simulate("aaa");
}
