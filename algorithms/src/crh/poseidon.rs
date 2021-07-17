// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use crate::{
    crypto_hash::{PoseidonCryptoHash, PoseidonDefaultParametersField, PoseidonParameters},
    traits::{CryptoHash, CRH},
    CRHError,
};

use snarkvm_fields::{PrimeField, ToConstraintField};

impl<F: PrimeField + PoseidonDefaultParametersField, const RATE: usize, const OPTIMIZED_FOR_WEIGHTS: bool> CRH
    for PoseidonCryptoHash<F, RATE, OPTIMIZED_FOR_WEIGHTS>
{
    type Output = F;
    type Parameters = PoseidonParameters<F>;

    // TODO (raychu86): Specify this value correctly. Currently an arbitrary value
    const INPUT_SIZE_BITS: usize = 20 * 48;

    fn setup(_message: &str) -> Self {
        Self {
            parameters: F::get_default_poseidon_parameters(RATE, OPTIMIZED_FOR_WEIGHTS).unwrap(),
        }
    }

    fn hash(&self, input: &[u8]) -> Result<Self::Output, CRHError> {
        Ok(Self::evaluate(&input.to_field_elements().unwrap())?)
    }

    fn parameters(&self) -> &Self::Parameters {
        &self.parameters
    }
}

impl<F: PrimeField + PoseidonDefaultParametersField, const RATE: usize, const OPTIMIZED_FOR_WEIGHTS: bool>
    From<PoseidonParameters<F>> for PoseidonCryptoHash<F, RATE, OPTIMIZED_FOR_WEIGHTS>
{
    fn from(parameters: PoseidonParameters<F>) -> Self {
        Self { parameters }
    }
}