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
//! Autogenerated weights for `pallet_proxy`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-02-22, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-10-0-0-176`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("moonbase-dev")`, DB CACHE: 1024

// Executed Command:
// ./target/production/moonbeam
// benchmark
// pallet
// --chain=moonbase-dev
// --steps=50
// --repeat=20
// --pallet=pallet_proxy
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

/// Weight functions for `pallet_proxy`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_proxy::WeightInfo for WeightInfo<T> {
	/// Storage: `Proxy::Proxies` (r:1 w:0)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
	/// Storage: `MaintenanceMode::MaintenanceMode` (r:1 w:0)
	/// Proof: `MaintenanceMode::MaintenanceMode` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `p` is `[1, 31]`.
	fn proxy(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `191 + p * (25 ±0)`
		//  Estimated: `4310 + p * (25 ±0)`
		// Minimum execution time: 15_939_000 picoseconds.
		Weight::from_parts(16_541_456, 0)
			.saturating_add(Weight::from_parts(0, 4310))
			// Standard Error: 1_068
			.saturating_add(Weight::from_parts(29_118, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(Weight::from_parts(0, 25).saturating_mul(p.into()))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:0)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
	/// Storage: `Proxy::Announcements` (r:1 w:1)
	/// Proof: `Proxy::Announcements` (`max_values`: None, `max_size`: Some(1837), added: 4312, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `MaintenanceMode::MaintenanceMode` (r:1 w:0)
	/// Proof: `MaintenanceMode::MaintenanceMode` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn proxy_announced(a: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `441 + a * (56 ±0) + p * (25 ±0)`
		//  Estimated: `5302 + a * (60 ±0) + p * (22 ±0)`
		// Minimum execution time: 39_909_000 picoseconds.
		Weight::from_parts(40_787_698, 0)
			.saturating_add(Weight::from_parts(0, 5302))
			// Standard Error: 2_365
			.saturating_add(Weight::from_parts(147_319, 0).saturating_mul(a.into()))
			// Standard Error: 2_443
			.saturating_add(Weight::from_parts(11_236, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 60).saturating_mul(a.into()))
			.saturating_add(Weight::from_parts(0, 22).saturating_mul(p.into()))
	}
	/// Storage: `Proxy::Announcements` (r:1 w:1)
	/// Proof: `Proxy::Announcements` (`max_values`: None, `max_size`: Some(1837), added: 4312, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn remove_announcement(a: u32, _p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `329 + a * (56 ±0)`
		//  Estimated: `5302`
		// Minimum execution time: 23_186_000 picoseconds.
		Weight::from_parts(24_682_780, 0)
			.saturating_add(Weight::from_parts(0, 5302))
			// Standard Error: 4_986
			.saturating_add(Weight::from_parts(168_805, 0).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Proxy::Announcements` (r:1 w:1)
	/// Proof: `Proxy::Announcements` (`max_values`: None, `max_size`: Some(1837), added: 4312, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn reject_announcement(a: u32, _p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `329 + a * (56 ±0)`
		//  Estimated: `5302`
		// Minimum execution time: 23_340_000 picoseconds.
		Weight::from_parts(24_867_304, 0)
			.saturating_add(Weight::from_parts(0, 5302))
			// Standard Error: 4_956
			.saturating_add(Weight::from_parts(160_959, 0).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:0)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
	/// Storage: `Proxy::Announcements` (r:1 w:1)
	/// Proof: `Proxy::Announcements` (`max_values`: None, `max_size`: Some(1837), added: 4312, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn announce(a: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `343 + a * (56 ±0) + p * (25 ±0)`
		//  Estimated: `5302`
		// Minimum execution time: 31_142_000 picoseconds.
		Weight::from_parts(33_453_242, 0)
			.saturating_add(Weight::from_parts(0, 5302))
			// Standard Error: 2_216
			.saturating_add(Weight::from_parts(171_609, 0).saturating_mul(a.into()))
			// Standard Error: 2_289
			.saturating_add(Weight::from_parts(21_281, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:1)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[1, 31]`.
	fn add_proxy(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `149 + p * (25 ±0)`
		//  Estimated: `4310`
		// Minimum execution time: 23_027_000 picoseconds.
		Weight::from_parts(23_771_068, 0)
			.saturating_add(Weight::from_parts(0, 4310))
			// Standard Error: 1_276
			.saturating_add(Weight::from_parts(42_599, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:1)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[1, 31]`.
	fn remove_proxy(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `149 + p * (25 ±0)`
		//  Estimated: `4310`
		// Minimum execution time: 23_134_000 picoseconds.
		Weight::from_parts(24_345_303, 0)
			.saturating_add(Weight::from_parts(0, 4310))
			// Standard Error: 1_768
			.saturating_add(Weight::from_parts(31_049, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:1)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[1, 31]`.
	fn remove_proxies(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `149 + p * (25 ±0)`
		//  Estimated: `4310`
		// Minimum execution time: 21_791_000 picoseconds.
		Weight::from_parts(22_943_249, 0)
			.saturating_add(Weight::from_parts(0, 4310))
			// Standard Error: 1_274
			.saturating_add(Weight::from_parts(30_215, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:1)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[1, 31]`.
	fn create_pure(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `161`
		//  Estimated: `4310`
		// Minimum execution time: 24_280_000 picoseconds.
		Weight::from_parts(25_255_463, 0)
			.saturating_add(Weight::from_parts(0, 4310))
			// Standard Error: 1_243
			.saturating_add(Weight::from_parts(11_365, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:1)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[0, 30]`.
	fn kill_pure(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `174 + p * (25 ±0)`
		//  Estimated: `4310`
		// Minimum execution time: 22_691_000 picoseconds.
		Weight::from_parts(23_654_957, 0)
			.saturating_add(Weight::from_parts(0, 4310))
			// Standard Error: 1_031
			.saturating_add(Weight::from_parts(28_090, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
