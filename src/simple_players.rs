use crate::traits::*;
use rand::prelude::*;

pub struct DummyPlayer {}

impl<S: State, A: Action> Player<S, A> for DummyPlayer {
    fn take_action(&mut self, _state: S, actions: Vec<A>) -> A {
        actions[0].clone()
    }
}

pub struct RandomPlayer {}

impl<S: State, A: Action> Player<S, A> for RandomPlayer {
    fn take_action(&mut self, _state: S, actions: Vec<A>) -> A {
        let mut rng = thread_rng();
        actions.choose(&mut rng).unwrap().clone()
    }
}

pub struct OpponentWrapper<S: State, A: Action, P: Player<S, A>> {
    inner: P,
    epsilon: f64,
    _s: std::marker::PhantomData<S>,
    _a: std::marker::PhantomData<A>,
}

impl<S: State, A: Action, P: Player<S, A>> OpponentWrapper<S, A, P> {
    pub fn new(inner: P, epsilon: f64) -> Self {
        OpponentWrapper {
            inner,
            epsilon,
            _s: std::marker::PhantomData,
            _a: std::marker::PhantomData,
        }
    }

    pub fn inner_mut(&mut self) -> &mut P {
        &mut self.inner
    }
}

impl<S: State, A: Action, P: Player<S, A>> Player<S, A> for OpponentWrapper<S, A, P> {
    fn take_action(&mut self, state: S, actions: Vec<A>) -> A {
        if random::<f64>() <= self.epsilon {
            // Take a random action
            let mut rng = thread_rng();
            actions.choose(&mut rng).unwrap().clone()
        } else {
            // Delegate
            self.inner.take_action(state, actions)
        }
    }
}
