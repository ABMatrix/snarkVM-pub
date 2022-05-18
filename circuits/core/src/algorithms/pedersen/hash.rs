// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use super::*;

impl<E: Environment, const NUM_WINDOWS: usize, const WINDOW_SIZE: usize> Hash
    for Pedersen<E, NUM_WINDOWS, WINDOW_SIZE>
{
    type Input = Boolean<E>;
    type Output = Field<E>;

    /// Returns the Pedersen hash of the given input as a field element.
    fn hash(&self, input: &[Self::Input]) -> Self::Output {
        // Compute the Pedersen hash as an affine group element, and return the x-coordinate.
        self.hash_uncompressed(input).to_x_coordinate()
    }
}

impl<E: Environment, const NUM_WINDOWS: usize, const WINDOW_SIZE: usize>
    Metadata<dyn Hash<Input = Boolean<E>, Output = Field<E>>> for Pedersen<E, NUM_WINDOWS, WINDOW_SIZE>
{
    type Case = (Vec<Vec<CircuitType<Group<E>>>>, Vec<CircuitType<Boolean<E>>>);
    type OutputType = CircuitType<Field<E>>;

    #[inline]
    fn count(case: &Self::Case) -> Count {
        count!(Pedersen<E, NUM_WINDOWS, WINDOW_SIZE>, HashUncompressed<Input = Boolean<E>, Output = Group<E>>, case)
    }

    #[inline]
    fn output_type(case: Self::Case) -> Self::OutputType {
        let hash_uncompressed_type = output_type!(Pedersen<E, NUM_WINDOWS, WINDOW_SIZE>, HashUncompressed<Input = Boolean<E>, Output = Group<E>>, case);
        match hash_uncompressed_type.is_constant() {
            true => CircuitType::from(hash_uncompressed_type.circuit().to_x_coordinate()),
            false => CircuitType::Private,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use snarkvm_algorithms::{crh::PedersenCompressedCRH, CRH};
    use snarkvm_circuits_environment::Circuit;
    use snarkvm_curves::AffineCurve;
    use snarkvm_utilities::{test_rng, UniformRand};

    const ITERATIONS: u64 = 10;
    const MESSAGE: &str = "PedersenCircuit0";
    const WINDOW_SIZE_MULTIPLIER: usize = 8;

    type Projective = <<Circuit as Environment>::Affine as AffineCurve>::Projective;

    fn check_hash<const NUM_WINDOWS: usize, const WINDOW_SIZE: usize>(mode: Mode) {
        // Initialize the Pedersen hash.
        let native = PedersenCompressedCRH::<Projective, NUM_WINDOWS, WINDOW_SIZE>::setup(MESSAGE);
        let circuit = Pedersen::<Circuit, NUM_WINDOWS, WINDOW_SIZE>::setup(MESSAGE);
        // Determine the number of inputs.
        let num_input_bits = NUM_WINDOWS * WINDOW_SIZE;

        for i in 0..ITERATIONS {
            // Sample a random input.
            let input = (0..num_input_bits).map(|_| bool::rand(&mut test_rng())).collect::<Vec<bool>>();
            // Compute the expected hash.
            let expected = native.hash(&input).expect("Failed to hash native input");
            // Prepare the circuit input.
            let circuit_input: Vec<Boolean<_>> = Inject::new(mode, input);

            Circuit::scope(format!("Pedersen {mode} {i}"), || {
                // Perform the hash operation.
                let candidate = circuit.hash(&circuit_input);
                assert_eq!(expected, candidate.eject_value());

                // Check constraint counts and output mode.
                let bases: Vec<Vec<CircuitType<Group<Circuit>>>> =
                    circuit.bases.iter().map(|b| b.iter().map(|b| CircuitType::from(b)).collect()).collect();
                let input = circuit_input.iter().map(|b| CircuitType::from(b)).collect::<Vec<_>>();
                let case = (bases, input);
                assert_count!(
                    Pedersen<Circuit, NUM_WINDOWS, WINDOW_SIZE>,
                    Hash<Input = Boolean<Circuit>, Output = Field<Circuit>>,
                    &case
                );
                assert_output_type!(
                    Pedersen<Circuit, NUM_WINDOWS, WINDOW_SIZE>,
                    Hash<Input = Boolean<Circuit>, Output = Field<Circuit>>,
                    case,
                    candidate
                );
            });
        }
    }

    #[test]
    fn test_hash_constant() {
        // Set the number of windows, and modulate the window size.
        check_hash::<1, WINDOW_SIZE_MULTIPLIER>(Mode::Constant);
        check_hash::<1, { 2 * WINDOW_SIZE_MULTIPLIER }>(Mode::Constant);
        check_hash::<1, { 3 * WINDOW_SIZE_MULTIPLIER }>(Mode::Constant);
        check_hash::<1, { 4 * WINDOW_SIZE_MULTIPLIER }>(Mode::Constant);
        check_hash::<1, { 5 * WINDOW_SIZE_MULTIPLIER }>(Mode::Constant);

        // Set the window size, and modulate the number of windows.
        check_hash::<1, WINDOW_SIZE_MULTIPLIER>(Mode::Constant);
        check_hash::<2, WINDOW_SIZE_MULTIPLIER>(Mode::Constant);
        check_hash::<3, WINDOW_SIZE_MULTIPLIER>(Mode::Constant);
        check_hash::<4, WINDOW_SIZE_MULTIPLIER>(Mode::Constant);
        check_hash::<5, WINDOW_SIZE_MULTIPLIER>(Mode::Constant);
    }

    #[test]
    fn test_hash_public() {
        // Set the number of windows, and modulate the window size.
        check_hash::<1, WINDOW_SIZE_MULTIPLIER>(Mode::Public);
        check_hash::<1, { 2 * WINDOW_SIZE_MULTIPLIER }>(Mode::Public);
        check_hash::<1, { 3 * WINDOW_SIZE_MULTIPLIER }>(Mode::Public);
        check_hash::<1, { 4 * WINDOW_SIZE_MULTIPLIER }>(Mode::Public);
        check_hash::<1, { 5 * WINDOW_SIZE_MULTIPLIER }>(Mode::Public);

        // Set the window size, and modulate the number of windows.
        check_hash::<1, WINDOW_SIZE_MULTIPLIER>(Mode::Public);
        check_hash::<2, WINDOW_SIZE_MULTIPLIER>(Mode::Public);
        check_hash::<3, WINDOW_SIZE_MULTIPLIER>(Mode::Public);
        check_hash::<4, WINDOW_SIZE_MULTIPLIER>(Mode::Public);
        check_hash::<5, WINDOW_SIZE_MULTIPLIER>(Mode::Public);
    }

    #[test]
    fn test_hash_private() {
        // Set the number of windows, and modulate the window size.
        check_hash::<1, WINDOW_SIZE_MULTIPLIER>(Mode::Private);
        check_hash::<1, { 2 * WINDOW_SIZE_MULTIPLIER }>(Mode::Private);
        check_hash::<1, { 3 * WINDOW_SIZE_MULTIPLIER }>(Mode::Private);
        check_hash::<1, { 4 * WINDOW_SIZE_MULTIPLIER }>(Mode::Private);
        check_hash::<1, { 5 * WINDOW_SIZE_MULTIPLIER }>(Mode::Private);

        // Set the window size, and modulate the number of windows.
        check_hash::<1, WINDOW_SIZE_MULTIPLIER>(Mode::Private);
        check_hash::<2, WINDOW_SIZE_MULTIPLIER>(Mode::Private);
        check_hash::<3, WINDOW_SIZE_MULTIPLIER>(Mode::Private);
        check_hash::<4, WINDOW_SIZE_MULTIPLIER>(Mode::Private);
        check_hash::<5, WINDOW_SIZE_MULTIPLIER>(Mode::Private);
    }
}
