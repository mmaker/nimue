use crate::{Unit, Merlin};

use super::super::hash::DuplexHash;
use super::super::{Arthur, InvalidTag};
use super::Absorbable;
use rand::{CryptoRng, RngCore};

/// A trait that equips a function with a generic method for absorbing types.
pub trait Absorbs<L: Unit> {
    fn absorb<A: Absorbable<L>>(&mut self, e: &A) -> Result<(), InvalidTag>;

    fn absorb_slice<A: Absorbable<L>>(&mut self, input: &[A]) -> Result<(), InvalidTag> {
        input.iter().map(|e| self.absorb(e)).collect()
    }
}

impl<S, R> Absorbs<S::L> for Arthur<S, R>
where
    S: DuplexHash,
    R: RngCore + CryptoRng,
{
    fn absorb<A: Absorbable<S::L>>(&mut self, input: &A) -> Result<(), InvalidTag> {
        let input = Absorbable::<S::L>::to_absorbable(input);
        self.merlin.absorb_native(&input).map(|_| ())
    }
}

impl<S> Absorbs<S::L> for Merlin<S>
where
    S: DuplexHash,
{
    fn absorb<A: Absorbable<S::L>>(&mut self, input: &A) -> Result<(), InvalidTag> {
        let input = Absorbable::<S::L>::to_absorbable(input);
        self.absorb_native(&input).map(|_| ())
    }
}
