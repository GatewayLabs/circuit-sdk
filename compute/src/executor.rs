use anyhow::Result;
use once_cell::sync::Lazy;
use std::sync::Arc;
use tandem::Circuit;

use crate::evaluator::{Evaluator, GatewayEvaluator};
use crate::garbler::{Garbler, GatewayGarbler};

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
        input_garbler: &[bool],
        input_evaluator: &[bool],
    ) -> Result<Vec<bool>> {
        let (mut garbler, mut msg_for_evaluator) = GatewayGarbler::start(circuit, input_garbler)?;

        let mut evaluator = GatewayEvaluator::new(circuit, input_evaluator)?;

        assert_eq!(garbler.steps(), evaluator.steps());
        let total_steps = garbler.steps();

        for _ in 0..total_steps {
            let (next_evaluator, msg_for_garbler) = evaluator.next(&msg_for_evaluator)?;
            evaluator = next_evaluator;

            let (next_garbler, reply) = garbler.next(&msg_for_garbler)?;
            garbler = next_garbler;

            msg_for_evaluator = reply;
        }

        let output = evaluator.output(&msg_for_evaluator)?;
        Ok(output)
    }
}

/// A static Lazy instance for holding the singleton LocalSimulator.
static SINGLETON_EXECUTOR: Lazy<Arc<dyn Executor + Send + Sync>> =
    Lazy::new(|| Arc::new(LocalSimulator) as Arc<dyn Executor + Send + Sync>);

/// Provides access to the singleton Executor instance.
pub(crate) fn get_executor() -> Arc<dyn Executor + Send + Sync> {
    SINGLETON_EXECUTOR.clone()
}
