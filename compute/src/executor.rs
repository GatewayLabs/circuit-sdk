use anyhow::Result;
use once_cell::sync::Lazy;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;
use std::sync::Arc;
use tandem::states::{Contributor, Evaluator};
use tandem::Circuit;

pub trait Executor {
    /// Executes the 2 Party MPC protocol.
    ///
    /// # Arguments
    /// * `circuit` - The circuit to be evaluated.
    /// * `input_contributor` - Input provided by the contributor.
    /// * `input_evaluator` - Input provided by the evaluator.
    ///
    /// # Returns
    /// The result of the simulation as a vector of booleans.
    fn execute(
        &self,
        circuit: &Circuit,
        input_contributor: &[bool],
        input_evaluator: &[bool],
    ) -> Result<Vec<bool>>;
}

pub struct LocalSimulator;

impl Executor for LocalSimulator {
    /// The Multi-Party Computation is performed using the full cryptographic protocol exposed by the
    /// [`Contributor`] and [`Evaluator`]. The messages between contributor and evaluator are exchanged
    /// using local message queues. This function thus simulates an MPC execution on a local machine
    /// under ideal network conditions, without any latency or bandwidth restrictions.
    fn execute(
        &self,
        circuit: &Circuit,
        input_contributor: &[bool],
        input_evaluator: &[bool],
    ) -> Result<Vec<bool>> {
        let (mut contrib, mut msg_for_eval) =
            Contributor::new(circuit, input_contributor, ChaCha20Rng::from_entropy())?;

        let mut eval = Evaluator::new(
            circuit.clone(),
            input_evaluator,
            ChaCha20Rng::from_entropy(),
        )?;

        tracing::debug!("contributor ciphertext: {:?}", hex::encode(&msg_for_eval));

        assert_eq!(contrib.steps(), eval.steps());

        for _ in 0..eval.steps() {
            let (next_state, msg_for_contrib) = eval.run(&msg_for_eval)?;
            eval = next_state;

            let (next_state, reply) = contrib.run(&msg_for_contrib)?;
            contrib = next_state;

            msg_for_eval = reply;
        }
        Ok(eval.output(&msg_for_eval)?)
    }
}

/// A static Lazy instance for holding the singleton LocalSimulator.
static SINGLETON_EXECUTOR: Lazy<Arc<dyn Executor + Send + Sync>> =
    Lazy::new(|| Arc::new(LocalSimulator) as Arc<dyn Executor + Send + Sync>);

/// Provides access to the singleton Executor instance.
pub(crate) fn get_executor() -> Arc<dyn Executor + Send + Sync> {
    SINGLETON_EXECUTOR.clone()
}
