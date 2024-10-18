use anyhow::Result;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use tandem::states::Evaluator as TandemEvaluator;
use tandem::Circuit;

pub trait Evaluator {
    fn new(circuit: &Circuit, input: &[bool]) -> Result<Self>
    where
        Self: Sized;
    fn next(self, message: &[u8]) -> Result<(Self, Vec<u8>)>
    where
        Self: Sized;
    fn steps(&self) -> u32;
    fn is_complete(&self) -> bool;
    fn output(self, message: &[u8]) -> Result<Vec<bool>>;
}

pub struct GatewayEvaluator {
    evaluator: TandemEvaluator<Circuit, Vec<bool>>,
    steps_remaining: u32,
}

impl Evaluator for GatewayEvaluator {
    fn new(circuit: &Circuit, input: &[bool]) -> Result<Self> {
        let evaluator =
            TandemEvaluator::new(circuit.clone(), input.to_vec(), ChaCha20Rng::from_entropy())?;
        let steps_remaining = evaluator.steps();
        Ok(GatewayEvaluator {
            evaluator,
            steps_remaining,
        })
    }

    fn next(self, message: &[u8]) -> Result<(Self, Vec<u8>)> {
        let (next_state, response) = self.evaluator.run(message)?;
        let steps_remaining = self.steps_remaining - 1;
        Ok((
            GatewayEvaluator {
                evaluator: next_state,
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

    fn output(self, message: &[u8]) -> Result<Vec<bool>> {
        self.evaluator.output(message).map_err(anyhow::Error::new)
    }
}
