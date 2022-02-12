pub mod dna;
pub mod newick_parser;
pub mod phylogenetic_tree;

use crate::Args;
use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_min_cost(args: &Args) -> Result<f64> {
    let sequences = parse_sequences(&args.sequences)?;
    let cost_matrix = parse_cost_matrix(&args.cost_matrix)?;
    let tree = newick_parser::parse(&args.tree)?;

    let mut total_cost = 0.0;
    
    for i_col in 0..sequences[0].len() {
        let col: Vec<_> = sequences
            .iter()
            .map(|x| x[i_col])
            .collect();
        
        let min_cost = tree
            .get_cost(&col, &cost_matrix)
            .into_values()
            .reduce(f64::min)
            .unwrap();
        
            total_cost += min_cost;
    }
    Ok(total_cost)
}

fn parse_cost_matrix(path: &str) -> Result<HashMap<(dna::Base, dna::Base), f64>> {
    let file = std::fs::read_to_string(path)?;

    // split each row by whitespace
    let mut raw_matrix: Vec<Vec<_>> = file
        .lines()
        .map(|l| l.split_whitespace().collect())
        .collect();

    // check if the base annotations are correct
    if !(raw_matrix[0].iter().eq(["A", "C", "T", "G"].iter())
        && raw_matrix
            .iter()
            .skip(1)
            .map(|x| &x[0])
            .eq(["A", "C", "T", "G"].iter()))
    {
        return Err(anyhow!("Couldn't parse matrix indices"));
    }

    // remove the base annotations
    raw_matrix.remove(0);
    for row in &mut raw_matrix {
        row.remove(0);
    }

    // construct the cost "matrix"
    let mut cost_matrix: HashMap<(dna::Base, dna::Base), f64> = HashMap::new();

    use dna::Base::*;
    for (row, base_from) in raw_matrix.iter().zip([A, C, T, G].iter()) {
        for (cost, base_to) in row.iter().zip([A, C, T, G].iter()) {
            let cost = cost.parse::<u32>()? as f64;
            cost_matrix.insert((*base_from, *base_to), cost);
            cost_matrix.insert((*base_to, *base_from), cost);
        }
    }

    // check if the cost matrix has an entry for every base combination
    if cost_matrix.len() != 16 {
        return Err(anyhow!("Couldn't parse matrix entries"));
    }

    Ok(cost_matrix)
}

fn parse_sequences(path: &str) -> Result<Vec<Vec<dna::Base>>> {
    let reader = BufReader::new(File::open(path)?);

    let mut sequences: Vec<Vec<dna::Base>> = Vec::new();

    // read each line, and parse the characters into dna::Base's
    for line in reader.lines() {
        let sequence: Result<Vec<dna::Base>> =
            line?.chars().map(|c| dna::Base::try_from(c)).collect();

        sequences.push(sequence?);
    }

    if sequences.is_empty() {
        return Err(anyhow!("No sequences found in {}", path));
    }

    // check if sequences are equal length
    if !sequences.iter().map(|s| s.len()).all_equal() {
        return Err(anyhow!("Sequences must be equal in length."));
    }

    Ok(sequences)
}
