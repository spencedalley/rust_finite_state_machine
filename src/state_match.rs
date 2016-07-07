// _Lower CaSe_, # this is a comment #, ^this is all upper case^

pub enum MachineState {
	Normal, 
	Comment, 
	Upper, 
	Lower, 
}

// Option<char> means we are either going to return a char or None
// The <> just means we can have an option of whatever we pass in
pub fn machine_cycle(state: &MachineState, character: char) -> (Option<char>, MachineState) {
    use std::ascii::AsciiExt; 

    match state {
        &MachineState::Normal => {
            match character {
                '#' => { return (None, MachineState::Comment); }
                '^' => { return (None, MachineState::Upper); }
                '_' => { return (None, MachineState::Lower); }
                _ => { return (Some(character), MachineState::Normal); } 
            }
        }
        &MachineState::Comment => {
            if character == '#' {
                return (None, MachineState::Normal); 
            } else {
                return (None, MachineState::Comment); 
            }
        }
        &MachineState::Upper => {
            if character == '^' {
                return (None, MachineState::Normal); 
            } else {
                return (Some(character.to_ascii_uppercase()), MachineState::Upper); 
            }
        }
        &MachineState::Lower => {
            if character == '_' {
                return (None, MachineState::Normal); 
            } else {
                return (Some(character.to_ascii_lowercase()), MachineState::Lower); 
            }
        }
    }
}

pub fn hello_world() {
    println!("Hello world")
}