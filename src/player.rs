use crate::traits::*;
use rand::prelude::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct QLearningPlayer<S: State> {
    q_table: HashMap<S, Vec<f64>>,
    epsilon: f64,
    min_epsilon: f64,
    epsilon_decay: f64,
    alpha: f64,
    gamma: f64,
    prev_state: Option<S>,
    prev_action_index: Option<usize>,
}

impl<S: State> QLearningPlayer<S> {
    pub fn new() -> Self {
        QLearningPlayer {
            q_table: HashMap::new(),
            epsilon: 1.,
            min_epsilon: 0.1,
            epsilon_decay: 0.99995,
            alpha: 0.1,
            gamma: 1.,
            prev_state: None,
            prev_action_index: None,
        }
    }

    fn update_q_table(&mut self, new_value: f64) {
        // Read the q-values (we can assume they were already initialized by take_action())
        let action_values = self
            .q_table
            .get_mut(self.prev_state.as_ref().unwrap())
            .unwrap();
        let i = self.prev_action_index.unwrap();
        action_values[i] += self.alpha * (new_value - action_values[i]);
    }
}

impl<S: State, A: Action> Player<S, A> for QLearningPlayer<S> {
    fn take_action(&mut self, state: S, actions: Vec<A>) -> A {
        self.prev_state = Some(state.clone());

        // Ensure the q-values are initialized for this state
        let action_values = self
            .q_table
            .entry(state)
            .or_insert_with(|| vec![0.; actions.len()]);

        let action_index = if random::<f64>() <= self.epsilon {
            // Take a random action
            thread_rng().gen_range(0, actions.len())
        } else {
            // Take the most rewarding action
            max(action_values).0
        };

        self.prev_action_index = Some(action_index);
        actions[self.prev_action_index.unwrap()].clone()
    }

    fn step(&mut self, state: S, actions: Vec<A>, reward: f64) -> A {
        let next_action = self.take_action(state.clone(), actions);
        // Read the q-values (we can assume they were already initialized by take_action())
        let action_values = self.q_table.get(&state).unwrap();
        let new_value = reward + self.gamma * max(action_values).1;
        self.update_q_table(new_value);
        next_action
    }

    fn end(&mut self, _state: S, reward: f64) {
        self.update_q_table(reward);
        self.epsilon = (self.epsilon * self.epsilon_decay).max(self.min_epsilon);
    }
}

impl<S: State, A: Action> LearningPlayer<S, A> for QLearningPlayer<S> {
    type Freezed = QLearnedPlayer<S>;
    fn freezed(&mut self) -> QLearnedPlayer<S> {
        QLearnedPlayer {
            q_table: self.q_table.clone(),
        }
    }

    fn on_cycle_end(&self) {
        println!("Q-table size = {}", self.q_table.len());
    }
}

pub struct QLearnedPlayer<S: State> {
    q_table: HashMap<S, Vec<f64>>,
}

impl<S: State, A: Action> Player<S, A> for QLearnedPlayer<S> {
    fn take_action(&mut self, state: S, actions: Vec<A>) -> A {
        match self.q_table.get(&state) {
            None => {
                // Here we act as if the row is made of only zeros, in which case
                // the argmax() would just return the first one
                actions[0].clone()
            }
            Some(action_values) => actions[max(action_values).0].clone(),
        }
    }
}

/// Get the maximum value and position of a list
/// Panics if the list is empty
fn max(values: &Vec<f64>) -> (usize, f64) {
    let mut max_i = 0;
    let mut max_el = values[0];
    for (i, &value) in values.iter().enumerate().skip(1) {
        if value > max_el {
            max_el = value;
            max_i = i;
        }
    }
    (max_i, max_el)
}
