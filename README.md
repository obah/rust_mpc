# Multiparty Computation (MPC) in Rust

This repository provides two Rust-based implementations of secret sharing techniques for secure Multiparty Computation (MPC): **Shamir's Secret Sharing** and **Additive Secret Sharing**. These methods allow multiple parties to compute functions over shared data without revealing their individual inputs.

## Overview

Multiparty Computation (MPC) enables secure computation where multiple participants jointly compute a function over their private inputs while keeping those inputs hidden.

This project includes two primary modules:

1. **Shamir's Secret Sharing**:

   - Uses polynomial-based secret sharing to distribute a secret among `n` participants.
   - Requires a threshold of participants to reconstruct the secret.
   - Supports addition and multiplication operations on shared secrets.

2. **Additive Secret Sharing**:
   - Distributes a secret as the sum of random shares.
   - Simple and efficient, ideal for addition-based computations.

---

## Code Structure

### Modules

1. **`shamir_sharing.rs`**
   Implements Shamir's Secret Sharing.

   **Key Functions:**

   - `calc_mod`: Computes the modular value while handling negative numbers.
   - `polynomial_eval`: Evaluates a polynomial at a given point.
   - `polynomial_multiply`: Multiplies two polynomials.
   - `share`: Generates shares of a secret using random polynomial coefficients.
   - `plot`: Simulates visualization of shares and polynomials.
   - `shamir_sharing`: Demonstrates addition and multiplication operations on shared secrets.

2. **`addictive_sharing.rs`**
   Implements Additive Secret Sharing.

   **Key Functions:**

   - `calc_mod`: Computes the modular value while handling negative numbers.
   - `share`: Distributes a secret as random shares summing to the original secret.
   - `open`: Reconstructs the secret by summing shares.
   - `sum_received_shares`: Computes the sum of received shares.
   - `addictive_sharing`: Demonstrates secure addition of inputs using additive sharing.

### Main File

The `main.rs` file imports both modules and serves as the entry point to run demonstrations of the two secret sharing techniques.

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) installed on your system.

### Running the Code

1. Clone the repository:

   ```bash
   git clone https://github.com/obah/rust_mpc.git
   cd rust_mpc
   ```

2. Run:

   ```bash
   cargo run
   ```

---

## Examples

### Shamir's Secret Sharing

- Distributes a secret using a polynomial and reconstructs it from shares.
- Supports secure addition and multiplication of shared secrets.

#### Sample Output:

```text
Polynomial 1: 5 + 12 x^1 + 8 x^2
Polynomial 2: 2 + 4 x^1 + 11 x^2
Combined Polynomial: 7 + 16 x^1 + 19 x^2
Total sum: 12
```

### Additive Secret Sharing

- Distributes a secret into additive shares.
- Reconstructs the secret by summing all shares.

#### Sample Output:

```text
Input from: Alice = [3, -5, 22], Bob = [-1, 4, 12], Chris = [20, -7, -3]
Total received shares: Alice = 22, Bob = 12, Chris = 30
Total sum: 64
```

---

## Key Features

1. **Modular Arithmetic**: Ensures all operations are performed within a finite field defined by a prime number.
2. **Secure Sharing**: Secrets are split into shares that reveal no information when viewed individually.
3. **Flexible Computations**: Supports both addition and multiplication for shared secrets.
4. **Visualization (Simulated)**: Demonstrates the relationship between shares and their underlying polynomials.

---

Enjoy secure computation with Rust!
