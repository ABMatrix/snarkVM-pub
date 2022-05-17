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

use crate::Integer;

use snarkvm_circuits_environment::{CircuitType, Eject, Environment, IntegerType, Mode};
use snarkvm_circuits_types_boolean::Boolean;

use std::marker::PhantomData;

// Wrapper struct around a vector of `CircuitType<Boolean<E>>` which represent an integer.
pub struct IntegerCircuitType<E: Environment, I: IntegerType> {
    bits_le: Vec<CircuitType<Boolean<E>>>,
    phantom: PhantomData<I>,
}

impl<E: Environment, I: IntegerType> IntegerCircuitType<E, I> {
    /// Initializes a new `IntegerCircuitType`.
    pub fn new(bits_le: Vec<CircuitType<Boolean<E>>>) -> Self {
        assert_eq!(
            bits_le.len(),
            I::BITS as usize,
            "Number of input bits does not match the expected number of bits required by the integer type"
        );
        IntegerCircuitType { bits_le, phantom: PhantomData }
    }

    pub fn mode(&self) -> Mode {
        self.bits_le.eject_mode()
    }

    pub fn circuit(self) -> Integer<E, I> {
        match self.mode() {
            Mode::Constant => {
                Integer { bits_le: self.bits_le.iter().map(|bit| bit.circuit()).collect(), phantom: PhantomData }
            }
            Mode::Public => panic!("Cannot retrieve the circuit when the mode is Public"),
            Mode::Private => panic!("Cannot retrieve the circuit when the mode is Private"),
        }
    }

    /// Returns the underlying vector of `CircuitType<Boolean<E>>`.
    pub fn bits_le(self) -> Vec<CircuitType<Boolean<E>>> {
        self.bits_le
    }
}

impl<E: Environment, I: IntegerType> From<Vec<CircuitType<Boolean<E>>>> for IntegerCircuitType<E, I> {
    fn from(bits_le: Vec<CircuitType<Boolean<E>>>) -> Self {
        assert_eq!(
            bits_le.len(),
            I::BITS as usize,
            "Number of input bits does not match the expected number of bits required by the integer type"
        );
        IntegerCircuitType { bits_le, phantom: PhantomData }
    }
}

impl<E: Environment, I: IntegerType> From<Integer<E, I>> for IntegerCircuitType<E, I> {
    fn from(integer: Integer<E, I>) -> Self {
        let mut bits_le = Vec::with_capacity(I::BITS as usize);
        for bit in integer.bits_le {
            bits_le.push(CircuitType::from(bit))
        }
        IntegerCircuitType { bits_le, phantom: PhantomData }
    }
}
