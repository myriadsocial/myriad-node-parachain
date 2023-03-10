use crate::{
	AccountIdOf, Config, Pallet, Server as NewServer, ServerByApiUrl, ServerById as NewServerById,
	ServerByOwner as NewServerByOwner, ServerCount as NewServerCount, ServerId,
	ServerIndex as NewServerIndex, ServerOf,
};
use frame_support::{
	pallet_prelude::*,
	sp_runtime::traits::SaturatedConversion,
	storage_alias,
	traits::{Get, StorageVersion},
	weights::Weight,
	Blake2_128Concat,
};
use sp_std::vec::Vec;

pub fn migrate<T: Config>() -> Weight {
	let mut weight: Weight = Weight::zero();
	let mut version = StorageVersion::get::<Pallet<T>>();

	if version < 1 {
		weight = weight.saturating_add(versions::v1::migrate::<T>());
		version = StorageVersion::new(1);
	}

	if version == 1 {
		weight = weight.saturating_add(versions::v2::migrate::<T>());
		version = StorageVersion::new(2);
	}

	if version == 2 {
		weight = weight.saturating_add(versions::v3::migrate::<T>());
		version = StorageVersion::new(3);
	}

	if version == 3 {
		weight = weight.saturating_add(versions::v4::migrate::<T>());
		version = StorageVersion::new(4);
	}

	if version == 4 {
		weight = weight.saturating_add(versions::v5::migrate::<T>());
		version = StorageVersion::new(5);
	}

	if version == 5 {
		weight = weight.saturating_add(versions::v6::migrate::<T>());
		version = StorageVersion::new(6);
	}

	if version == 6 {
		weight = weight.saturating_add(versions::v7::migrate::<T>());
		version = StorageVersion::new(7);
	}

	if version == 7 {
		weight = weight.saturating_add(versions::v8::migrate::<T>());
		version = StorageVersion::new(8);
	}

	version.put::<Pallet<T>>();
	weight
}

mod versions {
	use super::*;

	pub mod v1 {
		use super::*;

		pub fn migrate<T: Config>() -> Weight {
			let mut weight = T::DbWeight::get().writes(1);

			#[derive(Encode, Decode, Clone)]
			pub struct OldServer<AccountId> {
				pub id: Vec<u8>,
				pub owner: AccountId,
				pub name: Vec<u8>,
			}

			#[derive(Encode, Decode, Clone)]
			pub struct Server<AccountId> {
				pub id: Vec<u8>,
				pub owner: AccountId,
				pub name: Vec<u8>,
				pub api_url: Vec<u8>,
				pub web_url: Vec<u8>,
			}

			#[storage_alias]
			type ServerById<T: Config> =
				StorageMap<Server, Blake2_128Concat, Vec<u8>, Server<AccountIdOf<T>>>;

			ServerById::<T>::translate(|_key, old: OldServer<AccountIdOf<T>>| {
				weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));
				Some(Server {
					id: old.id,
					owner: old.owner,
					name: old.name,
					api_url: "https://api.example.com".as_bytes().to_vec(),
					web_url: "https://web.example.com".as_bytes().to_vec(),
				})
			});

			weight
		}
	}

	pub mod v2 {
		use super::*;

		pub fn migrate<T: Config>() -> Weight {
			let mut weight = T::DbWeight::get().writes(1);

			#[allow(dead_code)]
			#[derive(Encode, Decode, Clone)]
			pub struct OldServer<AccountId> {
				id: Vec<u8>,
				owner: AccountId,
				name: Vec<u8>,
				api_url: Vec<u8>,
				web_url: Vec<u8>,
			}

			#[allow(dead_code)]
			#[derive(Encode, Decode, Clone)]
			pub struct Server<AccountId> {
				id: u64,
				owner: AccountId,
				api_url: Vec<u8>,
			}
			impl<AccountId: Clone> Server<AccountId> {
				pub fn new(id: u64, owner: &AccountId, api_url: &[u8]) -> Self {
					Self { id, owner: owner.clone(), api_url: api_url.to_vec() }
				}
			}

			#[storage_alias]
			type OldServerById<T: Config> =
				StorageMap<Server, Blake2_128Concat, Vec<u8>, OldServer<AccountIdOf<T>>>;

			#[storage_alias]
			type ServerById<T: Config> =
				StorageMap<Server, Blake2_128Concat, ServerId, Server<AccountIdOf<T>>>;

			#[storage_alias]
			type ServerByOwner<T: Config> = StorageDoubleMap<
				Server,
				Blake2_128Concat,
				AccountIdOf<T>,
				Blake2_128Concat,
				ServerId,
				Server<AccountIdOf<T>>,
			>;

			OldServerById::<T>::translate(|_key, old: OldServer<AccountIdOf<T>>| {
				weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

				let new_server = Server::new(0, &old.owner, &old.api_url);

				ServerById::<T>::insert(0, new_server.clone());
				ServerByOwner::<T>::insert(old.owner, 0, new_server);
				NewServerCount::<T>::set(1);

				None
			});

			weight
		}
	}

	pub mod v3 {
		use super::*;

		pub fn migrate<T: Config>() -> Weight {
			let mut weight = T::DbWeight::get().writes(1);

			NewServerById::<T>::translate(|server_id: ServerId, old: ServerOf<T>| {
				weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

				if server_id == 0_u64 {
					return Some(old)
				}

				None
			});

			NewServerByOwner::<T>::translate(|_owner, server_id: ServerId, old: ServerOf<T>| {
				weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

				if server_id == 0_u64 {
					return Some(old)
				}

				None
			});

			weight = weight.saturating_add(T::DbWeight::get().writes(2));

			NewServerCount::<T>::set(1);
			NewServerIndex::<T>::set(1);

			weight
		}
	}

	pub mod v4 {
		use super::*;

		pub fn migrate<T: Config>() -> Weight {
			let mut weight = T::DbWeight::get().writes(1);

			NewServerById::<T>::translate(|server_id: ServerId, server: ServerOf<T>| {
				weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

				ServerByApiUrl::<T>::insert(server.get_api_url(), server_id);

				Some(server)
			});

			weight
		}
	}

	pub mod v5 {
		use crate::BalanceOf;

		use super::*;

		pub fn migrate<T: Config>() -> Weight {
			let mut weight = T::DbWeight::get().writes(1);

			#[allow(dead_code)]
			#[derive(Encode, Decode, Clone)]
			pub struct OldServer<AccountId> {
				id: u64,
				owner: AccountId,
				api_url: Vec<u8>,
			}

			#[allow(dead_code)]
			#[derive(Encode, Decode, Clone)]
			pub struct Server<AccountId, Balance> {
				id: u64,
				owner: AccountId,
				api_url: Vec<u8>,
				staked_amount: Balance,
			}

			#[storage_alias]
			type ServerById<T: Config> = StorageMap<
				Server,
				Blake2_128Concat,
				ServerId,
				Server<AccountIdOf<T>, BalanceOf<T>>,
			>;

			#[storage_alias]
			type ServerByOwner<T: Config> = StorageDoubleMap<
				Server,
				Blake2_128Concat,
				AccountIdOf<T>,
				Blake2_128Concat,
				ServerId,
				Server<AccountIdOf<T>, BalanceOf<T>>,
			>;

			ServerById::<T>::translate(|server_id: ServerId, old: OldServer<AccountIdOf<T>>| {
				weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

				let new_server = Server {
					id: server_id,
					owner: old.owner.clone(),
					api_url: old.api_url,
					staked_amount: 0u128.saturated_into::<BalanceOf<T>>(),
				};

				ServerByOwner::<T>::insert(&old.owner, server_id, &new_server);

				Some(new_server)
			});

			weight
		}
	}

	pub mod v6 {
		use crate::BalanceOf;

		use super::*;

		pub fn migrate<T: Config>() -> Weight {
			let mut weight = T::DbWeight::get().writes(1);

			#[allow(dead_code)]
			#[derive(Encode, Decode, Clone)]
			pub struct OldServer<AccountId, Balance> {
				id: u64,
				owner: AccountId,
				api_url: Vec<u8>,
				staked_amount: Balance,
			}

			NewServerById::<T>::translate(
				|server_id: ServerId, old: OldServer<AccountIdOf<T>, BalanceOf<T>>| {
					weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

					let new_server =
						NewServer::new(server_id, &old.owner, &old.api_url, old.staked_amount);

					NewServerByOwner::<T>::insert(&old.owner, server_id, &new_server);

					Some(new_server)
				},
			);

			weight
		}
	}

	pub mod v7 {
		use crate::BalanceOf;
		use frame_support::sp_runtime::traits::Zero;
		use frame_system::pallet_prelude::BlockNumberFor;

		use super::*;

		pub fn migrate<T: Config>() -> Weight {
			let mut weight = T::DbWeight::get().writes(1);

			pub type ServerOf<T> = NewServer<AccountIdOf<T>, BalanceOf<T>, BlockNumberFor<T>>;

			#[storage_alias]
			type RewardBalance<T: Config> = StorageNMap<
				Tipping,
				(
					NMapKey<Blake2_128Concat, AccountIdOf<T>>,
					NMapKey<Blake2_128Concat, u64>,
					NMapKey<Blake2_128Concat, Vec<u8>>,
				),
				BalanceOf<T>,
				ValueQuery,
			>;

			NewServerById::<T>::translate(|server_id: ServerId, server: ServerOf<T>| {
				weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

				if server_id == 0u64 {
					return Some(server)
				}

				if server.get_stake_amount() > &Zero::zero() {
					return Some(server)
				}

				ServerByApiUrl::<T>::remove(server.get_api_url());

				None
			});

			NewServerByOwner::<T>::translate(
				|owner: AccountIdOf<T>, server_id: ServerId, server: ServerOf<T>| {
					if server_id == 0u64 {
						return Some(server)
					}

					if server.get_stake_amount() > &Zero::zero() {
						return Some(server)
					}

					let mut data = Vec::new();
					let reward_balance = RewardBalance::<T>::drain_prefix((&owner, server_id));

					for (ft_identifier, amount) in reward_balance {
						if amount > Zero::zero() {
							data.push((ft_identifier, amount));
						}
					}

					if !data.is_empty() {
						for (ft_identifier, amount) in data.iter() {
							RewardBalance::<T>::insert((&owner, server_id, ft_identifier), amount);
						}

						return Some(server)
					}

					None
				},
			);

			weight
		}
	}

	pub mod v8 {
		use super::*;

		pub fn migrate<T: Config>() -> Weight {
			let mut weight = T::DbWeight::get().reads_writes(2, 1);
			let server = NewServerById::<T>::get(0);

			if let Some(server) = server {
				weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

				let api_url = server.get_api_url();
				let server_id = ServerByApiUrl::<T>::get(api_url);

				if let Some(server_id) = server_id {
					if server_id != server.get_id() {
						weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

						let unknown_server = NewServerById::<T>::get(server_id);

						if let Some(server) = unknown_server {
							weight = weight.saturating_add(T::DbWeight::get().writes(1));
							let owner = server.get_owner();
							NewServerByOwner::<T>::remove(owner, server_id);
						}

						NewServerById::<T>::remove(server_id);
					}
				}

				ServerByApiUrl::<T>::insert(api_url, server.get_id());
			}

			let total_servers = NewServerById::<T>::iter_keys().count();

			NewServerCount::<T>::set(total_servers as u64);

			weight
		}
	}
}
