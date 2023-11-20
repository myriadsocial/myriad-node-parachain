use crate::{self as pallet_access_token, mock::*, Error, Scopes, TimelineId};
use frame_support::{
	assert_noop, assert_ok,
	sp_runtime::traits::{Hash, Keccak256},
	traits::OnInitialize,
};

#[test]
fn create_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let owner = account_key("alice");
		let owner_origin = RuntimeOrigin::signed(owner);

		let access_token = pallet_access_token::AccessToken::new(
			owner.clone(),
			Keccak256::hash("hash".as_bytes()),
			Scopes::<TimelineId>::default(),
			0,
		);

		assert_ok!(AccessToken::create(
			owner_origin,
			Keccak256::hash("hash".as_bytes()),
			Scopes::<TimelineId>::default()
		));

		assert_eq!(
			AccessToken::all_access_tokens_by_owner(owner),
			Some(vec![access_token.clone()])
		);
		assert_eq!(
			AccessToken::access_token_by_hash(Keccak256::hash("hash".as_bytes())),
			Some(access_token)
		);
		assert_eq!(AccessToken::access_token_count(), 1);
		assert_eq!(AccessToken::access_token_index(), 1);
	})
}

// #[test]
// pub fn transfer_owner_works() {
// 	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
// 		let owner = account_key("alice");
// 		let owner_origin = RuntimeOrigin::signed(owner);

// 		let access_token_id = 0u64;
// 		let api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

// 		let new_owner = account_key("bob");
// 		let access_token = pallet_access_token::AccessToken::new(access_token_id, &new_owner, &api_url, 3);

// 		assert_ok!(AccessToken::register(RuntimeOrigin::signed(owner), api_url, None));
// 		assert_ok!(AccessToken::update_access_token(owner_origin, 0, ActionType::TransferOwner(new_owner)));

// 		assert_eq!(AccessToken::access_token_by_id(access_token_id), Some(access_token.clone()));
// 		assert_eq!(AccessToken::access_token_by_owner(owner, access_token_id), None);
// 		assert_eq!(AccessToken::access_token_by_owner(new_owner, access_token_id), Some(access_token));
// 	})
// }

// #[test]
// pub fn change_api_url_works() {
// 	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
// 		let owner = account_key("alice");
// 		let owner_origin = RuntimeOrigin::signed(owner);

// 		let access_token_id = 0u64;
// 		let api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

// 		let new_api_url = "https://api.testnet.myriad.social".as_bytes().to_vec();
// 		let access_token = pallet_access_token::AccessToken::new(access_token_id, &owner, &new_api_url, 3);

// 		assert_ok!(AccessToken::register(RuntimeOrigin::signed(owner), api_url.clone(), None));
// 		assert_ok!(AccessToken::update_access_token(
// 			owner_origin,
// 			0,
// 			ActionType::UpdateApiUrl(new_api_url.clone())
// 		));

// 		assert_eq!(AccessToken::access_token_by_api_url(api_url), None);
// 		assert_eq!(AccessToken::access_token_by_id(access_token_id), Some(access_token));
// 		assert_eq!(AccessToken::access_token_by_api_url(new_api_url), Some(access_token_id));
// 	})
// }

// #[test]
// pub fn deregister_works() {
// 	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
// 		let owner = account_key("alice");
// 		let owner_origin = RuntimeOrigin::signed(owner);

// 		let access_token_id = 0u64;
// 		let api_url = "https://api.dev.myriad.social".as_bytes().to_vec();
// 		let access_token =
// 			pallet_access_token::AccessToken::new(access_token_id, &owner, &api_url, 3).set_unstaked_at(Some(20));

// 		System::set_block_number(10);

// 		assert_ok!(AccessToken::register(RuntimeOrigin::signed(owner), api_url, None));
// 		assert_ok!(AccessToken::unregister(owner_origin, access_token_id));

// 		assert_eq!(AccessToken::tasks(20), vec![0]);
// 		assert_eq!(AccessToken::access_token_by_id(access_token_id), Some(access_token.clone()));
// 		assert_eq!(AccessToken::access_token_by_owner(owner, access_token_id), Some(access_token));
// 	})
// }

// #[test]
// pub fn cancel_deregister_works() {
// 	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
// 		let owner = account_key("alice");
// 		let other_owner = account_key("bob");

// 		let access_token_id = 0u64;
// 		let other_access_token_id = 1u64;

// 		let api_url = "https://api.dev.myriad.social".as_bytes().to_vec();
// 		let other_api_url = "https://api.testnet.myriad.social".as_bytes().to_vec();

// 		let access_token = pallet_access_token::AccessToken::new(access_token_id, &owner, &api_url, 3);

// 		System::set_block_number(10);

// 		assert_ok!(AccessToken::register(RuntimeOrigin::signed(owner), api_url, None));
// 		assert_ok!(AccessToken::unregister(RuntimeOrigin::signed(owner), access_token_id));

// 		assert_ok!(AccessToken::register(RuntimeOrigin::signed(other_owner), other_api_url, None));
// 		assert_ok!(AccessToken::unregister(RuntimeOrigin::signed(other_owner), other_access_token_id));

// 		assert_eq!(AccessToken::tasks(20), vec![0, 1]);

// 		assert_ok!(AccessToken::cancel_unregister(RuntimeOrigin::signed(owner), access_token_id));

// 		assert_eq!(AccessToken::access_token_by_id(access_token_id), Some(access_token.clone()));
// 		assert_eq!(AccessToken::access_token_by_owner(owner, access_token_id), Some(access_token));
// 		assert_eq!(AccessToken::tasks(20), vec![1]);

// 		assert_ok!(AccessToken::cancel_unregister(RuntimeOrigin::signed(other_owner), other_access_token_id));

// 		assert_eq!(AccessToken::tasks(20), Vec::<u64>::new());
// 	})
// }

// #[test]
// pub fn increase_stake_amount_works() {
// 	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
// 		let owner = account_key("alice");
// 		let api_url = "https://api.dev.myriad.social".as_bytes().to_vec();
// 		let access_token_id = 0u64;
// 		let amount = 3;

// 		let access_token = pallet_access_token::AccessToken::new(access_token_id, &owner, &api_url, 6);
// 		let access_token_account_id = AccessToken::access_token_account_id(access_token_id);

// 		assert_ok!(AccessToken::register(RuntimeOrigin::signed(owner), api_url, None));
// 		assert_ok!(AccessToken::update_access_token(
// 			RuntimeOrigin::signed(owner),
// 			access_token_id,
// 			ActionType::StakeAmount(amount)
// 		));

// 		assert_eq!(AccessToken::access_token_by_id(access_token_id), Some(access_token.clone()));
// 		assert_eq!(AccessToken::access_token_by_owner(owner, access_token_id), Some(access_token));
// 		assert_eq!(Balances::free_balance(owner), 4);
// 		assert_eq!(Balances::free_balance(access_token_account_id), 6);
// 	})
// }

// #[test]
// pub fn decrease_stake_amount_works() {
// 	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
// 		let owner = account_key("alice");
// 		let api_url = "https://api.dev.myriad.social".as_bytes().to_vec();
// 		let access_token_id = 0u64;
// 		let amount = 3;

// 		let access_token = pallet_access_token::AccessToken::new(access_token_id, &owner, &api_url, 3);
// 		let access_token_account_id = AccessToken::access_token_account_id(access_token_id);

// 		assert_ok!(AccessToken::register(RuntimeOrigin::signed(owner), api_url, None));
// 		assert_ok!(AccessToken::update_access_token(
// 			RuntimeOrigin::signed(owner),
// 			access_token_id,
// 			ActionType::StakeAmount(amount)
// 		));
// 		assert_ok!(AccessToken::update_access_token(
// 			RuntimeOrigin::signed(owner),
// 			access_token_id,
// 			ActionType::UnstakeAmount(amount)
// 		));

// 		assert_eq!(AccessToken::access_token_by_id(access_token_id), Some(access_token.clone()));
// 		assert_eq!(AccessToken::access_token_by_owner(owner, access_token_id), Some(access_token));
// 		assert_eq!(Balances::free_balance(owner), 7);
// 		assert_eq!(Balances::free_balance(access_token_account_id), 3);
// 	})
// }

// #[test]
// pub fn cant_register_when_api_url_exist() {
// 	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
// 		let owner = account_key("alice");
// 		let owner_origin = RuntimeOrigin::signed(owner);
// 		let api_url = b"https://api.dev.myriad.social".to_vec();

// 		assert_ok!(AccessToken::register(owner_origin, api_url, None));

// 		let other_owner = account_key("bob");
// 		let other_owner_origin = RuntimeOrigin::signed(other_owner);
// 		let other_api_url = b"https://api.dev.myriad.social".to_vec();

// 		assert_noop!(
// 			AccessToken::register(other_owner_origin, other_api_url, None),
// 			Error::<Test>::AlreadyExists,
// 		);
// 	})
// }

// #[test]
// pub fn cant_update_access_token_where_already_deregister() {
// 	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
// 		let owner = account_key("alice");
// 		let owner_origin = RuntimeOrigin::signed(owner);

// 		let access_token_id = 0u64;
// 		let api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

// 		System::set_block_number(10);

// 		assert_ok!(AccessToken::register(RuntimeOrigin::signed(owner), api_url, None));
// 		assert_ok!(AccessToken::unregister(owner_origin.clone(), access_token_id));

// 		let new_owner = account_key("bob");
// 		let new_api_url = "https://api.testnet.myriad.social".as_bytes().to_vec();
// 		let new_action = ActionType::StakeAmount(10);

// 		assert_noop!(
// 			AccessToken::update_access_token(
// 				owner_origin.clone(),
// 				access_token_id,
// 				ActionType::TransferOwner(new_owner)
// 			),
// 			Error::<Test>::WaitingToUnstaked,
// 		);

// 		assert_noop!(
// 			AccessToken::update_access_token(
// 				owner_origin.clone(),
// 				access_token_id,
// 				ActionType::UpdateApiUrl(new_api_url)
// 			),
// 			Error::<Test>::WaitingToUnstaked,
// 		);

// 		assert_noop!(
// 			AccessToken::update_access_token(owner_origin, access_token_id, new_action),
// 			Error::<Test>::WaitingToUnstaked,
// 		);
// 	})
// }

// #[test]
// pub fn cant_transfer_owner_when_access_token_id_not_exist() {
// 	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
// 		let owner = account_key("alice");
// 		let owner_origin = RuntimeOrigin::signed(owner);

// 		let fake_id = 0u64;
// 		let new_owner = account_key("bob");

// 		assert_noop!(
// 			AccessToken::update_access_token(owner_origin, fake_id, ActionType::TransferOwner(new_owner)),
// 			Error::<Test>::NotExists,
// 		);
// 	})
// }

// #[test]
// pub fn cant_transfer_owner_when_not_owner() {
// 	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
// 		let owner = account_key("alice");
// 		let owner_origin = RuntimeOrigin::signed(owner);

// 		let access_token_id = 0u64;
// 		let api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

// 		assert_ok!(AccessToken::register(owner_origin, api_url, None));

// 		let fake_owner = account_key("bob");
// 		let fake_owner_origin = RuntimeOrigin::signed(fake_owner);
// 		let new_owner = account_key("john");

// 		assert_noop!(
// 			AccessToken::update_access_token(
// 				fake_owner_origin,
// 				access_token_id,
// 				ActionType::TransferOwner(new_owner)
// 			),
// 			Error::<Test>::Unauthorized,
// 		);
// 	})
// }

// #[test]
// pub fn cant_change_api_url_when_access_token_id_not_exist() {
// 	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
// 		let owner = account_key("alice");
// 		let owner_origin = RuntimeOrigin::signed(owner);

// 		let fake_id = 0u64;
// 		let new_api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

// 		assert_noop!(
// 			AccessToken::update_access_token(owner_origin, fake_id, ActionType::UpdateApiUrl(new_api_url)),
// 			Error::<Test>::NotExists,
// 		);
// 	})
// }

// #[test]
// pub fn cant_change_api_url_when_not_owner() {
// 	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
// 		let owner = account_key("alice");
// 		let owner_origin = RuntimeOrigin::signed(owner);

// 		let access_token_id = 0u64;
// 		let api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

// 		assert_ok!(AccessToken::register(owner_origin, api_url, None));

// 		let fake_owner = account_key("bob");
// 		let fake_owner_origin = RuntimeOrigin::signed(fake_owner);
// 		let new_api_url = "https://api.testnet.myriad.social".as_bytes().to_vec();

// 		assert_noop!(
// 			AccessToken::update_access_token(
// 				fake_owner_origin,
// 				access_token_id,
// 				ActionType::UpdateApiUrl(new_api_url)
// 			),
// 			Error::<Test>::Unauthorized,
// 		);
// 	})
// }

// #[test]
// pub fn cant_change_api_url_when_api_url_exist() {
// 	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
// 		let owner = account_key("alice");
// 		let owner_origin = RuntimeOrigin::signed(owner);

// 		let access_token_id = 0u64;
// 		let api_url = b"https://api.dev.myriad.social".to_vec();

// 		assert_ok!(AccessToken::register(owner_origin.clone(), api_url, None));

// 		let other_owner = account_key("bob");
// 		let other_owner_origin = RuntimeOrigin::signed(other_owner);
// 		let other_api_url = b"https://api.testnet.myriad.social".to_vec();

// 		assert_ok!(AccessToken::register(other_owner_origin, other_api_url, None));

// 		let new_api_url = b"https://api.testnet.myriad.social".to_vec();

// 		assert_noop!(
// 			AccessToken::update_access_token(owner_origin, access_token_id, ActionType::UpdateApiUrl(new_api_url)),
// 			Error::<Test>::AlreadyExists,
// 		);
// 	})
// }

// #[test]
// pub fn cant_deregister_when_access_token_id_not_exist() {
// 	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
// 		let owner = account_key("alice");
// 		let owner_origin = RuntimeOrigin::signed(owner);

// 		let fake_id = 0u64;

// 		assert_noop!(AccessToken::unregister(owner_origin, fake_id), Error::<Test>::NotExists);
// 	})
// }

// #[test]
// pub fn cant_deregister_when_not_owner() {
// 	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
// 		let owner = account_key("alice");
// 		let owner_origin = RuntimeOrigin::signed(owner);

// 		let access_token_id = 0u64;
// 		let api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

// 		assert_ok!(AccessToken::register(owner_origin, api_url, None));

// 		let fake_owner = account_key("bob");
// 		let fake_owner_origin = RuntimeOrigin::signed(fake_owner);

// 		assert_noop!(AccessToken::unregister(fake_owner_origin, access_token_id), Error::<Test>::Unauthorized,);
// 	})
// }

// #[test]
// pub fn cant_deregister_when_max_scheduled_per_block_over_limit() {
// 	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
// 		let owner = account_key("john");
// 		let owner_origin = RuntimeOrigin::signed(owner);

// 		System::set_block_number(1);

// 		for n in 0..6 {
// 			assert_ok!(AccessToken::register(owner_origin.clone(), vec![n], None));
// 		}

// 		for n in 0..5 {
// 			assert_ok!(AccessToken::unregister(owner_origin.clone(), n));
// 		}

// 		assert_noop!(AccessToken::unregister(owner_origin, 5), Error::<Test>::FailedToSchedule);
// 	})
// }

// #[test]
// pub fn cant_cancel_deregister_when_access_token_id_not_exists() {
// 	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
// 		let owner = account_key("alice");
// 		let access_token_id = 0u64;

// 		assert_noop!(
// 			AccessToken::cancel_unregister(RuntimeOrigin::signed(owner), access_token_id),
// 			Error::<Test>::NotExists
// 		);
// 	})
// }

// #[test]
// pub fn cant_cancel_deregister_when_unauthorized() {
// 	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
// 		let owner = account_key("alice");
// 		let other_owner = account_key("bob");

// 		let access_token_id = 0u64;
// 		let api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

// 		assert_ok!(AccessToken::register(RuntimeOrigin::signed(owner), api_url, None));

// 		assert_noop!(
// 			AccessToken::cancel_unregister(RuntimeOrigin::signed(other_owner), access_token_id),
// 			Error::<Test>::Unauthorized
// 		);
// 	})
// }

// #[test]
// pub fn cant_cancel_deregister_when_not_unstaked() {
// 	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
// 		let owner = account_key("alice");
// 		let access_token_id = 0u64;
// 		let api_url = "https://api.dev.myriad.social".as_bytes().to_vec();

// 		assert_ok!(AccessToken::register(RuntimeOrigin::signed(owner), api_url, None));

// 		assert_noop!(
// 			AccessToken::cancel_unregister(RuntimeOrigin::signed(owner), access_token_id),
// 			Error::<Test>::NotExists
// 		);
// 	})
// }

// #[test]
// pub fn cant_increase_stake_amount_when_access_token_id_not_exist() {
// 	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
// 		let owner = account_key("alice");
// 		let access_token_id = 0u64;
// 		let amount = 10;

// 		assert_noop!(
// 			AccessToken::update_access_token(
// 				RuntimeOrigin::signed(owner),
// 				access_token_id,
// 				ActionType::StakeAmount(amount)
// 			),
// 			Error::<Test>::NotExists,
// 		);
// 	})
// }

// #[test]
// pub fn cant_increase_stake_amount_when_not_owner() {
// 	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
// 		let owner = account_key("alice");
// 		let api_url = "https://api.dev.myriad.social".as_bytes().to_vec();
// 		let access_token_id = 0u64;
// 		let amount = 10;

// 		let other_owner = account_key("bob");

// 		assert_ok!(AccessToken::register(RuntimeOrigin::signed(owner), api_url, None));
// 		assert_noop!(
// 			AccessToken::update_access_token(
// 				RuntimeOrigin::signed(other_owner),
// 				access_token_id,
// 				ActionType::StakeAmount(amount)
// 			),
// 			Error::<Test>::Unauthorized,
// 		);
// 	})
// }
