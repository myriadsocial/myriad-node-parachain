use crate as pallet_server;

use sp_runtime::BuildStorage;
use sp_core::{sr25519, Pair, H256};
use sp_io::TestExternalities;
use sp_runtime::traits::{BlakeTwo256, IdentityLookup};

use frame_support::{
	construct_runtime, parameter_types,
	traits::{ConstU16, ConstU32, ConstU64, Everything},
	weights::Weight,
};
use frame_system as system;

use pallet_balances::AccountData;

type Block = system::mocking::MockBlock<Test>;
type Balance = u64;

construct_runtime!(
	pub enum Test
	{
		System: system = 0,
		Server: pallet_server = 50,
		Balances: pallet_balances = 51,
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
	pub static ExistentialDeposit: Balance = 0;
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
	pub const MinimumStakeAmount: u64 = 3;
	pub const ScheduledBlockTime: u32 = 10;
	pub const MaxScheduledPerBlock: u32 = 5;
}

impl pallet_server::Config for Test {
	type Currency = Balances;
	type MaxScheduledPerBlock = MaxScheduledPerBlock;
	type MinimumStakeAmount = MinimumStakeAmount;
	type RuntimeEvent = RuntimeEvent;
	type ScheduledBlockTime = ScheduledBlockTime;
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

		pallet_balances::GenesisConfig::<Test> {
			balances: vec![
				(alice_public, 10),
				(bob_public, 20),
				(john_public, 30),
				(satoshi_public, 2),
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
