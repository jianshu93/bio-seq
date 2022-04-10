use crate::codec::{Codec, ParseBioErr};
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Codec)]
//#[width = 2]
#[repr(u8)]
pub enum Dna {
    A = 0b00,
    C = 0b01,
    G = 0b10,
    T = 0b11,
}

/*
impl codec::Codec for Dna {
    const WIDTH: u8 = 2;
    fn unsafe_from_bits(b: u8) -> Self {
        match b {
            _ => Self::A,
        }
    }
    fn try_from_bits(b: u8) -> Result<Self, codec::ParseBioErr> {
        match b {
            _ => Err(ParseBioErr),
        }
    }
    fn from_char(c: char) -> Result<Self, codec::ParseBioErr> {
        Ok(Self::A)
    }
    fn to_char(a: Self) -> char {
        match a {
            _ => 'A',
        }
    }
}
*/
impl From<Dna> for char {
    fn from(dna: Dna) -> char {
        match dna {
            Dna::A => 'A',
            Dna::C => 'C',
            Dna::G => 'G',
            Dna::T => 'T',
        }
    }
}

impl TryFrom<char> for Dna {
    type Error = ParseBioErr;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(Dna::A),
            'C' => Ok(Dna::C),
            'G' => Ok(Dna::G),
            'T' => Ok(Dna::T),
            _ => Err(ParseBioErr),
        }
    }
}

impl From<u8> for Dna {
    fn from(b: u8) -> Self {
        match b {
            0b00 => Dna::A,
            0b01 => Dna::C,
            0b10 => Dna::G,
            0b11 => Dna::T,
            _ => Dna::A, // unsafe mode
        }
    }
}

impl From<Dna> for u8 {
    fn from(dna: Dna) -> Self {
        dna as u8
    }
}

impl FromStr for Dna {
    type Err = ParseBioErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Dna::try_from(s.as_bytes()[0] as char)
    }
}

impl fmt::Display for Dna {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
