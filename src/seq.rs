use crate::alphabet::Alphabet;
use bitvec::prelude::*;
use std::fmt;
use std::marker::PhantomData;

pub struct Seq<A: Alphabet> {
    //<A: Alphabet> {
    bv: BitVec,
    _p: PhantomData<A>,
    //width: usize,
}

pub struct SeqSlice<'a, A: Alphabet> {
    //<A: Alphabet> {
    bv: &'a BitSlice,
    _p: PhantomData<A>,
    //width: usize,
}

impl<A: Alphabet> Seq<A> {
    pub fn from_vec(vec: Vec<A>) -> Self {
        let mut bv: BitVec = BitVec::new();
        for b in vec.iter() {
            bv.extend(b.to_bits());
        }
        Seq {
            bv: bv,
            _p: PhantomData,
        }
    }

    //    pub fn from_string(s: String) -> Self { }

    pub fn to_usize(&self) -> usize {
        self.bv.as_raw_slice()[0]
    }
}

impl<A: Alphabet + fmt::Debug> fmt::Display for Seq<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut v = Vec::new();
        let w = A::width();
        //        for c in self.bv.chunks(2) {
        for i in 0..(self.bv.len() / A::width()) {
            v.push(A::from_bits(&self.bv[(i * w)..((i * w) + w)]));

            //            v.push(c.as_raw_slice()[0]);
        }
        write!(
            f,
            "{:?}",
            v,
            //&self.bv[(4 * w)..((4 * w) + w)][2],
        )
    }
}

//impl<A> std::ops::Index<usize> for Seq<A>
//where
//    //Idx: std::ops::Index<usize>,
//    A: Alphabet,
//{
//    type Output = SeqSlice<A>;
//
//    fn index(&self, i: usize) -> Self::Output {
//        let w = A::width();
//        SeqSlice { bv: &self.bv[i*w..(i*w)+w], _p: PhantomData }
//    }
//}

macro_rules! dna {
    [$seq:literal] => {
       Seq::from_vec($seq)
    };
}

macro_rules! iupac {
    [$seq:literal] => {
       Seq::from_vec($seq)
    };
}

macro_rules! amino {
    [$seq:literal] => {
       Seq::from_vec($seq)
    };
}

//impl Iterator for Seq<A> {
//    fn next(&mut self) -> Option<A> {
//        A::from_bits(self.bv.pop(A::width()))
//    }
//}