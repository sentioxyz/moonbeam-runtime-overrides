// Copyright 2019-2022 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_evm`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-04-10, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-10-0-0-176`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: Compiled, CHAIN: Some("moonbase-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/moonbeam
// benchmark
// pallet
// --chain=moonbase-dev
// --steps=50
// --repeat=20
// --pallet=pallet_evm
// --extrinsic=*
// --wasm-execution=compiled
// --header=./file_header.txt
// --template=./benchmarking/frame-weight-template.hbs
// --output=./runtime/common/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for `pallet_evm`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_evm::WeightInfo for WeightInfo<T> {
	/// Storage: `TransactionPayment::NextFeeMultiplier` (r:1 w:0)
	/// Proof: `TransactionPayment::NextFeeMultiplier` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `EthereumChainId::ChainId` (r:1 w:0)
	/// Proof: `EthereumChainId::ChainId` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `EVM::AccountCodes` (r:2 w:0)
	/// Proof: `EVM::AccountCodes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Digest` (r:1 w:0)
	/// Proof: `System::Digest` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `EVM::AccountStorages` (r:1 w:0)
	/// Proof: `EVM::AccountStorages` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `x` is `[1, 10000000]`.
	fn runner_execute(_x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1574`
		//  Estimated: `7514`
		// Minimum execution time: 25_094_983_000 picoseconds.
		Weight::from_parts(25_279_087_568, 7514)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	fn withdraw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_878_000 picoseconds.
		Weight::from_parts(2_005_000, 0)
	}
}