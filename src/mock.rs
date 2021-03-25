//! Mocks for the stp258tokens module.

#![cfg(test)]

use super::*;
use frame_support::{
	construct_runtime, parameter_types,
	traits::{ChangeMembers, Contains, ContainsLengthBound, SaturatingCurrencyToVote},
};
use stp258_traits::parameter_type_with_key; 
use sp_core::H256;
use sp_runtime::{testing::Header, traits::IdentityLookup, AccountId32, Permill, Perbill};
use sp_std::cell::RefCell;

pub type AccountId = AccountId32;
pub type CurrencyId = u32;
pub type Balance = u64;

pub const DNAR: CurrencyId = 1;
pub const JUSD: CurrencyId = 2;
pub const SETT: CurrencyId = 3;
pub const ALICE: AccountId = AccountId32::new([0u8; 32]);
pub const BOB: AccountId = AccountId32::new([1u8; 32]);
pub const SERPER: AccountId = AccountId32::new([3u8; 32]);
pub const SETTPAY: AccountId = AccountId32::new([4u8; 32]);
pub const TREASURY_ACCOUNT: AccountId = AccountId32::new([2u8; 32]);
pub const ID_1: LockIdentifier = *b"1       ";
pub const ID_2: LockIdentifier = *b"2       ";

use crate as stp258_tokens;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for Runtime {
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = ::sp_runtime::traits::BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type BlockWeights = ();
	type BlockLength = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type DbWeight = ();
	type BaseCallFilter = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
}

thread_local! {
	static TEN_TO_FOURTEEN: RefCell<Vec<AccountId>> = RefCell::new(vec![
		AccountId32::new([10u8; 32]),
		AccountId32::new([11u8; 32]),
		AccountId32::new([12u8; 32]),
		AccountId32::new([13u8; 32]),
		AccountId32::new([14u8; 32]),
	]);
}

pub struct TenToFourteen;
impl Contains<AccountId> for TenToFourteen {
	fn sorted_members() -> Vec<AccountId> {
		TEN_TO_FOURTEEN.with(|v| v.borrow().clone())
	}
	#[cfg(feature = "runtime-benchmarks")]
	fn add(new: &AccountId) {
		TEN_TO_FOURTEEN.with(|v| {
			let mut members = v.borrow_mut();
			members.push(*new);
			members.sort();
		})
	}
}

impl ContainsLengthBound for TenToFourteen {
	fn max_len() -> usize {
		TEN_TO_FOURTEEN.with(|v| v.borrow().len())
	}
	fn min_len() -> usize {
		0
	}
}

parameter_types! {
	pub const ProposalBond: Permill = Permill::from_percent(5);
	pub const ProposalBondMinimum: u64 = 1;
	pub const SpendPeriod: u64 = 2;
	pub const Burn: Permill = Permill::from_percent(50);
	pub const TreasuryModuleId: ModuleId = ModuleId(*b"py/trsry");
	pub const GetTokenId: CurrencyId = DNAR;
}

impl pallet_treasury::Config for Runtime {
	type ModuleId = TreasuryModuleId;
	type Currency = CurrencyAdapter<Runtime, GetTokenId>;
	type ApproveOrigin = frame_system::EnsureRoot<AccountId>;
	type RejectOrigin = frame_system::EnsureRoot<AccountId>;
	type Event = Event;
	type OnSlash = ();
	type ProposalBond = ProposalBond;
	type ProposalBondMinimum = ProposalBondMinimum;
	type SpendPeriod = SpendPeriod;
	type Burn = Burn;
	type BurnDestination = ();
	type SpendFunds = ();
	type WeightInfo = ();
}

thread_local! {
	pub static MEMBERS: RefCell<Vec<AccountId>> = RefCell::new(vec![]);
	pub static PRIME: RefCell<Option<AccountId>> = RefCell::new(None);
}

pub struct TestChangeMembers;
impl ChangeMembers<AccountId> for TestChangeMembers {
	fn change_members_sorted(incoming: &[AccountId], outgoing: &[AccountId], new: &[AccountId]) {
		// new, incoming, outgoing must be sorted.
		let mut new_sorted = new.to_vec();
		new_sorted.sort();
		assert_eq!(new, &new_sorted[..]);

		let mut incoming_sorted = incoming.to_vec();
		incoming_sorted.sort();
		assert_eq!(incoming, &incoming_sorted[..]);

		let mut outgoing_sorted = outgoing.to_vec();
		outgoing_sorted.sort();
		assert_eq!(outgoing, &outgoing_sorted[..]);

		// incoming and outgoing must be disjoint
		for x in incoming.iter() {
			assert!(outgoing.binary_search(x).is_err());
		}

		let mut old_plus_incoming = MEMBERS.with(|m| m.borrow().to_vec());
		old_plus_incoming.extend_from_slice(incoming);
		old_plus_incoming.sort();

		let mut new_plus_outgoing = new.to_vec();
		new_plus_outgoing.extend_from_slice(outgoing);
		new_plus_outgoing.sort();

		assert_eq!(
			old_plus_incoming, new_plus_outgoing,
			"change members call is incorrect!"
		);

		MEMBERS.with(|m| *m.borrow_mut() = new.to_vec());
		PRIME.with(|p| *p.borrow_mut() = None);
	}

	fn set_prime(who: Option<AccountId>) {
		PRIME.with(|p| *p.borrow_mut() = who);
	}
}

parameter_types! {
	pub const ElectionsPhragmenModuleId: LockIdentifier = *b"phrelect";
	pub const CandidacyBond: u64 = 3;
	pub const VotingBond: u64 = 2;
	pub const DesiredMembers: u32 = 2;
	pub const DesiredRunnersUp: u32 = 2;
	pub const TermDuration: u64 = 5;
	pub const VotingBondBase: u64 = 2;
	pub const VotingBondFactor: u64 = 0;
}

impl pallet_elections_phragmen::Config for Runtime {
	type ModuleId = ElectionsPhragmenModuleId;
	type Event = Event;
	type Currency = CurrencyAdapter<Runtime, GetTokenId>;
	type CurrencyToVote = SaturatingCurrencyToVote;
	type ChangeMembers = TestChangeMembers;
	type InitializeMembers = ();
	type CandidacyBond = CandidacyBond;
	type VotingBondBase = VotingBondBase;
	type VotingBondFactor = VotingBondFactor;
	type TermDuration = TermDuration;
	type DesiredMembers = DesiredMembers;
	type DesiredRunnersUp = DesiredRunnersUp;
	type LoserCandidate = ();
	type KickedMember = ();
	type WeightInfo = ();
}

parameter_type_with_key! {
	pub ExistentialDeposits: |currency_id: CurrencyId| -> Balance {
		match currency_id {
			&DNAR => 2,
			&SETT => 1 * 10_000,
			&JUSD => 1 * 1_000,
			_ => 0,
		}
	};
}

parameter_type_with_key! {
	pub GetBaseUnit: |currency_id: CurrencyId| -> Balance {
		match currency_id {
			&SETT => 10_000,
			&JUSD => 1_000,
			_ => 0,
		}
	};
}

const SERP_QUOTE_MULTIPLE: Balance = 2;
const SINGLE_UNIT: Balance = 1;
const SERPER_RATIO: Perbill = Perbill::from_percent(25);
const SETT_PAY_RATIO: Perbill = Perbill::from_percent(75);

parameter_types! {
	pub DustAccount: AccountId = ModuleId(*b"orml/dst").into_account();
}

parameter_types! {
	pub const GetSerperAcc: AccountId = SERPER;
	pub const GetSerpQuoteMultiple: Balance = SERP_QUOTE_MULTIPLE;
	pub const GetSettPayAcc: AccountId = SETTPAY;
	pub const GetSingleUnit: Balance = SINGLE_UNIT;
	pub const GetSerperRatio: Perbill = SERPER_RATIO;
	pub const GetSettPayRatio: Perbill = SETT_PAY_RATIO;
	pub const GetSerpNativeId: CurrencyId = DNAR;
	pub const GetSettId: CurrencyId = SETT;
	pub const GetJusdId: CurrencyId = JUSD;
}

impl Config for Runtime {
	type Event = Event;
	type Balance = Balance;
	type Amount = i64;
	type CurrencyId = CurrencyId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type GetBaseUnit = GetBaseUnit;
	type GetSerpNativeId = GetSerpNativeId;
	type GetSettId = GetSerpNativeId;
	type GetJusdId = GetSerpNativeId;
	type GetSerpQuoteMultiple = GetSerpQuoteMultiple;
	type GetSerperAcc = GetSerperAcc;
	type GetSettPayAcc = GetSettPayAcc;
	type GetSerperRatio = GetSerperRatio;
	type GetSettPayRatio = GetSettPayRatio;
	type GetSingleUnit = GetSingleUnit;
	type OnDust = TransferDust<Runtime, DustAccount>;
}
pub type TreasuryCurrencyAdapter = <Runtime as pallet_treasury::Config>::Currency;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
type Block = frame_system::mocking::MockBlock<Runtime>;

construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Module, Call, Storage, Config, Event<T>},
		Stp258Tokens: stp258_tokens::{Module, Storage, Event<T>, Config<T>},
		Treasury: pallet_treasury::{Module, Call, Storage, Config, Event<T>},
		ElectionsPhragmen: pallet_elections_phragmen::{Module, Call, Storage, Event<T>},
	}
);

pub struct ExtBuilder {
	endowed_accounts: Vec<(AccountId, CurrencyId, Balance)>,
	treasury_genesis: bool,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			endowed_accounts: vec![],
			treasury_genesis: false,
		}
	}
}

impl ExtBuilder {
	pub fn balances(mut self, endowed_accounts: Vec<(AccountId, CurrencyId, Balance)>) -> Self {
		self.endowed_accounts = endowed_accounts;
		self
	}

	pub fn one_hundred_for_alice_n_bob_n_serper_n_settpay(self) -> Self {
		self.balances(vec![
			(ALICE, DNAR, 100), 
			(BOB, DNAR, 100),
			(SERPER, DNAR, 100),
			(SETTPAY, DNAR, 100),
			(ALICE, SETT, 100 * 10_000), 
			(BOB, SETT, 100 * 10_000),
			(SERPER, SETT, 100 * 10_000),
			(SETTPAY, SETT, 100 * 10_000),
			(ALICE, JUSD, 100 * 1_000), 
			(BOB, JUSD, 100 * 1_000),
			(SERPER, JUSD, 100 * 1_000),
			(SETTPAY, JUSD, 100 * 1_000),
			])
	}

	pub fn one_hundred_for_treasury_account(mut self) -> Self {
		self.treasury_genesis = true;
		self.balances(vec![(TREASURY_ACCOUNT, DNAR, 100)])
	}

	pub fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default()
			.build_storage::<Runtime>()
			.unwrap();

		stp258_tokens::GenesisConfig::<Runtime> {
			endowed_accounts: self.endowed_accounts,
		}
		.assimilate_storage(&mut t)
		.unwrap();

		if self.treasury_genesis {
			pallet_treasury::GenesisConfig::default()
				.assimilate_storage::<Runtime, _>(&mut t)
				.unwrap();

			pallet_elections_phragmen::GenesisConfig::<Runtime> {
				members: vec![(TREASURY_ACCOUNT, 10)],
			}
			.assimilate_storage(&mut t)
			.unwrap();
		}

		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}
