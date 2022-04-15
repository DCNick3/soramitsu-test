#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_core::U256;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Name of the coin (name() public function of ERC20)
		#[pallet::constant]
		type Name: Get<&'static str>;

		/// A symbol (shorter variant of a name) of the coin (symbol() public function of ERC20)
		#[pallet::constant]
		type Symbol: Get<&'static str>;

		/// Number of decimals used to get its user representation (decimals() public function of ERC20)
		#[pallet::constant]
		type Decimals: Get<u8>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn total_supply)]
	pub type TotalSupply<T> = StorageValue<_, U256>;

	#[pallet::storage]
	#[pallet::getter(fn balance_of)]
	pub type Balance<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, U256>;

	#[pallet::storage]
	#[pallet::getter(fn allowance)]
	pub type Allowance<T: Config> =
		StorageMap<_, Blake2_128Concat, (T::AccountId, T::AccountId), U256>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub total_supply: U256,
		pub balances: Vec<(T::AccountId, U256)>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { total_supply: Default::default(), balances: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			<TotalSupply<T>>::put(&self.total_supply);
			for (a, b) in &self.balances {
				<Balance<T>>::insert(a, b);
			}
		}
	}

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Transfer { from: T::AccountId, to: T::AccountId, amount: U256 },
		Approval { owner: T::AccountId, spender: T::AccountId, amount: U256 },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// TODO
		Overflow,
		/// TODO
		InsufficientFunds,
	}

	// private (non-dispatchable) functions
	impl<T: Config> Pallet<T> {
		fn transfer_impl(from: T::AccountId, to: T::AccountId, amount: U256) -> DispatchResult {
			let from_balance = <Balance<T>>::get(&from).unwrap_or(U256::zero());
			let new_from_balance =
				from_balance.checked_sub(amount).ok_or(Error::<T>::InsufficientFunds)?;
			let to_balance = <Balance<T>>::get(&to).unwrap_or(U256::zero());
			let new_to_balance = to_balance.checked_add(amount).ok_or(Error::<T>::Overflow)?;

			<Balance<T>>::insert(&from, new_from_balance);
			<Balance<T>>::insert(&to, new_to_balance);

			Self::deposit_event(Event::Transfer { from, to, amount });

			Ok(())
		}
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)] // TODO
		pub fn transfer(origin: OriginFor<T>, to: T::AccountId, amount: U256) -> DispatchResult {
			let owner = ensure_signed(origin)?;
			<Pallet<T>>::transfer_impl(owner, to, amount)
		}

		#[pallet::weight(10_000)] // TODO
		pub fn approve(
			origin: OriginFor<T>,
			spender: T::AccountId,
			amount: U256,
		) -> DispatchResult {
			todo!()
		}

		#[pallet::weight(10_000)] // TODO
		pub fn transfer_from(
			origin: OriginFor<T>,
			from: T::AccountId,
			to: T::AccountId,
			amount: U256,
		) -> DispatchResult {
			todo!()
		}
	}
}
