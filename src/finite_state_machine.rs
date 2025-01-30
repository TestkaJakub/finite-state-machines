use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct State {
    name: String,
    transitions: HashMap<String, String>,
}

impl State {
    fn new(name: &str) -> Self {
        State {
            name: name.to_string(),
            transitions: HashMap::new(),
        }
    }

    fn add_transition(&mut self, event: &str, target: &str) {
        self.transitions.insert(event.to_string(), target.to_string());
    }

    fn get_next_state(&self, event: &str) -> Option<&String> {
        self.transitions.get(event)
    }
}

pub struct FiniteStateMachine {
    states: HashMap<String, Rc<RefCell<State>>>, // Use Rc<RefCell<State>> for mutability
    current_state: Option<Rc<RefCell<State>>>,
}

impl FiniteStateMachine {
    pub fn new() -> Self {
        FiniteStateMachine {
            states: HashMap::new(),
            current_state: None,
        }
    }

    pub fn add_state(&mut self, state_name: &str) {
        let state = Rc::new(RefCell::new(State::new(state_name))); // Wrap in Rc<RefCell>
        self.states.insert(state_name.to_string(), Rc::clone(&state));
        if self.current_state.is_none() {
            self.current_state = Some(Rc::clone(&state));
        }
    }

    pub fn add_transition(&self, from_state: &str, event: &str, to_state: &str) {
        if let (Some(from), Some(to)) = (self.states.get(from_state), self.states.get(to_state)) {
            from.borrow_mut().add_transition(event, &to.borrow().name);
        } else {
            println!("States must be added before creating transitions");
        }
    }

    pub fn set_initial_state(&mut self, state_name: &str) {
        if let Some(state) = self.states.get(state_name) {
            self.current_state = Some(state.clone());
        } else {
            println!("State '{}' not found in the FSM", state_name);
        }
    }

    pub fn process_event(&mut self, event: &str) {
        if self.current_state.is_none() {
            println!("Initial state not set");
            return;
        }

        let current = self.current_state.as_ref().unwrap();
        let next_state_name = current.borrow().get_next_state(event).cloned();

        match next_state_name {
            Some(v) => {
                println!(
                    "Transitioning from {} to {} on event '{}'", 
                    current.borrow().name, v, event
                );
                self.set_initial_state(&v);
            }
            None => {
                println!(
                    "No transition found for event '{}' in state '{}'", 
                    event, current.borrow().name
                );
            }
        }
    }
}
