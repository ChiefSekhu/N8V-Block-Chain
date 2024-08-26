#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, dispatch};
use frame_system::ensure_signed;

pub trait Trait: frame_system::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as N8VTokenModule {
        TotalSupply get(fn total_supply): u64;
        Balances get(fn balance_of): map hasher(blake2_128_concat) T::AccountId => u64;
    }
}

decl_event!(
    pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
        Transfer(AccountId, AccountId, u64),
        Mint(AccountId, u64),
        Burn(AccountId, u64),
    }
);

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        #[weight = 10_000]
        pub fn mint(origin, to: T::AccountId, amount: u64) -> dispatch::DispatchResult {
            let _sender = ensure_signed(origin)?;

            let current_supply = Self::total_supply();
            let new_supply = current_supply.checked_add(amount).ok_or("Overflow")?;
            <TotalSupply>::put(new_supply);

            let balance = Self::balance_of(&to);
            let new_balance = balance.checked_add(amount).ok_or("Overflow")?;
            <Balances<T>>::insert(&to, new_balance);

            Self::deposit_event(RawEvent::Mint(to, amount));
            Ok(())
        }

        #[weight = 10_000]
        pub fn transfer(origin, to: T::AccountId, amount: u64) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;

            let sender_balance = Self::balance_of(&sender);
            ensure!(sender_balance >= amount, "Not enough balance");

            let new_sender_balance = sender_balance.checked_sub(amount).ok_or("Underflow")?;
            <Balances<T>>::insert(&sender, new_sender_balance);

            let receiver_balance = Self::balance_of(&to);
            let new_receiver_balance = receiver_balance.checked_add(amount).ok_or("Overflow")?;
            <Balances<T>>::insert(&to, new_receiver_balance);

            Self::deposit_event(RawEvent::Transfer(sender, to, amount));
            Ok(())
        }

        #[weight = 10_000]
        pub fn burn(origin, amount: u64) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;

            let balance = Self::balance_of(&sender);
            ensure!(balance >= amount, "Not enough balance");

            let new_balance = balance.checked_sub(amount).ok_or("Underflow")?;
            <Balances<T>>::insert(&sender, new_balance);

            let current_supply = Self::total_supply();
            let new_supply = current_supply.checked_sub(amount).ok_or("Underflow")?;
            <TotalSupply>::put(new_supply);

            Self::deposit_event(RawEvent::Burn(sender, amount));
            Ok(())
        }
    }
}
