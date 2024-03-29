use crate::simple_players::*;
use crate::traits::*;
use std::fs::File;
use std::io::prelude::*;

/// Train a given player against itself
pub fn train<S, A, P, E>(
    env: &mut E,
    player: &mut P,
    train_episodes: u32,
    eval_episodes: u32,
    cycles: u32,
    opponent_epsilon: f32,
    stats_file_name: &str,
) where
    S: State,
    A: Action,
    P: LearningPlayer<S, A>,
    E: Environment<State = S, Action = A>,
{
    let mut stats_file = File::create(stats_file_name).unwrap();
    let mut random_adversary = RandomPlayer::new();
    let mut adversary = OpponentWrapper::new(player.freezed(), opponent_epsilon);
    for cycle in 1..=cycles {
        // Train against a fixed adversary
        let train_score = run_duel(env, player, &mut adversary, train_episodes);

        // Eval the newly trained player against the fixed adversary
        let mut new_adversary = OpponentWrapper::new(player.freezed(), opponent_epsilon);
        let eval_score = run_duel(
            env,
            new_adversary.inner_mut(),
            adversary.inner_mut(),
            eval_episodes,
        );

        new_adversary.inner_mut().reset_stats();
        let eval_random_score = run_duel(
            env,
            new_adversary.inner_mut(),
            &mut random_adversary,
            eval_episodes,
        );
        let eval_random_stats = new_adversary.inner_mut().stats();
        serde_json::to_writer(&stats_file, &eval_random_stats).unwrap();
        stats_file.write("\n".as_bytes()).unwrap();

        adversary = new_adversary;

        println!(
            "== Cycle {}/{} ==\navg train score = {}, avg eval score = {}, avg eval random score = {}",
            cycle, cycles, train_score, eval_score, eval_random_score
        );
        println!("Eval random stats: {:?}", eval_random_stats);

        player.cycle_end();
    }
}

/// Run multiple matches between two players, alternating which one starts the match
/// Since we assume this is a zero-sum game, the score of the second one is simply the opposite
pub fn run_duel<S, A, P1, P2, E>(
    env: &mut E,
    player_1: &mut P1,
    player_2: &mut P2,
    episodes: u32,
) -> f32
where
    S: State,
    A: Action,
    P1: Player<S, A>,
    P2: Player<S, A>,
    E: Environment<State = S, Action = A>,
{
    assert_eq!(episodes % 2, 0, "episodes must be even");
    let mut score = 0.;
    for _ in (0..episodes).step_by(2) {
        score += run_match(env, player_1, player_2);
        score -= run_match(env, player_2, player_1);
    }
    score / episodes as f32
}

/// Run a match between two players and return the score the first one.
/// Since we assume this is a zero-sum game, the score of the second one is simply the opposite
pub fn run_match<S, A, P1, P2, E>(env: &mut E, player_1: &mut P1, player_2: &mut P2) -> f32
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
