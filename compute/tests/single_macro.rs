use circuit_macro::circuit;
use compute::operations::circuits::builder::CircuitBuilder;
use compute::uint::GarbledUint;

#[test]
fn test_macro_if_else() {
    #[circuit(execute)]
    fn mux_circuit(a: u8, b: u8) -> u8 {
        if a == b {
            let c = a * b;
            c + a
        } else {
            let x = a + b;
            x * x
        }
    }

    let a = 5_u8;
    let b = 7_u8;

    let result = mux_circuit(a, b);
    // assert_eq!(result, (a * b) + a);
    assert_eq!(result, (a + b) * (a + b));
}

#[test]
fn test_macro_addition() {
    #[circuit(execute)]
    fn addition_circuit(a: u8, b: u8) -> bool {
        let a = a + b;
        a == b
    }

    let a = 2_u8;
    let b = 2_u8;

    let result = addition_circuit(a, b);
    assert_eq!(result, false);
}
