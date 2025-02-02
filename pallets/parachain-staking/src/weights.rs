// This file is part of Webb.
// Copyright (C) 2021 Webb Technologies Inc.
//
// Tangle is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Tangle is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Tangle.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for parachain_staking
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-10, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/release/moonbeam
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// parachain_staking
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --template=./benchmarking/frame-weight-template.hbs
// --json-file
// raw.json
// --output
// weights.rs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for parachain_staking.
pub trait WeightInfo {
	#[rustfmt::skip]
	fn set_staking_expectations() -> Weight;
	#[rustfmt::skip]
	fn set_inflation() -> Weight;
	#[rustfmt::skip]
	fn set_parachain_bond_account() -> Weight;
	#[rustfmt::skip]
	fn set_parachain_bond_reserve_percent() -> Weight;
	#[rustfmt::skip]
	fn set_total_selected() -> Weight;
	#[rustfmt::skip]
	fn set_collator_commission() -> Weight;
	#[rustfmt::skip]
	fn set_blocks_per_round() -> Weight;
	#[rustfmt::skip]
	fn join_candidates(x: u32, ) -> Weight;
	#[rustfmt::skip]
	fn schedule_leave_candidates(x: u32, ) -> Weight;
	#[rustfmt::skip]
	fn execute_leave_candidates(x: u32, ) -> Weight;
	#[rustfmt::skip]
	fn cancel_leave_candidates(x: u32, ) -> Weight;
	#[rustfmt::skip]
	fn go_offline() -> Weight;
	#[rustfmt::skip]
	fn go_online() -> Weight;
	#[rustfmt::skip]
	fn candidate_bond_more() -> Weight;
	#[rustfmt::skip]
	fn schedule_candidate_bond_less() -> Weight;
	#[rustfmt::skip]
	fn execute_candidate_bond_less() -> Weight;
	#[rustfmt::skip]
	fn cancel_candidate_bond_less() -> Weight;
	#[rustfmt::skip]
	fn delegate(x: u32, y: u32, ) -> Weight;
	#[rustfmt::skip]
	fn schedule_leave_delegators() -> Weight;
	#[rustfmt::skip]
	fn execute_leave_delegators(x: u32, ) -> Weight;
	#[rustfmt::skip]
	fn cancel_leave_delegators() -> Weight;
	#[rustfmt::skip]
	fn schedule_revoke_delegation() -> Weight;
	#[rustfmt::skip]
	fn delegator_bond_more() -> Weight;
	#[rustfmt::skip]
	fn schedule_delegator_bond_less() -> Weight;
	#[rustfmt::skip]
	fn execute_revoke_delegation() -> Weight;
	#[rustfmt::skip]
	fn execute_delegator_bond_less() -> Weight;
	#[rustfmt::skip]
	fn cancel_revoke_delegation() -> Weight;
	#[rustfmt::skip]
	fn cancel_delegator_bond_less() -> Weight;
	#[rustfmt::skip]
	fn round_transition_on_initialize(x: u32, y: u32, ) -> Weight;
	#[rustfmt::skip]
	fn pay_one_collator_reward(y: u32, ) -> Weight;
	#[rustfmt::skip]
	fn base_on_initialize() -> Weight;
	#[rustfmt::skip]
	fn set_auto_compound(x: u32, y: u32, ) -> Weight;
	#[rustfmt::skip]
	fn delegate_with_auto_compound(x: u32, y: u32, z: u32, ) -> Weight;
}

/// Weights for parachain_staking using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	#[rustfmt::skip]
	fn set_staking_expectations() -> Weight {
		Weight::from_ref_time(27_136_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	#[rustfmt::skip]
	fn set_inflation() -> Weight {
		Weight::from_ref_time(59_628_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: ParachainStaking ParachainBondInfo (r:1 w:1)
	#[rustfmt::skip]
	fn set_parachain_bond_account() -> Weight {
		Weight::from_ref_time(27_174_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: ParachainStaking ParachainBondInfo (r:1 w:1)
	#[rustfmt::skip]
	fn set_parachain_bond_reserve_percent() -> Weight {
		Weight::from_ref_time(26_397_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: ParachainStaking TotalSelected (r:1 w:1)
	#[rustfmt::skip]
	fn set_total_selected() -> Weight {
		Weight::from_ref_time(28_958_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: ParachainStaking CollatorCommission (r:1 w:1)
	#[rustfmt::skip]
	fn set_collator_commission() -> Weight {
		Weight::from_ref_time(24_841_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: ParachainStaking TotalSelected (r:1 w:0)
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	#[rustfmt::skip]
	fn set_blocks_per_round() -> Weight {
		Weight::from_ref_time(65_362_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:0)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:0 w:1)
	// Storage: ParachainStaking BottomDelegations (r:0 w:1)
	#[rustfmt::skip]
	fn join_candidates(x: u32, ) -> Weight {
		Weight::from_ref_time(90_562_000_u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(155_000_u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	#[rustfmt::skip]
	fn schedule_leave_candidates(x: u32, ) -> Weight {
		Weight::from_ref_time(73_297_000_u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(132_000_u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: Balances Locks (r:2 w:2)
	// Storage: System Account (r:2 w:2)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking AutoCompoundingDelegations (r:1 w:1)
	// Storage: ParachainStaking BottomDelegations (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	#[rustfmt::skip]
	fn execute_leave_candidates(x: u32, ) -> Weight {
		Weight::from_ref_time(0_u64)
			// Standard Error: 87_000
			.saturating_add(Weight::from_ref_time(31_860_000_u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().reads(3_u64.saturating_mul(x as u64)))
			.saturating_add(T::DbWeight::get().writes(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64.saturating_mul(x as u64)))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	#[rustfmt::skip]
	fn cancel_leave_candidates(x: u32, ) -> Weight {
		Weight::from_ref_time(69_026_000_u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(141_000_u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	#[rustfmt::skip]
	fn go_offline() -> Weight {
		Weight::from_ref_time(40_151_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	#[rustfmt::skip]
	fn go_online() -> Weight {
		Weight::from_ref_time(39_580_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	#[rustfmt::skip]
	fn candidate_bond_more() -> Weight {
		Weight::from_ref_time(66_177_000_u64)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	#[rustfmt::skip]
	fn schedule_candidate_bond_less() -> Weight {
		Weight::from_ref_time(36_834_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	#[rustfmt::skip]
	fn execute_candidate_bond_less() -> Weight {
		Weight::from_ref_time(73_496_000_u64)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	#[rustfmt::skip]
	fn cancel_candidate_bond_less() -> Weight {
		Weight::from_ref_time(33_631_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	#[rustfmt::skip]
	fn delegate(x: u32, y: u32, ) -> Weight {
		Weight::from_ref_time(134_489_000_u64)
			// Standard Error: 21_000
			.saturating_add(Weight::from_ref_time(169_000_u64).saturating_mul(x as u64))
			// Standard Error: 6_000
			.saturating_add(Weight::from_ref_time(225_000_u64).saturating_mul(y as u64))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	#[rustfmt::skip]
	fn schedule_leave_delegators() -> Weight {
		Weight::from_ref_time(41_489_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking AutoCompoundingDelegations (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	#[rustfmt::skip]
	fn execute_leave_delegators(x: u32, ) -> Weight {
		Weight::from_ref_time(18_201_000_u64)
			// Standard Error: 22_000
			.saturating_add(Weight::from_ref_time(27_748_000_u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads(4_u64.saturating_mul(x as u64)))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64.saturating_mul(x as u64)))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	#[rustfmt::skip]
	fn cancel_leave_delegators() -> Weight {
		Weight::from_ref_time(42_390_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	#[rustfmt::skip]
	fn schedule_revoke_delegation() -> Weight {
		Weight::from_ref_time(40_930_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:0)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	#[rustfmt::skip]
	fn delegator_bond_more() -> Weight {
		Weight::from_ref_time(86_183_000_u64)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	#[rustfmt::skip]
	fn schedule_delegator_bond_less() -> Weight {
		Weight::from_ref_time(40_887_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking AutoCompoundingDelegations (r:1 w:0)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	#[rustfmt::skip]
	fn execute_revoke_delegation() -> Weight {
		Weight::from_ref_time(107_376_000_u64)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	#[rustfmt::skip]
	fn execute_delegator_bond_less() -> Weight {
		Weight::from_ref_time(93_139_000_u64)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	#[rustfmt::skip]
	fn cancel_revoke_delegation() -> Weight {
		Weight::from_ref_time(39_815_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	#[rustfmt::skip]
	fn cancel_delegator_bond_less() -> Weight {
		Weight::from_ref_time(46_787_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: MoonbeamOrbiters ForceRotation (r:1 w:0)
	// Storage: ParachainStaking Points (r:1 w:0)
	// Storage: ParachainStaking Staked (r:1 w:2)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	// Storage: ParachainStaking ParachainBondInfo (r:1 w:0)
	// Storage: System Account (r:302 w:301)
	// Storage: ParachainStaking CollatorCommission (r:1 w:0)
	// Storage: ParachainStaking CandidatePool (r:1 w:0)
	// Storage: ParachainStaking TotalSelected (r:1 w:0)
	// Storage: ParachainStaking CandidateInfo (r:9 w:0)
	// Storage: ParachainStaking DelegationScheduledRequests (r:9 w:0)
	// Storage: ParachainStaking TopDelegations (r:9 w:0)
	// Storage: ParachainStaking AutoCompoundingDelegations (r:9 w:0)
	// Storage: ParachainStaking Total (r:1 w:0)
	// Storage: ParachainStaking AwardedPts (r:2 w:1)
	// Storage: ParachainStaking AtStake (r:1 w:10)
	// Storage: MoonbeamOrbiters OrbiterPerRound (r:1 w:0)
	// Storage: MoonbeamOrbiters CurrentRound (r:0 w:1)
	// Storage: ParachainStaking SelectedCandidates (r:0 w:1)
	// Storage: ParachainStaking DelayedPayouts (r:0 w:1)
	#[rustfmt::skip]
	fn round_transition_on_initialize(x: u32, y: u32, ) -> Weight {
		Weight::from_ref_time(363_268_000_u64)
			// Standard Error: 1_140_000
			.saturating_add(Weight::from_ref_time(43_560_000_u64).saturating_mul(x as u64))
			// Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(139_000_u64).saturating_mul(y as u64))
			.saturating_add(T::DbWeight::get().reads(180_u64))
			.saturating_add(T::DbWeight::get().reads(4_u64.saturating_mul(x as u64)))
			.saturating_add(T::DbWeight::get().writes(171_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64.saturating_mul(x as u64)))
	}
	// Storage: ParachainStaking DelayedPayouts (r:1 w:0)
	// Storage: ParachainStaking Points (r:1 w:0)
	// Storage: ParachainStaking AwardedPts (r:2 w:1)
	// Storage: ParachainStaking AtStake (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: MoonbeamOrbiters OrbiterPerRound (r:1 w:0)
	#[rustfmt::skip]
	fn pay_one_collator_reward(y: u32, ) -> Weight {
		Weight::from_ref_time(61_374_000_u64)
			// Standard Error: 5_000
			.saturating_add(Weight::from_ref_time(15_651_000_u64).saturating_mul(y as u64))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().reads(1_u64.saturating_mul(y as u64)))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64.saturating_mul(y as u64)))
	}
	#[rustfmt::skip]
	fn base_on_initialize() -> Weight {
		Weight::from_ref_time(11_002_000_u64)
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:0)
	// Storage: ParachainStaking AutoCompoundingDelegations (r:1 w:1)
	#[rustfmt::skip]
	fn set_auto_compound(x: u32, y: u32, ) -> Weight {
		Weight::from_ref_time(61_986_000_u64)
			// Standard Error: 4_000
			.saturating_add(Weight::from_ref_time(244_000_u64).saturating_mul(x as u64))
			// Standard Error: 14_000
			.saturating_add(Weight::from_ref_time(216_000_u64).saturating_mul(y as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking AutoCompoundingDelegations (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking BottomDelegations (r:1 w:1)
	#[rustfmt::skip]
	fn delegate_with_auto_compound(x: u32, y: u32, _z: u32, ) -> Weight {
		Weight::from_ref_time(168_431_000_u64)
			// Standard Error: 5_000
			.saturating_add(Weight::from_ref_time(73_000_u64).saturating_mul(x as u64))
			// Standard Error: 5_000
			.saturating_add(Weight::from_ref_time(71_000_u64).saturating_mul(y as u64))
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	#[rustfmt::skip]
	fn set_staking_expectations() -> Weight {
		Weight::from_ref_time(27_136_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	#[rustfmt::skip]
	fn set_inflation() -> Weight {
		Weight::from_ref_time(59_628_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: ParachainStaking ParachainBondInfo (r:1 w:1)
	#[rustfmt::skip]
	fn set_parachain_bond_account() -> Weight {
		Weight::from_ref_time(27_174_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: ParachainStaking ParachainBondInfo (r:1 w:1)
	#[rustfmt::skip]
	fn set_parachain_bond_reserve_percent() -> Weight {
		Weight::from_ref_time(26_397_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: ParachainStaking TotalSelected (r:1 w:1)
	#[rustfmt::skip]
	fn set_total_selected() -> Weight {
		Weight::from_ref_time(28_958_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: ParachainStaking CollatorCommission (r:1 w:1)
	#[rustfmt::skip]
	fn set_collator_commission() -> Weight {
		Weight::from_ref_time(24_841_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: ParachainStaking TotalSelected (r:1 w:0)
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	#[rustfmt::skip]
	fn set_blocks_per_round() -> Weight {
		Weight::from_ref_time(65_362_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:0)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:0 w:1)
	// Storage: ParachainStaking BottomDelegations (r:0 w:1)
	#[rustfmt::skip]
	fn join_candidates(x: u32, ) -> Weight {
		Weight::from_ref_time(90_562_000_u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(155_000_u64).saturating_mul(x as u64))
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(7_u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	#[rustfmt::skip]
	fn schedule_leave_candidates(x: u32, ) -> Weight {
		Weight::from_ref_time(73_297_000_u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(132_000_u64).saturating_mul(x as u64))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: Balances Locks (r:2 w:2)
	// Storage: System Account (r:2 w:2)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking AutoCompoundingDelegations (r:1 w:1)
	// Storage: ParachainStaking BottomDelegations (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	#[rustfmt::skip]
	fn execute_leave_candidates(x: u32, ) -> Weight {
		Weight::from_ref_time(0_u64)
			// Standard Error: 87_000
			.saturating_add(Weight::from_ref_time(31_860_000_u64).saturating_mul(x as u64))
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().reads(3_u64.saturating_mul(x as u64)))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64.saturating_mul(x as u64)))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	#[rustfmt::skip]
	fn cancel_leave_candidates(x: u32, ) -> Weight {
		Weight::from_ref_time(69_026_000_u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(141_000_u64).saturating_mul(x as u64))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	#[rustfmt::skip]
	fn go_offline() -> Weight {
		Weight::from_ref_time(40_151_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	#[rustfmt::skip]
	fn go_online() -> Weight {
		Weight::from_ref_time(39_580_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	#[rustfmt::skip]
	fn candidate_bond_more() -> Weight {
		Weight::from_ref_time(66_177_000_u64)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	#[rustfmt::skip]
	fn schedule_candidate_bond_less() -> Weight {
		Weight::from_ref_time(36_834_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	#[rustfmt::skip]
	fn execute_candidate_bond_less() -> Weight {
		Weight::from_ref_time(73_496_000_u64)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	#[rustfmt::skip]
	fn cancel_candidate_bond_less() -> Weight {
		Weight::from_ref_time(33_631_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	#[rustfmt::skip]
	fn delegate(x: u32, y: u32, ) -> Weight {
		Weight::from_ref_time(134_489_000_u64)
			// Standard Error: 21_000
			.saturating_add(Weight::from_ref_time(169_000_u64).saturating_mul(x as u64))
			// Standard Error: 6_000
			.saturating_add(Weight::from_ref_time(225_000_u64).saturating_mul(y as u64))
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(7_u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	#[rustfmt::skip]
	fn schedule_leave_delegators() -> Weight {
		Weight::from_ref_time(41_489_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking AutoCompoundingDelegations (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	#[rustfmt::skip]
	fn execute_leave_delegators(x: u32, ) -> Weight {
		Weight::from_ref_time(18_201_000_u64)
			// Standard Error: 22_000
			.saturating_add(Weight::from_ref_time(27_748_000_u64).saturating_mul(x as u64))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().reads(4_u64.saturating_mul(x as u64)))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64.saturating_mul(x as u64)))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	#[rustfmt::skip]
	fn cancel_leave_delegators() -> Weight {
		Weight::from_ref_time(42_390_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	#[rustfmt::skip]
	fn schedule_revoke_delegation() -> Weight {
		Weight::from_ref_time(40_930_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:0)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	#[rustfmt::skip]
	fn delegator_bond_more() -> Weight {
		Weight::from_ref_time(86_183_000_u64)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(7_u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	#[rustfmt::skip]
	fn schedule_delegator_bond_less() -> Weight {
		Weight::from_ref_time(40_887_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking AutoCompoundingDelegations (r:1 w:0)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	#[rustfmt::skip]
	fn execute_revoke_delegation() -> Weight {
		Weight::from_ref_time(107_376_000_u64)
			.saturating_add(RocksDbWeight::get().reads(9_u64))
			.saturating_add(RocksDbWeight::get().writes(8_u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	#[rustfmt::skip]
	fn execute_delegator_bond_less() -> Weight {
		Weight::from_ref_time(93_139_000_u64)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(8_u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	#[rustfmt::skip]
	fn cancel_revoke_delegation() -> Weight {
		Weight::from_ref_time(39_815_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	#[rustfmt::skip]
	fn cancel_delegator_bond_less() -> Weight {
		Weight::from_ref_time(46_787_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	// Storage: MoonbeamOrbiters ForceRotation (r:1 w:0)
	// Storage: ParachainStaking Points (r:1 w:0)
	// Storage: ParachainStaking Staked (r:1 w:2)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	// Storage: ParachainStaking ParachainBondInfo (r:1 w:0)
	// Storage: System Account (r:302 w:301)
	// Storage: ParachainStaking CollatorCommission (r:1 w:0)
	// Storage: ParachainStaking CandidatePool (r:1 w:0)
	// Storage: ParachainStaking TotalSelected (r:1 w:0)
	// Storage: ParachainStaking CandidateInfo (r:9 w:0)
	// Storage: ParachainStaking DelegationScheduledRequests (r:9 w:0)
	// Storage: ParachainStaking TopDelegations (r:9 w:0)
	// Storage: ParachainStaking AutoCompoundingDelegations (r:9 w:0)
	// Storage: ParachainStaking Total (r:1 w:0)
	// Storage: ParachainStaking AwardedPts (r:2 w:1)
	// Storage: ParachainStaking AtStake (r:1 w:10)
	// Storage: MoonbeamOrbiters OrbiterPerRound (r:1 w:0)
	// Storage: MoonbeamOrbiters CurrentRound (r:0 w:1)
	// Storage: ParachainStaking SelectedCandidates (r:0 w:1)
	// Storage: ParachainStaking DelayedPayouts (r:0 w:1)
	#[rustfmt::skip]
	fn round_transition_on_initialize(x: u32, y: u32, ) -> Weight {
		Weight::from_ref_time(363_268_000_u64)
			// Standard Error: 1_140_000
			.saturating_add(Weight::from_ref_time(43_560_000_u64).saturating_mul(x as u64))
			// Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(139_000_u64).saturating_mul(y as u64))
			.saturating_add(RocksDbWeight::get().reads(180_u64))
			.saturating_add(RocksDbWeight::get().reads(4_u64.saturating_mul(x as u64)))
			.saturating_add(RocksDbWeight::get().writes(171_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64.saturating_mul(x as u64)))
	}
	// Storage: ParachainStaking DelayedPayouts (r:1 w:0)
	// Storage: ParachainStaking Points (r:1 w:0)
	// Storage: ParachainStaking AwardedPts (r:2 w:1)
	// Storage: ParachainStaking AtStake (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: MoonbeamOrbiters OrbiterPerRound (r:1 w:0)
	#[rustfmt::skip]
	fn pay_one_collator_reward(y: u32, ) -> Weight {
		Weight::from_ref_time(61_374_000_u64)
			// Standard Error: 5_000
			.saturating_add(Weight::from_ref_time(15_651_000_u64).saturating_mul(y as u64))
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().reads(1_u64.saturating_mul(y as u64)))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64.saturating_mul(y as u64)))
	}
	#[rustfmt::skip]
	fn base_on_initialize() -> Weight {
		Weight::from_ref_time(11_002_000_u64)
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:0)
	// Storage: ParachainStaking AutoCompoundingDelegations (r:1 w:1)
	#[rustfmt::skip]
	fn set_auto_compound(x: u32, y: u32, ) -> Weight {
		Weight::from_ref_time(61_986_000_u64)
			// Standard Error: 4_000
			.saturating_add(Weight::from_ref_time(244_000_u64).saturating_mul(x as u64))
			// Standard Error: 14_000
			.saturating_add(Weight::from_ref_time(216_000_u64).saturating_mul(y as u64))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking AutoCompoundingDelegations (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking BottomDelegations (r:1 w:1)
	#[rustfmt::skip]
	fn delegate_with_auto_compound(x: u32, y: u32, _z: u32, ) -> Weight {
		Weight::from_ref_time(168_431_000_u64)
			// Standard Error: 5_000
			.saturating_add(Weight::from_ref_time(73_000_u64).saturating_mul(x as u64))
			// Standard Error: 5_000
			.saturating_add(Weight::from_ref_time(71_000_u64).saturating_mul(y as u64))
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(8_u64))
	}
}
