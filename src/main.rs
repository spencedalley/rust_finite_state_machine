mod state_match; 

fn main() {
    state_match::hello_world(); 

    let mut state = state_match::MachineState::Normal; 
    let input = "This is some _LOWER cAsE code_, #this is a comment,# ^and this part is Upper Case^. The end."; 
    let mut processed_string = String::new(); 

    for character in input.chars() {
        let (output, new_state) = state_match::machine_cycle(&state, character);
        match output {
            // If output is "some" rather than None, put whatever the value is in c
            Some(c) => { processed_string.push(c); }
            None => {}
        }; 
        state = new_state; 
    }

    println!("Input:\t{}\t{}", input, processed_string); 
}