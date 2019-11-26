///! Define traits for environment, action, players, etc

/// The state of the environment
pub trait State: Clone + std::hash::Hash + Eq {}

/// An action that can be applied to an environment
pub trait Action: Clone {}

/// An environment, that can be represented as a state and to which actions can be applied.
/// The environment defines the associated types of the state and action.
pub trait Environment {
    type State: State;
    type Action: Action;

    fn reset(&mut self) -> (Self::State, Vec<Self::Action>);

    fn step(&mut self, action: Self::Action) -> (Self::State, f64, bool, Vec<Self::Action>);
}

/// A player that can take actions from a given state.
/// Unlink the environment, that defines it own state and action, players are generic over them
pub trait Player<S: State, A: Action> {
    fn take_action(&mut self, state: S, actions: Vec<A>) -> A;

    fn start(&mut self, state: S, actions: Vec<A>) -> A {
        self.take_action(state, actions)
    }

    fn step(&mut self, state: S, actions: Vec<A>, _reward: f64) -> A {
        self.take_action(state, actions)
    }

    fn end(&mut self, _state: S, _reward: f64) {}
}

/// A player that can generated a fixed version of self, used to train against previous
/// snapshots of itself
pub trait LearningPlayer<S: State, A: Action>: Player<S, A> {
    type Freezed: Player<S, A>;
    fn freezed(&mut self) -> Self::Freezed;
    fn on_cycle_end(&self) {}
}
