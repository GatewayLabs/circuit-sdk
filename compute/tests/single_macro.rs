use compute::prelude::*;

#[test]
fn testy() {
    use compute::prelude::*;

    #[circuit(execute)]
    fn mux(a: bool) -> bool {
        let x = false;
        let y = true;

        if a {
            x
        } else {
            y
        }
    }

    let bool1 = true;

    let result = mux(bool1);
    assert!(!result);
}
