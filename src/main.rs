mod lifetime_ownership;
mod match_result;

/// Before we start working on a project we need so get to know Rust
///
/// This is a showcase project where we simplify everything.
/// it might not do everything correctly but it'll showcase specific scenarios for sustain areas.
/// Run 'cargo test --all' to run the examples.
///
/// The goal is to write TDD for each scenario.
fn main() {
    println!(
        r#"
        Hello to you, run 
        'cargo doc --open'
        Once you're done there go on and do:
        'cargo test --all'
        to to see it all in action!
        "#
    );
}
