use crate::traits::*;

// pub fn train<P: LearningPlayer>(env: &mut Environment, player: P) {}

/// Run a match between two players and return the score the first one.
/// Since we assume this is a zero-sum game, the score of the second one is simply the opposite
pub fn run_match<S, A, P1, P2, E>(env: &mut E, player_1: &mut P1, player_2: &mut P2) -> f64
where
    S: State,
    A: Action,
    P1: Player<S, A>,
    P2: Player<S, A>,
    E: Environment<State = S, Action = A>,
{
    let (state, valid_actions) = env.reset();

    // Player 1's first action
    let action = player_1.start(state, valid_actions);
    let (state, reward_1, done, valid_actions) = env.step(action);
    let mut score = reward_1;
    debug_assert!(!done);

    // Player 2's first action
    let action = player_2.start(state, valid_actions);
    let (mut state, reward_2, done, mut valid_actions) = env.step(action);
    score -= reward_2;
    debug_assert!(!done);

    loop {
        // Player 1 turn
        let action = player_1.step(state, valid_actions, reward_1 - reward_2);
        let (_state, reward_1, done, _valid_actions) = env.step(action);
        state = _state;
        valid_actions = _valid_actions;
        score += reward_1;
        if done {
            player_1.end(state.clone(), reward_1);
            player_2.end(state, reward_2 - reward_1);
            return score;
        }

        // Player 2 turn
        let action = player_2.step(state, valid_actions, reward_2 - reward_1);
        let (_state, reward_2, done, _valid_actions) = env.step(action);
        state = _state;
        valid_actions = _valid_actions;
        score -= reward_2;
        if done {
            player_2.end(state.clone(), reward_2);
            player_1.end(state, reward_1 - reward_2);
            return score;
        }
    }
}
