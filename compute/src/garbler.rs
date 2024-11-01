use std::fmt::Debug;

use anyhow::Result;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use tandem::states::Contributor;
use tandem::Circuit;

pub trait Garbler {
    fn start(circuit: &Circuit, input: &[bool]) -> Result<(Self, Vec<u8>)>
    where
        Self: Sized;
    fn next(self, message: &[u8]) -> Result<(Self, Vec<u8>)>
    where
        Self: Sized;
    fn steps(&self) -> u32;
    fn is_complete(&self) -> bool;
}

pub struct GatewayGarbler {
    contributor: Contributor<Circuit, Vec<bool>>,
    steps_remaining: u32,
}

impl Garbler for GatewayGarbler {
    fn start(circuit: &Circuit, input: &[bool]) -> Result<(Self, Vec<u8>)> {
        let (contributor, message) =
            Contributor::new(circuit.clone(), input.to_vec(), ChaCha20Rng::from_entropy())?;
        let steps_remaining = contributor.steps();
        Ok((
            GatewayGarbler {
                contributor,
                steps_remaining,
            },
            message,
        ))
    }

    fn next(self, message: &[u8]) -> Result<(Self, Vec<u8>)> {
        let (next_state, response) = self.contributor.run(message)?;
        let steps_remaining = self.steps_remaining - 1;
        Ok((
            GatewayGarbler {
                contributor: next_state,
                steps_remaining,
            },
            response,
        ))
    }

    fn steps(&self) -> u32 {
        self.steps_remaining
    }

    fn is_complete(&self) -> bool {
        self.steps_remaining == 0
    }
}

impl Debug for GatewayGarbler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GatewayGarbler")
            .field("steps_remaining", &self.steps_remaining)
            .finish()
    }
}
