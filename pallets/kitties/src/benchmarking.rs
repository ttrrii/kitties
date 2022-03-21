//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Kitties;
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	create_kitty{
        let s in 0..100;
        let caller :T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller))

    set_price{
        let s in 0..100;
        let caller :T::AccountId = whitelisted_caller();
        let _=Kitties::<T>::create_kitty(RawOrigin::Signed(caller.clone()).into());
        let kitty_hash = Kitties::<T>::kitties_owned(caller.clone());
    }: _(RawOrigin::Signed(caller), kitty_hash[0],Some(s.into()))

    transfer{
        let s in 0..100;
        let caller :T::AccountId = whitelisted_caller();
        let to: T::AccountId = account("to", 2u32, 2u32);
        let _=Kitties::<T>::create_kitty(RawOrigin::Signed(caller.clone()).into());
        let kitty_hash = Kitties::<T>::kitties_owned(caller.clone());
    }: _(RawOrigin::Signed(caller),to,kitty_hash[0])

	impl_benchmark_test_suite!(Kitties, crate::mock::new_test_ext(), crate::mock::Test);
}