use crate::{mock, mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use sp_core::U256;

#[test]
fn test_transfer() {
	new_test_ext().execute_with(|| {
		assert_eq!(Erc20::balance_of(0), None);
		assert_eq!(Erc20::balance_of(1), Some(U256::from(110)));
		assert_eq!(Erc20::balance_of(2), Some(U256::from(90)));

		assert_ok!(Erc20::transfer(Origin::signed(1), 0, U256::from(10)));
		assert_ok!(Erc20::transfer(Origin::signed(1), 2, U256::from(10)));

		assert_eq!(Erc20::balance_of(0), Some(U256::from(10)));
		assert_eq!(Erc20::balance_of(1), Some(U256::from(90)));
		assert_eq!(Erc20::balance_of(2), Some(U256::from(100)));

		assert_eq!(
			<frame_system::Pallet<Test>>::events()
				.into_iter()
				.map(|ev| ev.event)
				.collect::<Vec<_>>(),
			vec![
				mock::Event::from(crate::Event::Transfer {
					from: 1,
					to: 0,
					amount: U256::from(10)
				}),
				mock::Event::from(crate::Event::Transfer {
					from: 1,
					to: 2,
					amount: U256::from(10)
				}),
			]
		);
	});
}

#[test]
fn test_transfer_no_funds() {
	new_test_ext().execute_with(|| {
		assert_eq!(Erc20::balance_of(0), None);
		assert_eq!(Erc20::balance_of(1), Some(U256::from(110)));
		assert_eq!(Erc20::balance_of(2), Some(U256::from(90)));

		assert_noop!(
			Erc20::transfer(Origin::signed(1), 0, U256::from(120)),
			Error::<Test>::InsufficientFunds
		);

		assert_eq!(Erc20::balance_of(0), None);
		assert_eq!(Erc20::balance_of(1), Some(U256::from(110)));
		assert_eq!(Erc20::balance_of(2), Some(U256::from(90)));

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
fn test_allowance() {
	new_test_ext().execute_with(|| {
		assert_eq!(Erc20::balance_of(0), None);
		assert_eq!(Erc20::balance_of(1), Some(U256::from(110)));
		assert_eq!(Erc20::balance_of(2), Some(U256::from(90)));

		assert_ok!(Erc20::approve(Origin::signed(1), 0, U256::from(20)));
		assert_ok!(Erc20::transfer_from(Origin::signed(0), 1, 0, U256::from(10)));
		assert_ok!(Erc20::transfer_from(Origin::signed(0), 1, 2, U256::from(10)));

		assert_eq!(Erc20::balance_of(0), Some(U256::from(10)));
		assert_eq!(Erc20::balance_of(1), Some(U256::from(90)));
		assert_eq!(Erc20::balance_of(2), Some(U256::from(100)));

		assert_eq!(
			<frame_system::Pallet<Test>>::events()
				.into_iter()
				.map(|ev| ev.event)
				.collect::<Vec<_>>(),
			vec![
				mock::Event::from(crate::Event::Approval {
					owner: 1,
					spender: 0,
					amount: U256::from(20)
				}),
				mock::Event::from(crate::Event::Transfer {
					from: 1,
					to: 0,
					amount: U256::from(10)
				}),
				mock::Event::from(crate::Event::Approval {
					owner: 1,
					spender: 0,
					amount: U256::from(10)
				}),
				mock::Event::from(crate::Event::Transfer {
					from: 1,
					to: 2,
					amount: U256::from(10)
				}),
				mock::Event::from(crate::Event::Approval {
					owner: 1,
					spender: 0,
					amount: U256::from(0)
				}),
			]
		);
	});
}

#[test]
fn test_transfer_no_allowance() {
	new_test_ext().execute_with(|| {
		assert_eq!(Erc20::balance_of(0), None);
		assert_eq!(Erc20::balance_of(1), Some(U256::from(110)));
		assert_eq!(Erc20::balance_of(2), Some(U256::from(90)));

		assert_ok!(Erc20::approve(Origin::signed(1), 0, U256::from(20)));
		assert_noop!(
			Erc20::transfer_from(Origin::signed(0), 1, 0, U256::from(30)),
			Error::<Test>::InsufficientAllowance
		);
		assert_noop!(
			Erc20::transfer_from(Origin::signed(0), 1, 2, U256::from(21)),
			Error::<Test>::InsufficientAllowance
		);

		assert_eq!(Erc20::balance_of(0), None);
		assert_eq!(Erc20::balance_of(1), Some(U256::from(110)));
		assert_eq!(Erc20::balance_of(2), Some(U256::from(90)));

		assert_eq!(
			<frame_system::Pallet<Test>>::events()
				.into_iter()
				.map(|ev| ev.event)
				.collect::<Vec<_>>(),
			vec![mock::Event::from(crate::Event::Approval {
				owner: 1,
				spender: 0,
				amount: U256::from(20)
			})]
		);
	});
}
