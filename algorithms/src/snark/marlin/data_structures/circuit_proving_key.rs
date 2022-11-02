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

use crate::{
    polycommit::sonic_pc,
    snark::marlin::{ahp::indexer::*, CircuitVerifyingKey, MarlinMode},
};
use snarkvm_curves::PairingEngine;
use snarkvm_utilities::{
    io::{self, Read, Write},
    serialize::*,
    FromBytes, ToBytes,
};

use serde::Serialize;
use serde_json::Result;

/// Proving key for a specific circuit (i.e., R1CS matrices).
#[derive(Clone, Debug, Serialize)]
pub struct CircuitProvingKey<E: PairingEngine, MM: MarlinMode> {
    /// The circuit verifying key.
    pub circuit_verifying_key: CircuitVerifyingKey<E, MM>,
    /// The randomness for the circuit polynomial commitments.
    pub circuit_commitment_randomness: Vec<sonic_pc::Randomness<E>>,
    /// The circuit itself.
    pub circuit: Circuit<E::Fr, MM>,
    /// The committer key for this index, trimmed from the universal SRS.
    pub committer_key: sonic_pc::CommitterKey<E>,
}

impl<E: PairingEngine, MM: MarlinMode> ToBytes for CircuitProvingKey<E, MM> {
    fn write_le<W: Write>(&self, mut writer: W) -> io::Result<()> {
        CanonicalSerialize::serialize_compressed(&self.circuit_verifying_key, &mut writer)?;
        CanonicalSerialize::serialize_compressed(&self.circuit_commitment_randomness, &mut writer)?;
        CanonicalSerialize::serialize_compressed(&self.circuit, &mut writer)?;

        self.committer_key.write_le(&mut writer)
    }
}

macro_rules! j2f {
    ($l:expr, $comma:tt) => {
        let j = String::from("\"");
        j.push_str(l);
        j.push_str("\"");
        file.write_all(sprintf!("\"{}\": ", l).as_bytes()).expect("write failed");
        file.write_all(serde_json::to_string_pretty(&$l).unwrap().as_bytes()).expect("write failed");
        if $comma {
            file.write_all(",\n"as_bytes()).expect("write failed");
        }
    };
}

impl<E: PairingEngine, MM: MarlinMode> FromBytes for CircuitProvingKey<E, MM> {
    #[inline]
    fn read_le<R: Read>(mut reader: R) -> io::Result<Self> {
        let circuit_verifying_key: CircuitVerifyingKey<E, MM> =
            CanonicalDeserialize::deserialize_compressed(&mut reader)?;
        let circuit_commitment_randomness = CanonicalDeserialize::deserialize_compressed(&mut reader)?;
        let circuit = CanonicalDeserialize::deserialize_compressed(&mut reader)?;
        let committer_key = FromBytes::read_le(&mut reader)?;

        // let mut file = std::fs::File::create("k.json").expect("create failed");
        // file.write_all("{\n".as_bytes()).expect("write failed");

        // file.write_all("\"circuit_verifying_key\": ".as_bytes()).expect("write failed");
        // file.write_all(serde_json::to_string_pretty(&circuit_verifying_key).unwrap().as_bytes()).expect("write failed");
        // file.write_all(", \"circuit_commitment_randomness\": ".as_bytes()).expect("write failed");
        // file.write_all(serde_json::to_string_pretty(&circuit_commitment_randomness).unwrap().as_bytes())
        //     .expect("write failed");
        // file.write_all(", \"circuit\": ".as_bytes()).expect("write failed");
        // file.write_all(serde_json::to_string_pretty(&circuit).unwrap().as_bytes()).expect("write failed");
        // file.write_all(", \"committer_key\": ".as_bytes()).expect("write failed");
        // file.write_all(serde_json::to_string_pretty(&committer_key).unwrap().as_bytes()).expect("write failed");
        // file.write_all("}\n".as_bytes()).expect("write failed");

        Ok(Self { circuit_verifying_key, circuit_commitment_randomness, circuit, committer_key })
    }
}
