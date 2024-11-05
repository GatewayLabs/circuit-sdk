use serde::{Deserialize, Serialize};
use tandem::Circuit;
use tandem::Gate;
use tandem::GateIndex;

// wrapper Gate
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GateW {
    /// A single input bit coming from the circuit contributor.
    InContrib,
    /// A single input bit coming from the circuit evaluator.
    InEval,
    /// A gate computing the XOR of the two specified gates.
    Xor(GateIndex, GateIndex),
    /// A gate computing the AND of the two specified gates.
    And(GateIndex, GateIndex),
    /// A gate computing the NOT of the specified gate.
    Not(GateIndex),
}

impl Into<Gate> for GateW {
    fn into(self) -> Gate {
        match self {
            GateW::InContrib => Gate::InContrib,
            GateW::InEval => Gate::InEval,
            GateW::Xor(a, b) => Gate::Xor(a, b),
            GateW::And(a, b) => Gate::And(a, b),
            GateW::Not(a) => Gate::Not(a),
        }
    }
}

impl From<Gate> for GateW {
    fn from(gate: Gate) -> Self {
        match gate {
            Gate::InContrib => GateW::InContrib,
            Gate::InEval => GateW::InEval,
            Gate::Xor(a, b) => GateW::Xor(a, b),
            Gate::And(a, b) => GateW::And(a, b),
            Gate::Not(a) => GateW::Not(a),
        }
    }
}

// Assuming `Gate` and `GateIndex` implement `Serialize` and `Deserialize`
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CircuitWrapper {
    gates: Vec<GateW>,
    output_gates: Vec<GateIndex>,
    and_gates: usize,
    eval_inputs: usize,
    contrib_inputs: usize,
}

// Implement conversions from `Circuit` to `CircuitWrapper` and vice versa
impl From<&Circuit> for CircuitWrapper {
    fn from(circuit: &Circuit) -> Self {
        CircuitWrapper {
            gates: circuit
                .gates()
                .iter()
                .map(|gate| gate.clone().into())
                .collect(),
            output_gates: circuit.output_gates().clone(),
            and_gates: circuit.and_gates(),
            eval_inputs: circuit.eval_inputs(),
            contrib_inputs: circuit.contrib_inputs(),
        }
    }
}

impl Into<Circuit> for CircuitWrapper {
    fn into(self) -> Circuit {
        Circuit::new(
            self.gates.iter().map(|gate| gate.clone().into()).collect(),
            self.output_gates,
        )
    }
}

pub fn serialize_circuit(circuit: &Circuit) -> anyhow::Result<Vec<u8>> {
    // Convert `Circuit` to `CircuitWrapper`
    let wrapper: CircuitWrapper = circuit.into();

    // Serialize `CircuitWrapper` using bincode
    let serialized_data = bincode::serialize(&wrapper)?;
    Ok(serialized_data)
}

pub fn deserialize_circuit(data: &[u8]) -> anyhow::Result<Circuit> {
    // Deserialize into `CircuitWrapper`
    let wrapper: CircuitWrapper = bincode::deserialize(data)?;

    // Convert `CircuitWrapper` back into `Circuit`
    let circuit: Circuit = wrapper.into();
    Ok(circuit)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_serialize_deserialize_circuit_struct() -> anyhow::Result<()> {
        #[circuit(compile)]
        fn multi_arithmetic(a: u8, b: u8, c: u8, d: u8) -> u8 {
            let res = a * b;
            let res = res + c;
            res - d
        }

        // Initialize the evaluator instance with circuit and dummy input
        let (circuit, _) = multi_arithmetic(0_u8, 0_u8, 0_u8, 0_u8);

        // Serialize the circuit
        let serialized_data = serialize_circuit(&circuit)?;
        println!("Serialized Circuit data: {:?}", serialized_data);

        // Deserialize back into a `Circuit` struct
        let deserialized_circuit = deserialize_circuit(&serialized_data)?;
        println!("Deserialized Circuit: {:?}", deserialized_circuit);

        // Check if the deserialized circuit is the same as the original circuit
        assert_eq!(circuit.gates(), deserialized_circuit.gates());
        assert_eq!(circuit.output_gates(), deserialized_circuit.output_gates());
        assert_eq!(circuit.and_gates(), deserialized_circuit.and_gates());
        assert_eq!(circuit.eval_inputs(), deserialized_circuit.eval_inputs());
        assert_eq!(
            circuit.contrib_inputs(),
            deserialized_circuit.contrib_inputs()
        );

        Ok(())
    }
}
