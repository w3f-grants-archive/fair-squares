// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_roles
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-06-30, STEPS: `100`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: ``, CPU: ``
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/fs-node
// benchmark
// pallet
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_roles
// --extrinsic
// *
// --steps
// 100
// --repeat
// 10
// --json-file=raw.json
// --output
// ./pallets/roles/src/weights.rs
// --template
// ./pallets/roles/src/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_roles.
pub trait WeightInfo {
	fn investor(b: u32, ) -> Weight;
	fn approval(b: u32, ) -> Weight;
	fn rejection(b: u32, ) -> Weight;
	fn set_admin(b: u32, ) -> Weight;
}

/// Weights for pallet_roles using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: RoleModule HouseSellerLog (r:1 w:0)
	// Storage: RoleModule InvestorLog (r:1 w:1)
	// Storage: RoleModule ServicerLog (r:1 w:0)
	// Storage: RoleModule TenantLog (r:1 w:0)
	// Storage: RoleModule AccountsRolesLog (r:0 w:1)
	fn investor(b: u32, ) -> Weight {
		(44_555_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((5_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Sudo Key (r:1 w:0)
	// Storage: RoleModule RoleApprovalList (r:1 w:1)
	// Storage: RoleModule HouseSellerLog (r:0 w:1)
	// Storage: RoleModule AccountsRolesLog (r:0 w:1)
	fn approval(_b: u32, ) -> Weight {
		(47_959_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Sudo Key (r:1 w:0)
	// Storage: RoleModule RoleApprovalList (r:1 w:1)
	fn rejection(b: u32, ) -> Weight {
		(40_261_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((5_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Sudo Key (r:1 w:1)
	fn set_admin(b: u32, ) -> Weight {
		(30_118_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((4_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: RoleModule HouseSellerLog (r:1 w:0)
	// Storage: RoleModule InvestorLog (r:1 w:1)
	// Storage: RoleModule ServicerLog (r:1 w:0)
	// Storage: RoleModule TenantLog (r:1 w:0)
	// Storage: RoleModule AccountsRolesLog (r:0 w:1)
	fn investor(b: u32, ) -> Weight {
		(44_555_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((5_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Sudo Key (r:1 w:0)
	// Storage: RoleModule RoleApprovalList (r:1 w:1)
	// Storage: RoleModule HouseSellerLog (r:0 w:1)
	// Storage: RoleModule AccountsRolesLog (r:0 w:1)
	fn approval(_b: u32, ) -> Weight {
		(47_959_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Sudo Key (r:1 w:0)
	// Storage: RoleModule RoleApprovalList (r:1 w:1)
	fn rejection(b: u32, ) -> Weight {
		(40_261_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((5_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Sudo Key (r:1 w:1)
	fn set_admin(b: u32, ) -> Weight {
		(30_118_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((4_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}