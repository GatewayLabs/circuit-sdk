use crate::int::GarbledInt;
use crate::operations::circuits::build_and_execute_mux;
use crate::uint::GarbledUint;

impl<const N: usize> GarbledUint<N> {
    // implementation of the MUX operation
    pub fn mux(&self, if_true: &GarbledUint<N>, if_false: &GarbledUint<N>) -> GarbledUint<N> {
        build_and_execute_mux(self, if_true, if_false)
    }

    // implementation of the MUX operation
    pub fn mux3(
        condition: &GarbledUint<N>,
        if_true: &GarbledUint<N>,
        if_false: &GarbledUint<N>,
    ) -> GarbledUint<N> {
        build_and_execute_mux(condition, if_true, if_false)
    }
}

impl<const N: usize> GarbledInt<N> {
    // implementation of the MUX operation
    pub fn mux(&self, if_true: &GarbledInt<N>, if_false: &GarbledInt<N>) -> GarbledInt<N> {
        build_and_execute_mux(&self.into(), &if_true.into(), &if_false.into()).into()
    }

    // implementation of the MUX operation
    pub fn mux3(
        condition: &GarbledInt<N>,
        if_true: &GarbledInt<N>,
        if_false: &GarbledInt<N>,
    ) -> GarbledInt<N> {
        build_and_execute_mux(&condition.into(), &if_true.into(), &if_false.into()).into()
    }
}
