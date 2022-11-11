// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Bridge Message Transfer Pallet
//!
//! A utility which could help move messages through bridges, e.g. move assets between different global consensus...

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

/// The log target of this pallet.
pub const LOG_TARGET: &str = "runtime::bridge-message-transfer";

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use xcm::{VersionedMultiAssets, VersionedMultiLocation};

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::error]
	pub enum Error<T> {
		/// TODO: add custom errors
		InitTransferError,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		// TODO: add here xcm_hash?
		/// Transfer was successfully entered to the system (does not mean already delivered)
		TransferFired,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// Root Calls

		// TODO: correct weigth
		#[pallet::weight((T::DbWeight::get().reads_writes(1, 1), DispatchClass::Operational))]
		pub fn transfer_asset_via_bridge(
			origin: OriginFor<T>,
			assets: VersionedMultiAssets,
			destination: VersionedMultiLocation,
		) -> DispatchResult {
			// TODO: do some checks

			// Deposit assets into `AccountId` that corresponds to the bridge
			// hub. In this way, Statemine acts as a reserve location to the
			// bridge, such that it need not trust any consensus system from
			// `./Parent/Parent/...`. (It may trust Polkadot, but would
			// Polkadot trust Kusama with its DOT?)

			// TODO: xcm - withdraw and fire ReserveAssetDeposited to the other side

			// TODO: send message through bridge
			// Construct and send `Xcm(vec![Instruction])` to
			// `./Parent/BridgeHubParaId`.

			Self::deposit_event(Event::TransferFired);
			Ok(())
		}
	}
}
