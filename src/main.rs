mod weighted_parsimony;

use clap::Parser;


/// Find out how parsimonious a phylogenetic tree is, 
/// using the weighted parsimony algorithm
#[derive(Parser, Debug)]
#[clap(author, about, version, long_about = None)]
pub struct Args {
    /// File containing DNA sequences, each on a new line, in plain text.
    #[clap(short, long)]
    sequences: String,

    /// File containing a cost matrix for the nucleotide transitions
    #[clap(short, long)]
    cost_matrix: String,

    /// Phylogenetic Tree in Newick Format
    #[clap(short, long)]
    tree: String,
}

fn main() {
    let args = Args::parse();

    match weighted_parsimony::get_min_cost(&args) {
        Ok(cost) => println!("Minimum weighted parsimony cost: {cost}"),
        Err(e) => println!("Error: {e}")
    }
}