use crate::errors::IOPatternError;
use crate::hash::{DuplexHash, Unit};
use crate::safe::{IOPattern, Safe};
use crate::traits::{ByteTranscript, ByteTranscriptReader};
use crate::DefaultHash;

/// Merlin is wrapper around a sponge that provides a secure
/// Fiat-Shamir implementation for public-coin protocols.
#[derive(Clone)]
pub struct Merlin<'a, H = DefaultHash, U = u8>
where
    H: DuplexHash<U>,
    U: Unit,
{
    pub(crate) safe: Safe<H, U>,
    pub(crate) transcript: &'a [u8],
}

impl<'a, U: Unit, H: DuplexHash<U>> Merlin<'a, H, U> {
    /// Creates a new [`Merlin`] instance with the given sponge and IO Pattern.
    ///
    /// The resulting object will act as the verifier in a zero-knowledge protocol.
    pub fn new(io_pattern: &IOPattern<H, U>, transcript: &'a [u8]) -> Self {
        let safe = Safe::new(io_pattern);
        Self { safe, transcript }
    }

    /// Read `input.len()` elements from the transcript.
    #[inline(always)]
    pub fn fill_next(&mut self, input: &mut [U]) -> Result<(), IOPatternError> {
        U::read(&mut self.transcript, input).unwrap();
        self.safe.absorb(input)
    }

    #[inline(always)]
    pub fn public_input(&mut self, input: &[U]) -> Result<(), IOPatternError> {
        self.safe.absorb(input)
    }

    /// Get a challenge of `count` elements.
    #[inline(always)]
    pub fn fill_challenges(&mut self, input: &mut [U]) -> Result<(), IOPatternError> {
        self.safe.squeeze(input)
    }

    /// Signals the end of the statement.
    #[inline(always)]
    pub fn ratchet(&mut self) -> Result<(), IOPatternError> {
        self.safe.ratchet()
    }

    /// Signals the end of the statement and returns the (compressed) sponge state.
    #[inline(always)]
    pub fn preprocess(self) -> Result<&'static [U], IOPatternError> {
        self.safe.preprocess()
    }
}

impl<'a, H: DuplexHash<U>, U: Unit> core::fmt::Debug for Merlin<'a, H, U> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Merlin").field(&self.safe).finish()
    }
}

impl<'a, H: DuplexHash<u8>> ByteTranscript for Merlin<'a, H, u8> {
    #[inline(always)]
    fn public_bytes(&mut self, input: &[u8]) -> Result<(), IOPatternError> {
        self.public_input(input)
    }
    #[inline(always)]
    fn fill_challenge_bytes(&mut self, output: &mut [u8]) -> Result<(), IOPatternError> {
        self.fill_challenges(output)
    }
}

impl<'a, H: DuplexHash<u8>> ByteTranscriptReader for Merlin<'a, H, u8> {
    #[inline(always)]
    fn fill_next_bytes(&mut self, input: &mut [u8]) -> Result<(), IOPatternError> {
        self.fill_next(input)
    }
}
