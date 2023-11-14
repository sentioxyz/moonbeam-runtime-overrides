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
//! Autogenerated weights for `pallet_author_inherent`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-11-01, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-10-0-0-176`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("moonbase-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/moonbeam
// benchmark
// pallet
// --chain=moonbase-dev
// --steps=50
// --repeat=20
// --pallet=pallet_author_inherent
// --extrinsic=*
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/common/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_author_inherent`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_author_inherent::WeightInfo for WeightInfo<T> {
	/// Storage: ParachainSystem ValidationData (r:1 w:0)
	/// Proof Skipped: ParachainSystem ValidationData (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AuthorInherent HighestSlotSeen (r:1 w:1)
	/// Proof: AuthorInherent HighestSlotSeen (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: AuthorInherent Author (r:1 w:0)
	/// Proof: AuthorInherent Author (max_values: Some(1), max_size: Some(20), added: 515, mode: MaxEncodedLen)
	/// Storage: ParachainStaking SelectedCandidates (r:1 w:0)
	/// Proof Skipped: ParachainStaking SelectedCandidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AuthorFilter EligibleCount (r:1 w:0)
	/// Proof Skipped: AuthorFilter EligibleCount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Randomness PreviousLocalVrfOutput (r:1 w:0)
	/// Proof Skipped: Randomness PreviousLocalVrfOutput (max_values: Some(1), max_size: None, mode: Measured)
	fn kick_off_authorship_validation() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `371`
		//  Estimated: `1856`
		// Minimum execution time: 14_675_000 picoseconds.
		Weight::from_parts(15_236_000, 0)
			.saturating_add(Weight::from_parts(0, 1856))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}