mod weighted_parsimony;

fn main() {
    match weighted_parsimony::get_min_score() {
        Ok(score) => println!("Minimum weighted parsimony score: {score}"),
        Err(e) => println!("Error: {e}")
    }
}