use std::collections::HashMap;

use super::dna::Base;

#[derive(Debug, Default)]
pub struct Node {
    pub sequence_id: Option<usize>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    pub fn with_sequence_id(id: usize) -> Self {
        Self {
            sequence_id: Some(id),
            ..Default::default()
        }
    }

    pub fn with_children(left: Node, right: Node) -> Self {
        Self {
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
            ..Default::default()
        }
    }

    pub fn get_cost(
        &self,
        sequences_col: &[Base],
        matrix: &HashMap<(Base, Base), f64>,
    ) -> HashMap<Base, f64> {
        let mut costs = HashMap::new();

        use Base::*;
        match self.sequence_id {
            // Leaf node
            Some(id) => {
                for base in [A, C, T, G] {
                    let cost = if base == sequences_col[id - 1] {
                        0.0
                    } else {
                        f64::INFINITY
                    };

                    costs.insert(base, cost);
                }

                assert!(costs.len() == 4);
            }

            // Internal node
            None => {
                for a in [A, C, T, G] {
                    let mut min = 0.0;

                    for child in [self.left.as_ref().unwrap(), self.right.as_ref().unwrap()] {
                        let child_costs = child.get_cost(sequences_col, matrix);

                        min += [A, C, T, G]
                            .iter()
                            .map(|b| child_costs[b] + matrix[&(a, *b)])
                            .reduce(f64::min)
                            .unwrap();
                    }

                    costs.insert(a, min);
                }
            }
        }

        costs
    }
}
