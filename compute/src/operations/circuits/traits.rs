/// A trait that defines a set of logical and arithmetic operations to be implemented for a custom circuit executor type.
///
/// This trait is designed to provide generic functionality for various kinds of operations
/// that work on individual types and vectors of types. The trait assumes the existence of two
/// associated types:
/// - `Type`: Represents an individual element used in operations.
/// - `TypeVec`: Represents a vector of `Type`, used for operations on collections.
///
/// # Associated Types
///
/// - Type: The base type for individual operations.
/// - TypeVec: A vector of the base type, used for operations involving collections.
///
/// # Required Methods
///
/// Each method must be implemented to provide the desired functionality.
pub trait CircuitExecutor {
    /// The base type for individual elements in operations.
    type Type;

    /// A vector type for collections of `Type`.
    type TypeVec;

    /// Performs a bitwise XOR operation on two vectors of `Type`.
    ///
    /// # Parameters
    ///
    /// - `a`: A reference to the first vector of elements.
    /// - `b`: A reference to the second vector of elements.
    ///
    /// # Returns
    ///
    /// A vector of elements resulting from the bitwise XOR operation.
    fn xor(&mut self, a: &Self::TypeVec, b: &Self::TypeVec) -> Self::TypeVec;

    /// Performs a bitwise AND operation on two vectors of `Type`.
    ///
    /// # Parameters
    ///
    /// - `a`: A reference to the first vector of elements.
    /// - `b`: A reference to the second vector of elements.
    ///
    /// # Returns
    ///
    /// A vector of elements resulting from the bitwise AND operation.
    fn and(&mut self, a: &Self::TypeVec, b: &Self::TypeVec) -> Self::TypeVec;

    /// Performs a bitwise OR operation on two vectors of `Type`.
    ///
    /// # Parameters
    ///
    /// - `a`: A reference to the first vector of elements.
    /// - `b`: A reference to the second vector of elements.
    ///
    /// # Returns
    ///
    /// A vector of elements resulting from the bitwise OR operation.
    fn or(&mut self, a: &Self::TypeVec, b: &Self::TypeVec) -> Self::TypeVec;

    /// Performs a bitwise NOT operation on a vector of `Type`.
    ///
    /// # Parameters
    ///
    /// - `a`: A reference to the vector of elements.
    ///
    /// # Returns
    ///
    /// A vector of elements resulting from the bitwise NOT operation.
    fn not(&mut self, a: &Self::TypeVec) -> Self::TypeVec;

    /// Performs a multiplexer operation, selecting values from one of two vectors based on a control bit.
    ///
    /// # Parameters
    ///
    /// - `s`: The control bit.
    /// - `a`: A reference to the first vector of elements.
    /// - `b`: A reference to the second vector of elements.
    ///
    /// # Returns
    ///
    /// A vector of elements selected based on the control bit.
    fn mux(&mut self, s: &Self::Type, a: &Self::TypeVec, b: &Self::TypeVec) -> Self::TypeVec;

    /// Performs an addition operation on two vectors of `Type`.
    ///
    /// # Parameters
    ///
    /// - `a`: A reference to the first vector of elements.
    /// - `b`: A reference to the second vector of elements.
    ///
    /// # Returns
    ///
    /// A vector of elements representing the sum of the input vectors.
    fn add(&mut self, a: &Self::TypeVec, b: &Self::TypeVec) -> Self::TypeVec;

    /// Performs a subtraction operation on two vectors of `Type`.
    ///
    /// # Parameters
    ///
    /// - `a`: A reference to the first vector of elements.
    /// - `b`: A reference to the second vector of elements.
    ///
    /// # Returns
    ///
    /// A vector of elements representing the difference of the input vectors.
    fn sub(&mut self, a: &Self::TypeVec, b: &Self::TypeVec) -> Self::TypeVec;

    /// Performs a multiplication operation on two vectors of `Type`.
    ///
    /// # Parameters
    ///
    /// - `a`: A reference to the first vector of elements.
    /// - `b`: A reference to the second vector of elements.
    ///
    /// # Returns
    ///
    /// A vector of elements representing the product of the input vectors.
    fn mul(&mut self, a: &Self::TypeVec, b: &Self::TypeVec) -> Self::TypeVec;

    /// Performs a division operation on two vectors of `Type`.
    ///
    /// # Parameters
    ///
    /// - `a`: A reference to the dividend vector.
    /// - `b`: A reference to the divisor vector.
    ///
    /// # Returns
    ///
    /// A vector of elements representing the quotient of the division.
    fn div(&mut self, a: &Self::TypeVec, b: &Self::TypeVec) -> Self::TypeVec;

    /// Performs a remainder operation on two vectors of `Type`.
    ///
    /// # Parameters
    ///
    /// - `a`: A reference to the dividend vector.
    /// - `b`: A reference to the divisor vector.
    ///
    /// # Returns
    ///
    /// A vector of elements representing the remainder of the division.
    fn rem(&mut self, a: &Self::TypeVec, b: &Self::TypeVec) -> Self::TypeVec;

    /// Checks the equality of two vectors of `Type`.
    ///
    /// # Parameters
    ///
    /// - `a`: A reference to the first vector.
    /// - `b`: A reference to the second vector.
    ///
    /// # Returns
    ///
    /// A single element of type `Type` representing the equality (`1` for true, `0` for false).
    fn eq(&mut self, a: &Self::TypeVec, b: &Self::TypeVec) -> Self::Type;

    /// Checks the inequality of two vectors of `Type`.
    ///
    /// # Parameters
    ///
    /// - `a`: A reference to the first vector.
    /// - `b`: A reference to the second vector.
    ///
    /// # Returns
    ///
    /// A single element of type `Type` representing the inequality (`1` for true, `0` for false).
    fn ne(&mut self, a: &Self::TypeVec, b: &Self::TypeVec) -> Self::Type;

    /// Performs a comparison of two vectors and returns whether the first is greater than the second.
    ///
    /// # Parameters
    ///
    /// - `a`: A reference to the first vector.
    /// - `b`: A reference to the second vector.
    ///
    /// # Returns
    ///
    /// A single element of type `Type` representing the comparison result (`1` for true, `0` for false).
    fn gt(&mut self, a: &Self::TypeVec, b: &Self::TypeVec) -> Self::Type;

    /// Checks if the first vector is greater than or equal to the second vector.
    ///
    /// # Parameters
    ///
    /// - `a`: A reference to the first vector of elements.
    /// - `b`: A reference to the second vector of elements.
    ///
    /// # Returns
    ///
    /// A single element of type `Type` representing the result (`1` for true, `0` for false).
    fn ge(&mut self, a: &Self::TypeVec, b: &Self::TypeVec) -> Self::Type;

    /// Checks if the first vector is less than the second vector.
    ///
    /// # Parameters
    ///
    /// - `a`: A reference to the first vector of elements.
    /// - `b`: A reference to the second vector of elements.
    ///
    /// # Returns
    ///
    /// A single element of type `Type` representing the result (`1` for true, `0` for false).
    fn lt(&mut self, a: &Self::TypeVec, b: &Self::TypeVec) -> Self::Type;

    /// Checks if the first vector is less than or equal to the second vector.
    ///
    /// # Parameters
    ///
    /// - `a`: A reference to the first vector of elements.
    /// - `b`: A reference to the second vector of elements.
    ///
    /// # Returns
    ///
    /// A single element of type `Type` representing the result (`1` for true, `0` for false).
    fn le(&mut self, a: &Self::TypeVec, b: &Self::TypeVec) -> Self::Type;

    /// Compares two vectors and returns a tuple indicating the comparison result.
    ///
    /// # Parameters
    ///
    /// - `a`: A reference to the first vector of elements.
    /// - `b`: A reference to the second vector of elements.
    ///
    /// # Returns
    ///
    /// A tuple containing two elements of type `Type`:
    /// - The first element indicates if the first vector is less than the second (`1` for true, `0` for false).
    /// - The second element indicates if the vectors are equal (`1` for true, `0` for false).
    fn compare(&mut self, a: &Self::TypeVec, b: &Self::TypeVec) -> (Self::Type, Self::Type);

    /// Performs a NAND (NOT AND) operation on two vectors of `Type`.
    ///
    /// # Parameters
    ///
    /// - `a`: A reference to the first vector of elements.
    /// - `b`: A reference to the second vector of elements.
    ///
    /// # Returns
    ///
    /// A vector of elements resulting from the NAND operation.
    fn nand(&mut self, a: &Self::TypeVec, b: &Self::TypeVec) -> Self::TypeVec;

    /// Performs a logical AND operation on two single elements of type `Type`.
    ///
    /// # Parameters
    ///
    /// - `a`: A reference to the first element.
    /// - `b`: A reference to the second element.
    ///
    /// # Returns
    ///
    /// A single element of type `Type` resulting from the logical AND operation.
    fn land(&mut self, a: &Self::Type, b: &Self::Type) -> Self::Type;

    /// Performs a NOR (NOT OR) operation on two vectors of `Type`.
    ///
    /// # Parameters
    ///
    /// - `a`: A reference to the first vector of elements.
    /// - `b`: A reference to the second vector of elements.
    ///
    /// # Returns
    ///
    /// A vector of elements resulting from the NOR operation.
    fn nor(&mut self, a: &Self::TypeVec, b: &Self::TypeVec) -> Self::TypeVec;

    /// Performs a XNOR (NOT XOR) operation on two vectors of `Type`.
    ///
    /// # Parameters
    ///
    /// - `a`: A reference to the first vector of elements.
    /// - `b`: A reference to the second vector of elements.
    ///
    /// # Returns
    ///
    /// A vector of elements resulting from the XNOR operation.
    fn xnor(&mut self, a: &Self::TypeVec, b: &Self::TypeVec) -> Self::TypeVec;

    /// Performs a logical OR operation on two vectors of `Type` and returns a single result.
    ///
    /// # Parameters
    ///
    /// - `a`: A reference to the first vector of elements.
    /// - `b`: A reference to the second vector of elements.
    ///
    /// # Returns
    ///
    /// A single element of type `Type` representing the logical OR result across the input vectors.
    fn lor(&mut self, a: &Self::TypeVec, b: &Self::TypeVec) -> Self::Type;
}
