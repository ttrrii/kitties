use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn test_create_kitty() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		System::set_block_number(1);
		assert_ok!(KittyModule::create_kitty(Origin::signed(1)));
	});
}

#[test]
fn test_transfer_kitties(){
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let _=KittyModule::create_kitty(Origin::signed(1));
		let kitty_hash = KittyModule::kitties_owned(1)[0];

		assert_ok!(KittyModule::transfer(Origin::signed(1),2,kitty_hash));
	});
}

#[test]
fn test_buy_kitty(){
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let _=KittyModule::create_kitty(Origin::signed(1));
		let kitty_hash = KittyModule::kitties_owned(1)[0];
		let _ = KittyModule::set_price(Origin::signed(1), kitty_hash, Some(1_000_000u128));
		let _ = Balances::set_balance(Origin::root(), 2, 1_000_000_000_000, 1_000_000);
		assert_eq!(Balances::free_balance(2),1_000_000_000_000);
		assert_ok!(KittyModule::buy_kitty(Origin::signed(2),kitty_hash,1_1000_000u128));
	});
}