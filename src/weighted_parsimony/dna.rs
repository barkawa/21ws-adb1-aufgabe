use anyhow::{anyhow, Result};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Base {
    A,
    C,
    T,
    G,
}

impl TryFrom<char> for Base {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            'A' => Ok(Self::A),
            'C' => Ok(Self::C),
            'T' => Ok(Self::T),
            'G' => Ok(Self::G),
            other => Err(anyhow!(
                "Not a nucleotide: Found '{other}', expected one of [A, C, T, G]"
            )),
        }
    }
}