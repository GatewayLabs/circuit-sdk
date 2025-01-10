use anyhow::Result;
use once_cell::sync::Lazy;
use std::sync::Arc;
use tandem::Circuit;

use crate::evaluator::{Evaluator, GatewayEvaluator};
use crate::garbler::{Garbler, GatewayGarbler};

use std::thread::sleep;
use std::time::Duration;

// 50 MB per second simulated latency
const DEFAULT_SIMULATED_LATENCY: f64 = 50.0 * 1024.0 * 1024.0; // bytes per second
/// A static Lazy instance for holding the singleton LocalSimulator.
static SINGLETON_EXECUTOR: Lazy<Arc<dyn Executor + Send + Sync>> = Lazy::new(|| {
    Arc::new(LocalSimulator::new(DEFAULT_SIMULATED_LATENCY)) as Arc<dyn Executor + Send + Sync>
});

/// Provides access to the singleton Executor instance.
pub fn get_executor() -> Arc<dyn Executor + Send + Sync> {
    SINGLETON_EXECUTOR.clone()
}

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

    fn instance() -> &'static Arc<dyn Executor + Send + Sync>
    where
        Self: Sized,
    {
        &SINGLETON_EXECUTOR
    }
}

pub struct LocalSimulator {
    latency: f64,
}

impl LocalSimulator {
    pub fn new(latency: f64) -> Self {
        LocalSimulator { latency }
    }
}

impl Default for LocalSimulator {
    fn default() -> Self {
        LocalSimulator {
            latency: DEFAULT_SIMULATED_LATENCY,
        }
    }
}

impl Executor for LocalSimulator {
    /// The Multi-Party Computation is performed using the full cryptographic protocol exposed by the
    /// `Contributor` and `Evaluator`. The messages between contributor and evaluator are exchanged
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
            simulate_transfer_time_ms(&msg_for_garbler, &self.latency);

            evaluator = next_evaluator;

            let (next_garbler, reply) = garbler.next(&msg_for_garbler)?;
            simulate_transfer_time_ms(&reply, &self.latency);
            garbler = next_garbler;

            msg_for_evaluator = reply;
        }

        let output = evaluator.output(&msg_for_evaluator)?;
        Ok(output)
    }
}

fn simulate_transfer_time_ms(payload: &Vec<u8>, latency: &f64) {
    let bytes_size = payload.len() as f64;
    let transfer_time = bytes_size / latency * 1000.0; // Convert seconds to milliseconds
    sleep(Duration::from_millis(transfer_time as u64))
}
