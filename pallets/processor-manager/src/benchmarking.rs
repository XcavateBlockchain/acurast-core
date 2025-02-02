//! Benchmarking setup for pallet-acurast-processor-manager

use crate::stub::{alice_account_id, generate_account};

use super::*;

use acurast_common::ListUpdateOperation;
use frame_benchmarking::{benchmarks, whitelist_account};
use frame_support::{
    sp_runtime::{
        traits::{IdentifyAccount, StaticLookup, Verify},
        AccountId32,
    },
    traits::{Get, IsType},
};
use frame_system::RawOrigin;
use sp_std::prelude::*;

pub trait BenchmarkHelper<T: Config> {
    fn dummy_proof() -> T::Proof;
    fn advertisement() -> T::Advertisement;
}

fn generate_pairing_update_add<T: Config>(index: u32) -> ProcessorPairingUpdateFor<T>
where
    T::AccountId: From<AccountId32>,
{
    let processor_account_id = generate_account(index).into();
    let timestamp = 1657363915002u128;
    // let message = [caller.encode(), timestamp.encode(), 1u128.encode()].concat();
    let signature = T::BenchmarkHelper::dummy_proof();
    ProcessorPairingUpdateFor::<T> {
        operation: ListUpdateOperation::Add,
        item: ProcessorPairingFor::<T>::new_with_proof(processor_account_id, timestamp, signature),
    }
}

benchmarks! {
    where_clause { where
        T: Config,
        T::AccountId: IsType<<<T::Proof as Verify>::Signer as IdentifyAccount>::AccountId>,
        T::AccountId: From<AccountId32>,
        <<T as frame_system::Config>::Lookup as StaticLookup>::Source: From<<<T::Proof as Verify>::Signer as IdentifyAccount>::AccountId>,
    }

    update_processor_pairings {
        let x in 1 .. T::MaxPairingUpdates::get();
        let mut updates = Vec::<ProcessorPairingUpdateFor<T>>::new();
        let caller: T::AccountId = alice_account_id().into();
        whitelist_account!(caller);
        for i in 0..x {
            updates.push(generate_pairing_update_add::<T>(i));
        }
    }: _(RawOrigin::Signed(caller), updates.try_into().unwrap())

    pair_with_manager {
        let manager_account = generate_account(0).into();
        let processor_account = generate_account(1).into();
        let timestamp = 1657363915002u128;
        // let message = [manager_account.encode(), timestamp.encode(), 1u128.encode()].concat();
        let signature = T::BenchmarkHelper::dummy_proof();
        let item = ProcessorPairingFor::<T>::new_with_proof(manager_account, timestamp, signature);
    }: _(RawOrigin::Signed(processor_account), item)

    recover_funds {
        let caller: T::AccountId = alice_account_id().into();
        whitelist_account!(caller);
        let update = generate_pairing_update_add::<T>(0);
        Pallet::<T>::update_processor_pairings(RawOrigin::Signed(caller.clone()).into(), vec![update.clone()].try_into().unwrap())?;
    }: _(RawOrigin::Signed(caller.clone()), update.item.account.into().into(), caller.clone().into().into())

    heartbeat {
        let caller: T::AccountId = alice_account_id().into();
        whitelist_account!(caller);
        let update = generate_pairing_update_add::<T>(0);
        Pallet::<T>::update_processor_pairings(RawOrigin::Signed(caller.clone()).into(), vec![update.clone()].try_into().unwrap())?;
    }: _(RawOrigin::Signed(caller))

    advertise_for {
        let caller: T::AccountId = alice_account_id().into();
        whitelist_account!(caller);
        let update = generate_pairing_update_add::<T>(0);
        Pallet::<T>::update_processor_pairings(RawOrigin::Signed(caller.clone()).into(), vec![update.clone()].try_into().unwrap())?;
        let ad = T::BenchmarkHelper::advertisement();
    }: _(RawOrigin::Signed(caller), update.item.account.into().into(), ad)

    impl_benchmark_test_suite!(Pallet, mock::ExtBuilder::default().build(), mock::Test);
}
