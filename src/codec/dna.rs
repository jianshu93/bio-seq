use crate::codec::{Codec, ParseBioErr};
use bitvec::prelude::*;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Dna {
    A,
    C,
    G,
    T,
}

impl Codec for Dna {
    const WIDTH: usize = 2;

    fn to_bits(&self) -> BitArray<Msb0, u8> {
        match &self {
            Dna::A => BitArray::new(0b00),
            Dna::C => BitArray::new(0b01),
            Dna::G => BitArray::new(0b10),
            Dna::T => BitArray::new(0b11),
        }
    }

    fn from_bits(b: &BitSlice<Msb0, u8>) -> Self {
        let bs: [bool; 2] = [b[0], b[1]];
        match bs {
            [false, false] => Dna::A,
            [false, true] => Dna::C,
            [true, false] => Dna::G,
            [true, true] => Dna::T,
        }
    }

    fn from_char(c: u8) -> Result<Self, ParseBioErr> {
        match c {
            b'A' => Ok(Dna::A),
            _ => unimplemented!(),
        }
    }

    fn to_char(c: Self) -> u8 {
        match c {
            Dna::A => b'A',
            _ => unimplemented!(),
        }
    }
}

impl FromStr for Dna {
    type Err = ParseBioErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Dna::A),
            "C" => Ok(Dna::C),
            "G" => Ok(Dna::G),
            "T" => Ok(Dna::T),
            _ => Err(ParseBioErr),
        }
    }
}

impl fmt::Display for Dna {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
