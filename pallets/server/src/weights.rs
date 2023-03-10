//! Autogenerated weights for pallet_server
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-22, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `Irmans-MacBook-Pro.local`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/myriad
// benchmark
// pallet
// --chain=dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet-server
// --extrinsic=*
// --steps=50
// --repeat=20
// --heap-pages=4096
// --template=./.maintain/pallet-weight-template.hbs
// --output=./pallets/server/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_server.
pub trait WeightInfo {
	fn register(s: u32, ) -> Weight;
	fn update_server(s: u32, ) -> Weight;
	fn unregister() -> Weight;
	fn cancel_unregister() -> Weight;
	fn on_initialize_server() -> Weight;
}

/// Weights for pallet_server using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Server ServerByApiUrl (r:1 w:1)
	// Storage: Server ServerCount (r:1 w:1)
	// Storage: Server ServerIndex (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Server ServerById (r:0 w:1)
	// Storage: Server ServerByOwner (r:0 w:1)
	/// The range of component `s` is `[1, 100]`.
	fn register(s: u32, ) -> Weight {
		Weight::from_ref_time(157_000_000_u64)
			// Standard Error: 2_258
			.saturating_add(Weight::from_ref_time(31_163_u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	// Storage: Server ServerById (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Server ServerByOwner (r:0 w:1)
	// Storage: Server ServerByApiUrl (r:2 w:2)
	/// The range of component `s` is `[0, 2]`.
	fn update_server(s: u32, ) -> Weight {
		Weight::from_ref_time(87_000_000_u64)
			// Standard Error: 1_129_362
			.saturating_add(Weight::from_ref_time(614_285_u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads(1_u64.saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64.saturating_mul(s as u64)))
	}
	// Storage: Server ServerById (r:1 w:1)
	// Storage: Server Tasks (r:1 w:1)
	// Storage: Server ServerByOwner (r:0 w:1)
	fn unregister() -> Weight {
		Weight::from_ref_time(77_000_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: Server ServerById (r:1 w:1)
	// Storage: Server Tasks (r:1 w:1)
	// Storage: Server ServerByOwner (r:0 w:1)
	fn cancel_unregister() -> Weight {
		Weight::from_ref_time(77_000_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: Server Tasks (r:1 w:1)
	// Storage: Server ServerById (r:1 w:1)
	// Storage: Server ServerCount (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Server ServerByOwner (r:0 w:1)
	// Storage: Server ServerByApiUrl (r:0 w:1)
	fn on_initialize_server() -> Weight {
		Weight::from_ref_time(156_000_000_u64)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Server ServerByApiUrl (r:1 w:1)
	// Storage: Server ServerCount (r:1 w:1)
	// Storage: Server ServerIndex (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Server ServerById (r:0 w:1)
	// Storage: Server ServerByOwner (r:0 w:1)
	/// The range of component `s` is `[1, 100]`.
	fn register(s: u32, ) -> Weight {
		Weight::from_ref_time(157_000_000_u64)
			// Standard Error: 2_258
			.saturating_add(Weight::from_ref_time(31_163_u64).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	// Storage: Server ServerById (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Server ServerByOwner (r:0 w:1)
	// Storage: Server ServerByApiUrl (r:2 w:2)
	/// The range of component `s` is `[0, 2]`.
	fn update_server(s: u32, ) -> Weight {
		Weight::from_ref_time(87_000_000_u64)
			// Standard Error: 1_129_362
			.saturating_add(Weight::from_ref_time(614_285_u64).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().reads(1_u64.saturating_mul(s as u64)))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64.saturating_mul(s as u64)))
	}
	// Storage: Server ServerById (r:1 w:1)
	// Storage: Server Tasks (r:1 w:1)
	// Storage: Server ServerByOwner (r:0 w:1)
	fn unregister() -> Weight {
		Weight::from_ref_time(77_000_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	// Storage: Server ServerById (r:1 w:1)
	// Storage: Server Tasks (r:1 w:1)
	// Storage: Server ServerByOwner (r:0 w:1)
	fn cancel_unregister() -> Weight {
		Weight::from_ref_time(77_000_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	// Storage: Server Tasks (r:1 w:1)
	// Storage: Server ServerById (r:1 w:1)
	// Storage: Server ServerCount (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Server ServerByOwner (r:0 w:1)
	// Storage: Server ServerByApiUrl (r:0 w:1)
	fn on_initialize_server() -> Weight {
		Weight::from_ref_time(156_000_000_u64)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
}