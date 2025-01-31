use crate::finite_state_machine::FiniteStateMachine;

pub struct Engine {
    fsm: FiniteStateMachine,
}

impl Engine {
    pub fn new(fsm: FiniteStateMachine) -> Self {
        return Engine {
            fsm
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        while self.fsm.get_current_state().as_deref() != Some("End") && self.fsm.get_current_state().as_deref() != Some("Death"){
            self.update()?;
        }
        Ok(())
    }

    fn update(&mut self) -> Result<(), String> {
        let current_state = self.fsm.get_current_state()
                                          .ok_or("No current state found")?;

        println!("Current state is {}", current_state);

        let possible_transitions = self.fsm.get_possible_transitions().ok_or("No possible transitions found")?;

        println!("Possible moves are:");

        for transition in possible_transitions.iter(){
            println!("{}", transition);
        }

        let mut input = String::new();

        std::io::stdin()
        .read_line(&mut input)
        .map_err(|_| "Failed to read input")?;

        let input = input.trim();

        self.fsm.process_event(&input);

        return Ok(())
    }
}

// pub fn Engine(fsm: FiniteStateMachine) {

// }