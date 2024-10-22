#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_std::collections::btree_map::BTreeMap;
    use sp_std::vec::Vec;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {}

    #[pallet::storage]
    #[pallet::getter(fn get_value)]
    pub type KeyValueStore<T: Config> = StorageMap<_, Twox64Concat, Vec<u8>, Vec<u8>, OptionQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        KeyValueStored(Vec<u8>, Vec<u8>),
    }

    #[pallet::error]
    pub enum Error<T> {
        KeyNotFound,
        KeyAlreadyExists,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn store_key_value(origin: OriginFor<T>, key: Vec<u8>, value: Vec<u8>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(!KeyValueStore::<T>::contains_key(&key), Error::<T>::KeyAlreadyExists);
            KeyValueStore::<T>::insert(key.clone(), value.clone());

            Self::deposit_event(Event::KeyValueStored(key, value));
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn retrieve_value(origin: OriginFor<T>, key: Vec<u8>) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            let value = KeyValueStore::<T>::get(&key).ok_or(Error::<T>::KeyNotFound)?;

            // Logic to return value, for now, we'll just print it to the log
            log::info!("Retrieved value: {:?}", value);
            Ok(())
        }
    }
}
