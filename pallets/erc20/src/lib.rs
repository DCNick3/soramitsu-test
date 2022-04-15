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

	/// Amount of tokens in existence
	#[pallet::storage]
	#[pallet::getter(fn total_supply)]
	pub type TotalSupply<T> = StorageValue<_, U256>;

	/// Amount of tokens owned by each account
	#[pallet::storage]
	#[pallet::getter(fn balance_of)]
	pub type Balance<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, U256>;

	/// Remaining number of tokens that `spender` (second key)
	/// 	will be allowed to spend on behalf of `owner` (first key) using transfer_from
	#[pallet::storage]
	#[pallet::getter(fn allowance)]
	pub type Allowance<T: Config> =
		StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, T::AccountId, U256>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		/// Initial total supply of tokens
		/// _should_ be the sum of all balances
		pub total_supply: U256,
		/// Initial balances of accounts
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
		/// Tokens transfer has occurred
		Transfer {
			/// The address from which tokens were deducted
			from: T::AccountId,
			/// The address to which tokens were sent
			to: T::AccountId,
			/// Amount of tokens transferred
			amount: U256,
		},
		/// Approval was updated (either due to owner's request or spender's spending)
		Approval {
			/// Account from which tokens are allowed to be spent
			owner: T::AccountId,
			/// Account that is authorized to spend tokens
			spender: T::AccountId,
			/// Amount of tokens allowed to be spent
			amount: U256,
		},
		// I don't think that there is such a thing as "zero address" in substrate, so these are separate events
		/// Some amount of tokens is introduced into the system
		Mint {
			/// Account which gets the tokens
			account: T::AccountId,
			/// Amount of tokens minted
			amount: U256,
		},
		/// Some amount of tokens is destroyed
		Burn {
			/// Account from which the tokens are burnt
			account: T::AccountId,
			/// Amount of destroyed tokens
			amount: U256,
		},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// An overflow occurred when calculating balances
		Overflow,
		/// "from" account has insufficient funds to perform the transfer
		InsufficientFunds,
		/// "spender" account has insufficient allowance to perform the transfer
		InsufficientAllowance,
		/// An account has less tokens that were requested to be burnt
		BurnExceedsBalance,
	}

	// private (non-dispatchable) functions
	impl<T: Config> Pallet<T> {
		fn check_allowance(
			owner: T::AccountId,
			spender: T::AccountId,
			amount: U256,
		) -> Result<Option<U256>, DispatchError> {
			let allowance = <Allowance<T>>::get(owner, spender).unwrap_or(U256::zero());

			// if allowance is U256::max_value - do not spend it
			if allowance == U256::max_value() {
				return Ok(None);
			}
			Ok(Some(allowance.checked_sub(amount).ok_or(Error::<T>::InsufficientAllowance)?))
		}

		fn transfer_impl(from: T::AccountId, to: T::AccountId, amount: U256) -> DispatchResult {
			let from_balance = <Balance<T>>::get(&from).unwrap_or(U256::zero());
			let from_balance =
				from_balance.checked_sub(amount).ok_or(Error::<T>::InsufficientFunds)?;
			let to_balance = <Balance<T>>::get(&to).unwrap_or(U256::zero());
			let to_balance = to_balance.checked_add(amount).ok_or(Error::<T>::Overflow)?;

			<Balance<T>>::insert(&from, from_balance);
			<Balance<T>>::insert(&to, to_balance);

			Self::deposit_event(Event::Transfer { from, to, amount });

			Ok(())
		}

		fn approve_impl(
			owner: T::AccountId,
			spender: T::AccountId,
			amount: U256,
		) -> DispatchResult {
			<Allowance<T>>::insert(&owner, &spender, amount);

			Self::deposit_event(Event::Approval { owner, spender, amount });

			Ok(())
		}
	}

	// non-dispatchable functions, but other pallets can call them
	impl<T: Config> Pallet<T> {
		pub fn mint(account: T::AccountId, amount: U256) -> DispatchResult {
			let total_supply = <TotalSupply<T>>::get().unwrap_or(U256::zero());
			let balance = <Balance<T>>::get(&account).unwrap_or(U256::zero());

			let total_supply = total_supply.checked_add(amount).ok_or(Error::<T>::Overflow)?;
			let balance = balance.checked_add(amount).ok_or(Error::<T>::Overflow)?;

			<TotalSupply<T>>::put(total_supply);
			<Balance<T>>::insert(&account, balance);

			Self::deposit_event(Event::Mint { account, amount });

			Ok(())
		}
		pub fn burn(account: T::AccountId, amount: U256) -> DispatchResult {
			let total_supply = <TotalSupply<T>>::get().unwrap_or(U256::zero());
			let balance = <Balance<T>>::get(&account).unwrap_or(U256::zero());

			let total_supply = total_supply.checked_sub(amount).ok_or(Error::<T>::Overflow)?;
			let balance = balance.checked_sub(amount).ok_or(Error::<T>::BurnExceedsBalance)?;

			<TotalSupply<T>>::put(total_supply);
			<Balance<T>>::insert(&account, balance);

			Self::deposit_event(Event::Burn { account, amount });

			Ok(())
		}
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Moves `amount` tokens from the caller’s account to `to`.
		/// Emits a `Transfer` event.
		#[pallet::weight(10_000)] // TODO
		pub fn transfer(origin: OriginFor<T>, to: T::AccountId, amount: U256) -> DispatchResult {
			let owner = ensure_signed(origin)?;
			Self::transfer_impl(owner, to, amount)
		}

		/// Sets `amount` as the allowance of `spender` over the caller’s tokens.
		/// Emits an `Approval` event.
		#[pallet::weight(10_000)] // TODO
		pub fn approve(
			origin: OriginFor<T>,
			spender: T::AccountId,
			amount: U256,
		) -> DispatchResult {
			let owner = ensure_signed(origin)?;
			Self::approve_impl(owner, spender, amount)
		}

		/// Moves `amount` tokens from from to to using the allowance mechanism.
		/// `amount` is then deducted from the caller’s allowance.
		///
		/// Emits a `Transfer` event.
		/// Might emit `Approval` event (if the approval amount is not infinite)
		#[pallet::weight(10_000)] // TODO
		pub fn transfer_from(
			origin: OriginFor<T>,
			from: T::AccountId,
			to: T::AccountId,
			amount: U256,
		) -> DispatchResult {
			let spender = ensure_signed(origin)?;

			// first - check if there is enough approval
			let new_allowance = Self::check_allowance(from.clone(), spender.clone(), amount)?;

			// then try to transfer (and therefore check the balance)
			Self::transfer_impl(from.clone(), to, amount)?;

			// finally - spend the allowance (if it's not infinite)
			if let Some(new_allowance) = new_allowance {
				Self::approve_impl(from, spender, new_allowance)?;
			}

			// using some type to accumulate side-effects and then apply them in one step would be more "beautiful" and composable
			// but this works good enough when we need to track only two preconditions

			Ok(())
		}
	}
}
