use crate::{Config, Pallet};
use frame_support::{pallet_prelude::*, traits::Get, weights::Weight};

pub fn migrate<T: Config>() -> Weight {
	let mut weight: Weight = Weight::zero();
	let mut version = StorageVersion::get::<Pallet<T>>();

	if version < 1 {
		weight = weight.saturating_add(version::v1::migrate::<T>());
		version = StorageVersion::new(1);
	}

	if version == 1 {
		weight = weight.saturating_add(version::v2::migrate::<T>());
		version = StorageVersion::new(2);
	}

	version.put::<Pallet<T>>();
	weight
}

mod version {
	use super::*;

	pub mod v1 {
		use super::*;
		use frame_support::storage_alias;
		use sp_std::vec::Vec;

		use crate::{AccountIdOf, BalanceOf, TipsBalance};

		pub fn migrate<T: Config>() -> Weight {
			let mut weight = T::DbWeight::get().writes(1);

			pub type TipsBalanceOf<T> = TipsBalance<BalanceOf<T>, AccountIdOf<T>, Vec<u8>>;

			#[storage_alias]
			type TipsBalanceByReference<T: Config> = StorageNMap<
				Tipping,
				(
					NMapKey<Blake2_128Concat, Vec<u8>>,
					NMapKey<Blake2_128Concat, Vec<u8>>,
					NMapKey<Blake2_128Concat, Vec<u8>>,
					NMapKey<Blake2_128Concat, Vec<u8>>,
				),
				TipsBalanceOf<T>,
			>;

			TipsBalanceByReference::<T>::translate_values(|tips_balance: TipsBalanceOf<T>| {
				if tips_balance.get_server_id() == b"myriad" {
					weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

					let amount = *tips_balance.get_amount();
					let new_tips_balance_info =
						tips_balance.get_tips_balance_info().clone().set_server_id(&b"0".to_vec());

					let mut new_tips_balance = TipsBalance::new(&new_tips_balance_info, &amount);
					let new_key = new_tips_balance.key();

					// Handle duplicate
					if let Some(tips_balance) = TipsBalanceByReference::<T>::get(&new_key) {
						new_tips_balance.add_amount(*tips_balance.get_amount());
					}

					if let Some(account_id) = new_tips_balance.get_account_id().clone() {
						new_tips_balance.set_account_id(&account_id);
					}

					TipsBalanceByReference::<T>::insert(&new_key, new_tips_balance);

					return None
				}

				Some(tips_balance)
			});

			weight
		}
	}

	pub mod v2 {
		use super::*;
		use frame_support::storage_alias;
		use sp_std::vec::Vec;

		use crate::{
			AccountIdOf, BalanceOf, TipsBalance,
			TipsBalanceByReference as NewTipsBalanceByReference, TipsBalanceInfo,
		};

		pub fn migrate<T: Config>() -> Weight {
			let mut weight = T::DbWeight::get().writes(1);

			pub type TipsBalanceOf<T> = TipsBalance<BalanceOf<T>, AccountIdOf<T>, Vec<u8>>;
			pub type ServerOf<T> = Server<AccountIdOf<T>>;

			#[derive(Encode, Decode, Clone, Debug)]
			pub struct Server<AccountId> {
				id: u64,
				owner: AccountId,
				api_url: Vec<u8>,
			}
			impl<AccountId> Server<AccountId> {
				pub fn get_owner(&self) -> &AccountId {
					&self.owner
				}
			}

			#[storage_alias]
			type ServerById<T: Config> = StorageMap<Server, Blake2_128Concat, u64, ServerOf<T>>;

			#[storage_alias]
			type TipsBalanceByReference<T: Config> = StorageNMap<
				Tipping,
				(
					NMapKey<Blake2_128Concat, Vec<u8>>,
					NMapKey<Blake2_128Concat, Vec<u8>>,
					NMapKey<Blake2_128Concat, Vec<u8>>,
					NMapKey<Blake2_128Concat, Vec<u8>>,
				),
				TipsBalanceOf<T>,
			>;

			if let Some(server) = ServerById::<T>::get(0) {
				let server_id = server.get_owner();

				TipsBalanceByReference::<T>::translate_values(|tips_balance: TipsBalanceOf<T>| {
					weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

					let amount = *tips_balance.get_amount();
					let account_id = tips_balance.get_account_id();
					let tips_balance_info = tips_balance.get_tips_balance_info();

					let old_server_id = tips_balance_info.get_server_id();
					let reference_type = tips_balance_info.get_reference_type();
					let reference_id = tips_balance_info.get_reference_id();
					let ft_identifier = tips_balance_info.get_ft_identifier();

					let new_tips_balance_info = TipsBalanceInfo::new(
						server_id,
						reference_type,
						reference_id,
						ft_identifier,
					);

					let mut new_tips_balance = TipsBalance::new(&new_tips_balance_info, &amount);

					if let Some(account_id) = account_id {
						new_tips_balance.set_account_id(account_id);
					}

					if old_server_id == b"0" {
						NewTipsBalanceByReference::<T>::insert(
							new_tips_balance.key(),
							new_tips_balance,
						);

						return None
					}

					Some(tips_balance)
				});
			}

			weight
		}
	}
}
