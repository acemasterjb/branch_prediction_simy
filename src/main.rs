use std::borrow::BorrowMut;

use indoc::indoc;  // aka: indented document
use rand::distributions::{Distribution, Uniform};

#[derive(Default)]
struct PredictionState {
    tally_always_taken: f64,
    tally_always_not_taken: f64,
    tally_one_bit: f64,
    tally_two_bit: f64,

    one_bit_state: bool,

    // Not-taken state: [0, 1]
    // taken state: [2, 3]
    two_bit_state: u8,
}

fn main() {
    let not_taken_threshold: f64 = 0.5;

    let mut random_num_generator = rand::thread_rng();
    let distribution: Uniform<f64> = Uniform::from(0.0..1.0);
    let mut branch_outcomes: Vec<f64> = vec![];

    for _ in 0..10_000 {
        branch_outcomes.push(distribution.sample(&mut random_num_generator));
    };

    let mut prediction_state: PredictionState = PredictionState::default();
    let prediction_state: &mut PredictionState = prediction_state.borrow_mut();
    let mut is_taken: bool;

    for outcome in branch_outcomes {
        is_taken = outcome > not_taken_threshold;

        if is_taken {
            update_state_for_taken(prediction_state);
        } else {
            update_state_for_not_taken(prediction_state);
        }
        
        prediction_state.one_bit_state = is_taken;
    }

    print_intro();
    print_output(prediction_state);
}

fn update_state_for_taken(prediction_state: &mut PredictionState) {
    prediction_state.tally_always_taken += 1.0;
    
    if prediction_state.one_bit_state {
        prediction_state.tally_one_bit += 1.0;
    }
    
    if prediction_state.two_bit_state > 1 {
        if prediction_state.two_bit_state > 2 {
            prediction_state.two_bit_state -= 1;
        }
        prediction_state.tally_two_bit += 1.0;
    } else {
        // prediction was wrong, continue...
        prediction_state.two_bit_state += 1;
    }
}

fn update_state_for_not_taken(prediction_state: &mut PredictionState) {
    prediction_state.tally_always_not_taken += 1.0;

    if !prediction_state.one_bit_state {
        prediction_state.tally_one_bit += 1.0;
    }

    if prediction_state.two_bit_state == 2 {
        // move to the end of "taken" state
        prediction_state.two_bit_state += 1;
    } else if prediction_state.two_bit_state == 3 {
        // move to start of "not-taken" state
        prediction_state.two_bit_state = 0;
    } else {
        // correct prediction case
        prediction_state.tally_two_bit += 1.0;
    }
}

fn print_intro() {
    println!("\t\t\t\t\t👨‍🔬 Branch Prediction Simulation 👨‍🔬\n");

    println!(indoc!{"
        Simulation Process:
            1) Randomly generate 10k numbers in range [0, 1) and insert into a vector
            2) For every number in said vector:
            3) Check for prediction results if [taken, not-taken, agrees with 1-bit and 2-bit history]
            4) Update n-bit histories and go to step 1 until vector is exhausted
    "});
}

fn print_output(prediction_state: &mut PredictionState) {
    println!("Always Taken: {}%\n", (prediction_state.tally_always_taken / 1e4) * 100.0);
    println!("Always Not Taken: {}%\n", (prediction_state.tally_always_not_taken / 1e4) * 100.0);
    println!("One-Bit History: {}%\n", (prediction_state.tally_one_bit / 1e4) * 100.0);
    println!("Two-Bit History: {}%", (prediction_state.tally_two_bit / 1e4) * 100.0);
}
