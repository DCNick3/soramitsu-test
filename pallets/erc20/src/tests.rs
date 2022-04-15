use crate::{mock, mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use sp_core::U256;

#[test]
fn test_transfer() {
	new_test_ext().execute_with(|| {
		assert_eq!(Erc20::balance_of(0), None);
		assert_eq!(Erc20::balance_of(1), Some(110.into()));
		assert_eq!(Erc20::balance_of(2), Some(90.into()));

		assert_ok!(Erc20::transfer(Origin::signed(1), 0, 10.into()));
		assert_ok!(Erc20::transfer(Origin::signed(1), 2, 10.into()));

		assert_eq!(Erc20::balance_of(0), Some(10.into()));
		assert_eq!(Erc20::balance_of(1), Some(90.into()));
		assert_eq!(Erc20::balance_of(2), Some(100.into()));

		assert_eq!(
			<frame_system::Pallet<Test>>::events()
				.into_iter()
				.map(|ev| ev.event)
				.collect::<Vec<_>>(),
			vec![
				mock::Event::from(crate::Event::Transfer { from: 1, to: 0, amount: 10.into() }),
				mock::Event::from(crate::Event::Transfer { from: 1, to: 2, amount: 10.into() }),
			]
		);
	});
}

#[test]
fn test_transfer_no_funds() {
	new_test_ext().execute_with(|| {
		assert_eq!(Erc20::balance_of(1), Some(110.into()));

		assert_noop!(
			Erc20::transfer(Origin::signed(1), 0, 120.into()),
			Error::<Test>::InsufficientFunds
		);

		assert_eq!(
			<frame_system::Pallet<Test>>::events()
				.into_iter()
				.map(|ev| ev.event)
				.collect::<Vec<_>>(),
			vec![]
		);
	});
}

#[test]
fn test_transfer_overflow() {
	new_test_ext().execute_with(|| {
		assert_eq!(Erc20::balance_of(2), Some(90.into()));
		assert_eq!(Erc20::balance_of(3), Some(U256::max_value()));

		assert_noop!(
			Erc20::transfer(Origin::signed(3), 1, U256::max_value() - U256::from(89)),
			Error::<Test>::Overflow
		);
	});
}

#[test]
fn test_allowance() {
	new_test_ext().execute_with(|| {
		assert_eq!(Erc20::balance_of(0), None);
		assert_eq!(Erc20::balance_of(1), Some(110.into()));
		assert_eq!(Erc20::balance_of(2), Some(90.into()));

		assert_ok!(Erc20::approve(Origin::signed(1), 0, 20.into()));
		assert_ok!(Erc20::transfer_from(Origin::signed(0), 1, 0, 10.into()));
		assert_ok!(Erc20::transfer_from(Origin::signed(0), 1, 2, 10.into()));

		assert_eq!(Erc20::balance_of(0), Some(10.into()));
		assert_eq!(Erc20::balance_of(1), Some(90.into()));
		assert_eq!(Erc20::balance_of(2), Some(100.into()));

		assert_eq!(
			<frame_system::Pallet<Test>>::events()
				.into_iter()
				.map(|ev| ev.event)
				.collect::<Vec<_>>(),
			vec![
				mock::Event::from(crate::Event::Approval {
					owner: 1,
					spender: 0,
					amount: 20.into()
				}),
				mock::Event::from(crate::Event::Transfer { from: 1, to: 0, amount: 10.into() }),
				mock::Event::from(crate::Event::Approval {
					owner: 1,
					spender: 0,
					amount: 10.into()
				}),
				mock::Event::from(crate::Event::Transfer { from: 1, to: 2, amount: 10.into() }),
				mock::Event::from(crate::Event::Approval {
					owner: 1,
					spender: 0,
					amount: 0.into()
				}),
			]
		);
	});
}

#[test]
fn test_transfer_no_allowance() {
	new_test_ext().execute_with(|| {
		assert_ok!(Erc20::approve(Origin::signed(1), 0, 20.into()));
		assert_noop!(
			Erc20::transfer_from(Origin::signed(0), 1, 0, 30.into()),
			Error::<Test>::InsufficientAllowance
		);
		assert_noop!(
			Erc20::transfer_from(Origin::signed(0), 1, 2, 21.into()),
			Error::<Test>::InsufficientAllowance
		);

		assert_eq!(
			<frame_system::Pallet<Test>>::events()
				.into_iter()
				.map(|ev| ev.event)
				.collect::<Vec<_>>(),
			vec![mock::Event::from(crate::Event::Approval {
				owner: 1,
				spender: 0,
				amount: 20.into()
			})]
		);
	});
}

#[test]
fn test_allowance_no_funds() {
	new_test_ext().execute_with(|| {
		assert_eq!(Erc20::balance_of(1), Some(110.into()));

		assert_ok!(Erc20::approve(Origin::signed(1), 0, 120.into()));
		assert_noop!(
			Erc20::transfer_from(Origin::signed(0), 1, 0, 120.into()),
			Error::<Test>::InsufficientFunds
		);
		assert_noop!(
			Erc20::transfer_from(Origin::signed(0), 1, 2, 111.into()),
			Error::<Test>::InsufficientFunds
		);

		assert_eq!(
			<frame_system::Pallet<Test>>::events()
				.into_iter()
				.map(|ev| ev.event)
				.collect::<Vec<_>>(),
			vec![mock::Event::from(crate::Event::Approval {
				owner: 1,
				spender: 0,
				amount: 120.into()
			})]
		);
	});
}

#[test]
fn test_allowance_overflow() {
	new_test_ext().execute_with(|| {
		assert_eq!(Erc20::balance_of(2), Some(90.into()));
		assert_eq!(Erc20::balance_of(3), Some(U256::max_value()));

		assert_ok!(Erc20::approve(Origin::signed(3), 0, U256::max_value()));
		assert_noop!(
			Erc20::transfer_from(Origin::signed(0), 3, 2, U256::max_value()),
			Error::<Test>::Overflow
		);

		assert_eq!(
			<frame_system::Pallet<Test>>::events()
				.into_iter()
				.map(|ev| ev.event)
				.collect::<Vec<_>>(),
			vec![mock::Event::from(crate::Event::Approval {
				owner: 3,
				spender: 0,
				amount: U256::max_value()
			})]
		);
	});
}

#[test]
fn test_unlimited_allowance() {
	new_test_ext().execute_with(|| {
		assert_eq!(Erc20::balance_of(1), Some(110.into()));

		assert_ok!(Erc20::approve(Origin::signed(1), 0, U256::max_value()));
		assert_ok!(Erc20::transfer_from(Origin::signed(0), 1, 0, 1.into()));
		assert_ok!(Erc20::transfer_from(Origin::signed(0), 1, 0, 2.into()));

		assert_eq!(Erc20::allowance(1, 0), Some(U256::max_value()));

		assert_eq!(
			<frame_system::Pallet<Test>>::events()
				.into_iter()
				.map(|ev| ev.event)
				.collect::<Vec<_>>(),
			vec![
				mock::Event::from(crate::Event::Approval {
					owner: 1,
					spender: 0,
					amount: U256::max_value()
				}),
				// notice: no approval updates here (it's unlimited)
				mock::Event::from(crate::Event::Transfer { from: 1, to: 0, amount: 1.into() }),
				mock::Event::from(crate::Event::Transfer { from: 1, to: 0, amount: 2.into() })
			]
		);
	});
}
