use anyhow::{anyhow, Result};
use std::{iter::Peekable, num::ParseIntError, str::Chars};

use super::phylogenetic_tree::Node;

pub fn parse(s: &str) -> Result<Node> {
    let mut parser = NewickParser {
        iter: s.chars().peekable(),
    };

    parser.start()
}

struct NewickParser<'a> {
    iter: Peekable<Chars<'a>>,
}

// A LL(1) recursive descent parser for Newick formatted strings
// using the following simple grammar (w/o branch lengths and names):
//      START -> NODE ';'
//      NODE  -> '(' NODE ',' NODE ')' | sequence_id
impl<'a> NewickParser<'a> {

    // START -> NODE ';'
    fn start(&mut self) -> Result<Node> {
        let root = self.node()?;
        self.assert_next(';')?;
        Ok(root)
    }

    // NODE  -> '(' NODE ',' NODE ')' | sequence_id
    fn node(&mut self) -> Result<Node> {
        match self.iter.peek() {
            // Case 1: Internal node
            // NODE -> '(' NODE ',' NODE ')'
            Some(val) if *val == '(' => {
                self.assert_next('(')?;
                let left = self.node()?;
                self.assert_next(',')?;
                let right = self.node()?;
                self.assert_next(')')?;

                Ok(Node::with_children(left, right))
            }

            // Case 2: Leaf node
            // NODE -> sequence_id
            Some(val) if ('0'..='9').contains(val) => {
                let seq_id = self.assert_and_parse_sequence_id()?;

                Ok(Node::with_sequence_id(seq_id))
            }

            Some(other) => Err(anyhow!("Syntax error: Expected '(' or a number, got {other}")),
            _ => panic!(),
        }
    }

    fn assert_next(&mut self, expected: char) -> Result<()> {
        match self.iter.next() {
            Some(c) if c == expected => Ok(()),
            Some(c) => Err(anyhow!("Syntax error: Expected '{expected}', found '{c}'")),
            None => Err(anyhow!("Syntax error: Expected '{expected}', found EOF")),
        }
    }

    fn assert_and_parse_sequence_id(&mut self) -> Result<usize, ParseIntError> {
        let mut id = "".to_string();

        while let Some(c) = self.iter.peek() {
            if ('0'..='9').contains(c) {
                id.push(*c);
                self.iter.next();
            } else {
                break;
            }
        }

        let id = id.parse::<usize>()?;

        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tree() {
        let result = parse("((2,3),((1,4),5));");
        assert!(result.is_ok());
    }

    #[test]
    fn missing_semicolon() {
        let result = parse("((2,3),((1,4),5))");
        assert!(result.is_err());
    }

    #[test]
    fn too_many_children() {
        let result = parse("((2,3),((1,4,0),5));");
        assert!(result.is_err());
    }

    #[test]
    fn missing_parentheses() {
        let result = parse("((2,3),((1,4),5;");
        assert!(result.is_err());
    }

    #[test]
    fn unercognized_characters() {
        let result = parse("((a,%)+((1,4),5));");
        assert!(result.is_err());
    }
}
