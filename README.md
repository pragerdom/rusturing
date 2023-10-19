# RusTuring
A simple Turing machine implementation in Rust inspired by the [Common Lisp implementation](https://github.com/JMazlik01/turing-machine.git).

## Table of contents

1. [Rust](#rust-language)
2. [Usage](#the-idea-behind-usage)
3. [Example](#example)
4. [Notes](#notes)

## Rust language
Compiling and running requires having [Rust language](https://www.rust-lang.org/tools/install) installed. Tested by `cargo`.

## The idea behind usage
To create a new [Turing machine](https://en.wikipedia.org/wiki/Turing_machine), use the ```TuringMachine::new``` constructor and then visualize and simulate it with the ```TuringMachine.simulate``` method.

To specify the machine's instructions, use the `Instruction` data structure.
```rust
struct Instruction {
    // (Current state, Current char) -> (Next state, New char, Move direction)
    current_state: String,
    current_char: char,
    next_state: String,
    new_char: char,
    move_direction: i32,
}
```

Different visualization is to be added. See examples for the reference.

## Example
Full examples can be found in `src/main.rs`.

#### Binary flip
```rust
fn main() {
    let instructions = vec![
        Instruction::new("Q0".to_string(), '0', "Q0".to_string(), '1', 1),
        Instruction::new("Q0".to_string(), '1', "Q0".to_string(), '0', 1),
        Instruction::new("Q0".to_string(), 'b', "Q1".to_string(), 'b', 1),
        Instruction::new("Q1".to_string(), 'b', "Q2".to_string(), 'b', -1),
        Instruction::new("Q1".to_string(), '1', "Q1".to_string(), '1', -1),
        Instruction::new("Q1".to_string(), '0', "Q1".to_string(), '0', -1),
    ];

    let description = "Flips the bits of a binary input".to_string();
    let initial_state = "Q0".to_string();
    let final_states = vec!["Q2".to_string()];

    let turing_machine = TuringMachine::new(description, instructions, initial_state, final_states);

    let input = "1101";
    turing_machine.simulate(input);

}
```

After running we get the following simulation:

```
====================
Description of the machine: Flips the bits of a binary input

Step:   Tape State      (Current) -> (Next)
0:      b1101b          (Q0, 1) -> (Q0, 0, 1)
1:      b0101b          (Q0, 1) -> (Q0, 0, 1)
2:      b0001b          (Q0, 0) -> (Q0, 1, 1)
3:      b0011b          (Q0, 1) -> (Q0, 0, 1)
4:      b0010b          (Q0, b) -> (Q1, b, 1)
5:      b0010bb         (Q1, b) -> (Q2, b, -1)
------
Derivation: 1101 ⊢* 0010
```

## Notes
- Symbol b is reserved for blank character.
- Symbols are exactly of character length 1.