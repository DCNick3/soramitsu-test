use crate::{mock, mock::*};
use frame_support::assert_ok;
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

		let mut events = <frame_system::Pallet<Test>>::events();
		let event = events.remove(0).event;
		assert_eq!(
			event,
			mock::Event::from(crate::Event::Transfer { from: 1, to: 0, amount: U256::from(10) })
		);

		let event = events.remove(0).event;
		assert_eq!(
			event,
			mock::Event::from(crate::Event::Transfer { from: 1, to: 2, amount: U256::from(10) })
		);
	});
}
