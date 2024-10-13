use crate::operations::circuits::build_and_execute_mux;
use crate::uint::GarbledUint;

impl<const N: usize> GarbledUint<N> {
    // implementation of the MUX operation
    pub fn mux(&self, if_true: &GarbledUint<N>, if_false: &GarbledUint<N>) -> GarbledUint<N> {
        build_and_execute_mux(&self, &if_true, &if_false)
    }

    pub fn mux3(
        condition: &GarbledUint<N>,
        if_true: &GarbledUint<N>,
        if_false: &GarbledUint<N>,
    ) -> GarbledUint<N> {
        build_and_execute_mux(condition, if_true, if_false)
    }
}
