use crate::traits::*;
use rand::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct QLearningStats {
    pub total_actions: u32,
    pub random_actions: u32,
    pub dummy_actions: u32,
    pub learned_actions: u32,
}

impl QLearningStats {
    fn new() -> Self {
        Self {
            total_actions: 0,
            random_actions: 0,
            dummy_actions: 0,
            learned_actions: 0,
        }
    }
}

#[derive(Clone)]
pub struct QLearningPlayer<S: State> {
    // A map from state, to (hit count, q-value)
    q_table: HashMap<S, (u32, Vec<f64>)>,
    epsilon: f64,
    min_epsilon: f64,
    epsilon_decay: f64,
    alpha: f64,
    gamma: f64,
    prev_state: Option<S>,
    prev_action_index: Option<usize>,
    stats: QLearningStats,
}

impl<S: State> QLearningPlayer<S> {
    pub fn new() -> Self {
        QLearningPlayer {
            q_table: HashMap::new(),
            epsilon: 1.,
            min_epsilon: 0.1,
            epsilon_decay: 0.999999,
            alpha: 0.1,
            gamma: 1.,
            prev_state: None,
            prev_action_index: None,
            stats: QLearningStats::new(),
        }
    }

    fn update_q_table(&mut self, new_value: f64) {
        // Read the q-values (we can assume they were already initialized by take_action())
        let action_values = self
            .q_table
            .get_mut(self.prev_state.as_ref().unwrap())
            .unwrap();
        let i = self.prev_action_index.unwrap();
        action_values.0 += 1;
        action_values.1[i] += self.alpha * (new_value - action_values.1[i]);
    }
}

impl<S: State, A: Action> Player<S, A> for QLearningPlayer<S> {
    type Stats = QLearningStats;

    fn take_action(&mut self, state: S, actions: Vec<A>) -> A {
        self.stats.total_actions += 1;
        self.prev_state = Some(state.clone());

        // Ensure the q-values are initialized for this state
        let action_values = &self
            .q_table
            .entry(state)
            .or_insert_with(|| (0, vec![0.; actions.len()]))
            .1;

        let action_index = if random::<f64>() <= self.epsilon {
            // Take a random action
            self.stats.random_actions += 1;
            thread_rng().gen_range(0, actions.len())
        } else {
            // Take the most rewarding action
            if action_values.iter().all(|&x| x == 0.) {
                self.stats.dummy_actions += 1;
            } else {
                self.stats.learned_actions += 1;
            }
            max(action_values).0
        };

        self.prev_action_index = Some(action_index);
        actions[action_index].clone()
    }

    fn step(&mut self, state: S, actions: Vec<A>, reward: f64) -> A {
        // Read the q-values (we can assume they were already initialized by take_action())
        let max_q_value = &self
            .q_table
            .get(&state)
            .map(|(_, q_row)| max(q_row).1)
            .unwrap_or(0.);
        let new_value = reward + self.gamma * max_q_value;
        self.update_q_table(new_value);
        self.take_action(state.clone(), actions)
    }

    fn end(&mut self, _state: S, reward: f64) {
        self.update_q_table(reward);
        self.epsilon = (self.epsilon * self.epsilon_decay).max(self.min_epsilon);
    }

    fn reset_stats(&mut self) {
        self.stats = QLearningStats::new();
    }

    fn stats(&self) -> Option<Self::Stats> {
        Some(self.stats.clone())
    }
}

impl<S: State, A: Action> LearningPlayer<S, A> for QLearningPlayer<S>
where
    Self: Player<S, A>,
{
    type Freezed = QLearnedPlayer<S>;

    fn freezed(&self) -> QLearnedPlayer<S> {
        QLearnedPlayer {
            q_table: self.q_table.clone(),
            stats: QLearningStats::new(),
        }
    }

    fn cycle_end(&mut self) {
        println!(
            "Q-table size = {}, epsilon = {}",
            self.q_table.len(),
            self.epsilon
        );
        println!("Stats = {:?}", self.stats());
        self.reset_stats();

        // Load stats from q-table: hit count by (game depth, learned actions)
        let mut stats: HashMap<(u16, u8), Vec<u32>> = HashMap::new();
        for (state, (hit_count, q_values)) in &self.q_table {
            let learned_actions =
                q_values
                    .iter()
                    .fold(0, |num, &q_value| if q_value == 0. { num } else { num + 1 });
            stats
                .entry((state.game_depth(), learned_actions))
                .or_default()
                .push(*hit_count);
        }
        let mut stats: Vec<(u16, u8, u32, u32)> = stats
            .into_iter()
            .map(|(key, value)| {
                (
                    key.0,
                    key.1,
                    (value.iter().map(|&x| x as f64).sum::<f64>() / value.len() as f64) as u32,
                    value.len() as u32,
                )
            })
            .collect();
        stats.sort_by_key(|v| (v.0, v.1));
        println!("depth | learned actions | avg. visits | num. states");
        for stat in stats {
            if stat.0 > 1 {
                continue;
            }
            println!(
                "{: >5} | {: >15} | {: >11} | {: >11}",
                stat.0, stat.1, stat.2, stat.3
            );
        }
    }
}

pub struct QLearnedPlayer<S: State> {
    q_table: HashMap<S, (u32, Vec<f64>)>,
    stats: QLearningStats,
}

impl<S: State, A: Action> Player<S, A> for QLearnedPlayer<S> {
    type Stats = QLearningStats;

    fn take_action(&mut self, state: S, actions: Vec<A>) -> A {
        self.stats.total_actions += 1;
        match self.q_table.get(&state) {
            None => {
                // Here we act as if the row is made of only zeros, in which case
                // the argmax() would just return the first one
                self.stats.dummy_actions += 1;
                actions[0].clone()
            }
            Some((_, action_values)) => {
                if action_values.iter().all(|&x| x == 0.) {
                    self.stats.dummy_actions += 1;
                } else {
                    self.stats.learned_actions += 1;
                }
                actions[max(&action_values).0].clone()
            }
        }
    }

    fn reset_stats(&mut self) {
        self.stats = QLearningStats::new();
    }

    fn stats(&self) -> Option<Self::Stats> {
        Some(self.stats.clone())
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
