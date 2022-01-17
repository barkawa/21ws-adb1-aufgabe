use std::{iter::Peekable, num::ParseIntError};

use anyhow::{Result, anyhow};


pub fn get_min_score() -> Result<f64> {
    // open seq file
    // parse seq file into ndarray

    // open matrix file
    // parse matrix file into ndarray

    // open tree file
    // parse and construct closure

    // for col in sequences.cols
    // do the thing

    Ok(420.69)
}



pub struct NewickParser<'a> {
    iter: &'a mut Peekable<std::str::Chars<'a>>,
}

impl<'a> NewickParser<'a> {
    pub fn new(iter: &'a mut Peekable<std::str::Chars<'a>>) -> Self { 
        Self { iter } 
    }

    // Simple grammar for the Newick-Format without branch lengths and names:
    //      START -> NODE ';'
    //      NODE  -> '(' NODE ',' NODE ')' | sequence_id
    //
    pub fn parse(&mut self) -> Result<()> /* -> Result<impl Fn([u8]) -> f64> */ {
        self.start()
    }

    fn start(&mut self) -> Result<()> {
        self.node()?;
        self.assert_next(';')?;
        Ok(())
    }

    fn node(&mut self) -> Result<()> {
        match self.iter.peek() {

            // NODE -> '(' NODE ',' NODE ')'
            Some(val) if *val == '(' => {
                self.assert_next('(')?;
                self.node()?;
                self.assert_next(',')?;
                self.node()?;
                self.assert_next(')')?;
                Ok(())
            },

            // NODE -> sequence_id
            Some(val) if ('0'..='9').contains(val) =>  {
                let seq_id = self.assert_and_parse_sequence_id()?;
                println!("sequence id: {seq_id}");
                Ok(())
            },

            _ => Err(anyhow!("Syntax error")),
        }
    }


    fn assert_next(&mut self, c: char) -> Result<()> {
        
        match self.iter.next() {
            Some(val) if val == c => Ok(()),
            Some(val) => Err(anyhow!("Syntax error at {}", val)),
            None => Err(anyhow!("Syntax error")),
        }
    }


    fn assert_and_parse_sequence_id(&mut self) -> Result<u32, ParseIntError> {
        
        let mut id = "".to_string();
        
        while let Some(c) = self.iter.peek() {
            if ('0'..='9').contains(c) {
                id.push(*c);
                self.iter.next();
            } else {
                break;
            }
        }

        id.parse::<u32>()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tree() {
        let string = "((2,3),((1,4),5));";
        let mut iter = string.chars().peekable();
        let mut parser = NewickParser::new(&mut iter);
        assert!(parser.parse().is_ok());
    }

    #[test]
    fn wrong() {
        let string = "(1bla((2,3),4))";
        let mut iter = string.chars().peekable();
        let mut parser = NewickParser::new(&mut iter);
        assert!(parser.parse().is_err());
    }
}