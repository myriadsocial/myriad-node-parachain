use crate as pallet_tipping;

use sp_core::{
	sr25519::{self as sr25519, Signature},
	Pair, H256,
};
use sp_io::TestExternalities;
use sp_runtime::traits::{BlakeTwo256, IdentifyAccount, IdentityLookup, Verify};
use sp_runtime::BuildStorage;

use frame_support::{
	construct_runtime, parameter_types,
	traits::{AsEnsureOriginWithArg, ConstU16, ConstU32, ConstU64, Everything},
	weights::Weight,
};
use frame_system as system;

use pallet_balances::AccountData;

type Block = system::mocking::MockBlock<Test>;
type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
type Moment = u64;
type Balance = u128;
type AssetId = u32;

construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		Timestamp: pallet_timestamp = 2,
		Balances: pallet_balances = 10,
		Assets: pallet_assets,
		Tipping: pallet_tipping,
	}
);

parameter_types! {
	pub BlockWeights: system::limits::BlockWeights = system::limits::BlockWeights::simple_max(Weight::from_parts(1024, 1024));
}

impl system::Config for Test {
	type AccountData = AccountData<Balance>;
	type AccountId = sr25519::Public;
	type BaseCallFilter = Everything;
	type BlockHashCount = ConstU64<250>;
	type BlockLength = ();
	type Block = Block;
	type BlockWeights = ();
	type DbWeight = ();
	type Nonce = u32;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Lookup = IdentityLookup<Self::AccountId>;
	type MaxConsumers = ConstU32<2>;
	type OnKilledAccount = ();
	type OnNewAccount = ();
	type OnSetCode = ();
	type PalletInfo = PalletInfo;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type SS58Prefix = ConstU16<42>;
	type SystemWeightInfo = ();
	type Version = ();
}

parameter_types! {
	pub const MinimumPeriod: Moment = 10 / 2;
}

impl pallet_timestamp::Config for Test {
	type MinimumPeriod = MinimumPeriod;
	type Moment = Moment;
	type OnTimestampSet = ();
	type WeightInfo = ();
}

parameter_types! {
	pub static ExistentialDeposit: Balance = 1;
}

impl pallet_balances::Config for Test {
	type AccountStore = System;
	type Balance = Balance;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type RuntimeHoldReason = RuntimeHoldReason;
	type FreezeIdentifier = ();
	type MaxHolds = ConstU32<0>;
	type MaxFreezes = ConstU32<0>;
}

parameter_types! {
	pub const AdminFee: u8 = 5;
	pub const TransactionFee: u8 = 5;
}

impl pallet_tipping::Config for Test {
	type RuntimeCall = RuntimeCall;
	type TimeProvider = Timestamp;
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type Assets = Assets;
	type WeightInfo = ();
	type AdminFee = AdminFee;
	type TransactionFee = TransactionFee;
}

parameter_types! {
	pub const ApprovalDeposit: Balance = 1;
	pub const AssetAccountDeposit: Balance = 10;
	pub const AssetDeposit: Balance = 1;
	pub const MetadataDepositBase: Balance = 1;
	pub const MetadataDepositPerByte: Balance = 1;
	pub const RemoveItemsLimit: u32 = 1000;
	pub const StringLimit: u32 = 50;
}

impl pallet_assets::Config for Test {
	type ApprovalDeposit = ApprovalDeposit;
	type AssetAccountDeposit = AssetAccountDeposit;
	type AssetDeposit = AssetDeposit;
	type AssetId = AssetId;
	type AssetIdParameter = codec::Compact<AssetId>;
	type Balance = Balance;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
	type CallbackHandle = ();
	type CreateOrigin = AsEnsureOriginWithArg<system::EnsureSigned<AccountId>>;
	type Currency = Balances;
	type Extra = ();
	type ForceOrigin = system::EnsureRoot<AccountId>;
	type Freezer = ();
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type RemoveItemsLimit = RemoveItemsLimit;
	type RuntimeEvent = RuntimeEvent;
	type StringLimit = StringLimit;
	type WeightInfo = ();
}

pub fn account_key(s: &str) -> sr25519::Public {
	sr25519::Pair::from_string(&format!("//{}", s), None)
		.expect("static values are valud; qed")
		.public()
}

pub struct ExternalityBuilder {
	existential_deposit: u64,
}

impl ExternalityBuilder {
	pub fn build(&self) -> TestExternalities {
		let mut t = system::GenesisConfig::<Test>::default().build_storage().unwrap();

		let alice_public = account_key("alice");
		let bob_public = account_key("bob");
		let john_public = account_key("john");
		let satoshi_public = account_key("satoshi");
		let sender_1_public = account_key("sender_1");
		let sender_2_public = account_key("sender_2");
		let admin_public = account_key("admin");

		pallet_assets::GenesisConfig::<Test> {
			assets: vec![(1, alice_public, true, 1), (2, alice_public, true, 1)],
			metadata: vec![
				(1, b"DeBio".to_vec(), b"DBIO".to_vec(), 18),
				(2, b"Doge".to_vec(), b"DOGE".to_vec(), 18),
			],
			accounts: vec![
				(1, alice_public, 10),
				(1, bob_public, 20),
				(1, john_public, 30),
				(1, satoshi_public, 40),
				(1, admin_public, 50),
				(1, sender_1_public, 20_000),
				(1, sender_2_public, 20_000),
				(2, alice_public, 10),
				(2, bob_public, 20),
				(2, john_public, 30),
				(2, satoshi_public, 40),
				(2, admin_public, 50),
				(2, sender_1_public, 20_000),
				(2, sender_2_public, 20_000),
			],
		}
		.assimilate_storage(&mut t)
		.unwrap();

		pallet_balances::GenesisConfig::<Test> {
			balances: vec![
				(alice_public, 10),
				(bob_public, 20),
				(john_public, 30),
				(satoshi_public, 40),
				(admin_public, 50),
				(sender_1_public, 20_000),
				(sender_2_public, 20_000),
			],
		}
		.assimilate_storage(&mut t)
		.unwrap();

		let mut ext = TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}

	pub fn existential_deposit(mut self, existential_deposit: u64) -> Self {
		self.existential_deposit = existential_deposit;
		self
	}

	pub fn default() -> Self {
		Self { existential_deposit: 0 }
	}
}
