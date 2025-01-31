mod finite_state_machine;
use finite_state_machine::FiniteStateMachine;

mod engine;
use engine::Engine;

fn main() {
    let mut fsm = FiniteStateMachine::new();

    fsm.add_state("Start");
    fsm.add_state("State A");
    fsm.add_state("State B");
    fsm.add_state("State C");
    fsm.add_state("End");
    fsm.add_state("Death");

    fsm.add_transition("Start", "A", "State A");
    fsm.add_transition("Start", "B", "State B");
    fsm.add_transition("State A", "C", "State C");
    fsm.add_transition("State C", "B", "State B");
    fsm.add_transition("State B", "End", "End");
    fsm.add_transition("State B", "Death", "Death");

    fsm.set_initial_state("Start");

    let mut engine = Engine::new(fsm);
    
    if let Err(v) = engine.run() {
        println!("{}", v)
    }    
}