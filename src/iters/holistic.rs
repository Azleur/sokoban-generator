use std::marker::PhantomData;

use crate::base::{Board, BaseIter, SecondaryIter};

use crate::iters::empty::{SerialMaker, RngMaker};
use crate::iters::filled::{SerialFiller, RngFiller};
use crate::iters::filter::Symmetries;
use crate::iters::solvable::Solvable;

pub fn serial(size: u8) -> ExhaustiveConnector<Symmetries<SerialMaker>, Symmetries<SerialFiller>> {
    ExhaustiveConnector::<Symmetries<SerialMaker>, Symmetries<SerialFiller>>::new(size).unwrap()
}

pub fn random(size: u8) -> ConsumingConnector<Symmetries<RngMaker>, Symmetries<RngFiller>> {
    ConsumingConnector::<Symmetries<RngMaker>, Symmetries<RngFiller>>::new(size)
}

pub fn solvable_serial(size: u8) -> ExhaustiveConnector<Symmetries<SerialMaker>, Symmetries<Solvable<SerialFiller>>> {
    ExhaustiveConnector::<Symmetries<SerialMaker>, Symmetries<Solvable<SerialFiller>>>::new(size).unwrap()
}

pub fn solvable_random(size: u8) -> ConsumingConnector<Symmetries<RngMaker>, Symmetries<Solvable<RngFiller>>> {
    ConsumingConnector::<Symmetries<RngMaker>, Symmetries<Solvable<RngFiller>>>::new(size)
}

pub struct ExhaustiveConnector<T: BaseIter, U: SecondaryIter> {
    base: T,
    modifier: U,
}

impl<T: BaseIter, U: SecondaryIter> ExhaustiveConnector<T, U> {
    pub fn new(size: u8) -> Option<Self> {
        let mut base = T::new_base(size);
        let board = base.next()?;
        let modifier = U::new_secondary(board);

        return Some(ExhaustiveConnector { base, modifier });
    }
}

impl<T: BaseIter, U: SecondaryIter> Iterator for ExhaustiveConnector<T, U> {
    type Item = Board;

    fn next(&mut self) -> Option<Board> {
        loop {
            if let Some(out) = self.modifier.next() {
                return Some(out);
            } else if let Some(empty) = self.base.next() {
                self.modifier = U::new_secondary(empty);
                continue;
            } else {
                return None;
            }
        }
    }
}

pub struct ConsumingConnector<T: BaseIter, U: SecondaryIter> {
    base: T,
    phantom: PhantomData<U>,
}

impl<T: BaseIter, U: SecondaryIter> ConsumingConnector<T, U> {
    pub fn new(size: u8) -> Self {
        ConsumingConnector {
            base: T::new_base(size),
            phantom: PhantomData,
        }
    }
}

impl<T: BaseIter, U: SecondaryIter> Iterator for ConsumingConnector<T, U> {
    type Item = Board;

    fn next(&mut self) -> Option<Board> {
        if let Some(empty) = self.base.next() {
            if let Some(filled) = U::new_secondary(empty).next() {
                return Some(filled);
            }
        }

        return None;
    }
}