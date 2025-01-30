mod finite_state_machine;
use finite_state_machine::*;

fn main() {
    let mut fsm = FiniteStateMachine::new();

    fsm.add_state("State A");
    fsm.add_state("State B");

    fsm.add_transition("State A", "transition to B", "State B");

    fsm.set_initial_state("State A");

    fsm.process_event("transition to B");
}