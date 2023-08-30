// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_collective_content`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-08-18, STEPS: `10`, REPEAT: `3`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `cob`, CPU: `<UNKNOWN>`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("collectives-polkadot-dev")`, DB CACHE: 1024

// Executed Command:
// ./target/debug/polkadot-parachain
// benchmark
// pallet
// --chain=collectives-polkadot-dev
// --steps=10
// --repeat=3
// --pallet=pallet_collective_content
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./parachains/runtimes/collectives/collectives-polkadot/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_collective_content`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective_content::WeightInfo for WeightInfo<T> {
	/// Storage: `AmbassadorContent::Charter` (r:0 w:1)
	/// Proof: `AmbassadorContent::Charter` (`max_values`: Some(1), `max_size`: Some(70), added: 565, mode: `MaxEncodedLen`)
	fn set_charter() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 99_000_000 picoseconds.
		Weight::from_parts(99_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `AmbassadorCollective::Members` (r:1 w:0)
	/// Proof: `AmbassadorCollective::Members` (`max_values`: None, `max_size`: Some(42), added: 2517, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorContent::AnnouncementsCount` (r:1 w:1)
	/// Proof: `AmbassadorContent::AnnouncementsCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorContent::NextAnnouncementExpireAt` (r:1 w:1)
	/// Proof: `AmbassadorContent::NextAnnouncementExpireAt` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorContent::Announcements` (r:0 w:1)
	/// Proof: `AmbassadorContent::Announcements` (`max_values`: None, `max_size`: Some(90), added: 2565, mode: `MaxEncodedLen`)
	fn announce() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `285`
		//  Estimated: `3507`
		// Minimum execution time: 273_000_000 picoseconds.
		Weight::from_parts(278_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3507))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `AmbassadorCollective::Members` (r:1 w:0)
	/// Proof: `AmbassadorCollective::Members` (`max_values`: None, `max_size`: Some(42), added: 2517, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorContent::Announcements` (r:1 w:1)
	/// Proof: `AmbassadorContent::Announcements` (`max_values`: None, `max_size`: Some(90), added: 2565, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorContent::AnnouncementsCount` (r:1 w:1)
	/// Proof: `AmbassadorContent::AnnouncementsCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn remove_announcement() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `450`
		//  Estimated: `3555`
		// Minimum execution time: 326_000_000 picoseconds.
		Weight::from_parts(338_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3555))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `AmbassadorContent::NextAnnouncementExpireAt` (r:1 w:1)
	/// Proof: `AmbassadorContent::NextAnnouncementExpireAt` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorContent::Announcements` (r:101 w:100)
	/// Proof: `AmbassadorContent::Announcements` (`max_values`: None, `max_size`: Some(90), added: 2565, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorContent::AnnouncementsCount` (r:0 w:1)
	/// Proof: `AmbassadorContent::AnnouncementsCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[0, 100]`.
	fn cleanup_announcements(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `167 + x * (96 ±0)`
		//  Estimated: `3555 + x * (2565 ±0)`
		// Minimum execution time: 37_000_000 picoseconds.
		Weight::from_parts(144_774_246, 0)
			.saturating_add(Weight::from_parts(0, 3555))
			// Standard Error: 329_536
			.saturating_add(Weight::from_parts(138_770_008, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(x.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(x.into())))
			.saturating_add(Weight::from_parts(0, 2565).saturating_mul(x.into()))
	}
}
