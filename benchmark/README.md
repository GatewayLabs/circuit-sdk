# Encrypted Computation Benchmarks

This benchmark suite compares the performance of encrypted operations for two encryption schemes: Gateway and TFHE, on a set of fundamental operations including addition, bitwise operations, comparison, and mux operations. The computations use 128-bit integers and aim to demonstrate the efficiency of these two approaches in privacy-preserving computation.

The Gateway and TFHE frameworks differ significantly in speed, as seen in the measured times. While Gateway encryption consistently completes operations in the millisecond range, TFHE operations take substantially longer, ranging from hundreds of milliseconds to several seconds. These differences underscore the suitability of Gateway encryption for performance-sensitive applications where computational overhead needs to be minimized, especially in real-time or high-throughput environments.

Each function is tested in isolation to benchmark its specific computational characteristics, and the results, with times measured in milliseconds, indicate that Gateway generally performs at a much higher speed, making it more suitable for use cases that require frequent and rapid encrypted operations.

| Operation                        | Gateway Time (ms) | TFHE Time (ms) |
|----------------------------------|-------------------|----------------|
| **Bitwise AND**                  | 36.323           | 454.66         |
| **Bitwise XOR**                  | 34.371           | 461.06         |
| **Bitwise NOT**                  | 35.107           | 309.42         |
| **NAND**                         | 38.727           | 456.45         |
| **NOR**                          | 39.228           | 457.36         |
| **XNOR**                         | 35.852           | 457.43         |
| **Bitwise OR**                   | 38.793           | 458.68         |
| **Equality (EQ)**                | 38.752           | 527.59         |
| **Inequality (NEQ)**             | 38.855           | 524.97         |
| **Greater Than (GT)**            | 47.575           | 640.32         |
| **Less Than (LT)**               | 47.785           | 642.34         |
| **Greater Than or Equal (GE)**   | 47.580           | 639.54         |
| **Less Than or Equal (LE)**      | 47.571           | 645.66         |
| **Addition**                     | 41.681           | 833.84         |
| **Subtraction**                  | 44.541           | 839.91         |
| **Multiplication**               | 1,040.8          | 15,932         |
| **Division**                     | 3,408.9          | 91,404         |
| **Modulus**                      | 3,434.3          | 87,183         |
| **MUX**                          | 36.123           | 172.44         |

_Benchmarks were run on a Apple M3 Pro with 36 GB RAM._
