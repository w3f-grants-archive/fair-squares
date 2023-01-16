use crate::{
	mock::*, 
	types::{PaymentDetail, PaymentState},
	Payment as PaymentStore, PaymentHandler, ScheduledTask, ScheduledTasks, Task,
};
use frame_support::{assert_noop, assert_ok, storage::with_transaction};
use sp_runtime::{Percent, TransactionOutcome};
type Error = crate::Error<Test>;

fn last_event() -> Event {
	System::events().pop().expect("Event expected").event
}

#[test]
fn test_pay_works() {
	new_test_ext().execute_with(|| {
		let creator_initial_balance = 100_000_000_000;
		let payment_amount = 2000 as u64;
		let expected_incentive_amount = payment_amount / INCENTIVE_PERCENTAGE as u64;

		// the payment amount should not be reserved
		assert_eq!(
			Balances::free_balance(&PAYMENT_CREATOR),
			creator_initial_balance
		);
		assert_eq!(Balances::free_balance(&PAYMENT_RECIPENT), 1);

		// should be able to create a payment with available balance
		assert_ok!(PaymentModule::pay(
			Origin::signed(PAYMENT_CREATOR),
			PAYMENT_RECIPENT,
			payment_amount,
			None
		));
		assert_eq!(
			last_event(),
			crate::Event::<Test>::PaymentCreated {
				from: PAYMENT_CREATOR,
				amount: payment_amount,
				remark: None
			}
			.into()
		);

		assert_eq!(
			PaymentStore::<Test>::get(PAYMENT_CREATOR, PAYMENT_RECIPENT),
			Some(PaymentDetail {
				amount: payment_amount,
				incentive_amount: expected_incentive_amount,
				state: PaymentState::Created,
				resolver_account: RESOLVER_ACCOUNT,
				fee_detail: Some((FEE_RECIPIENT_ACCOUNT, 0)),
			})
		);
		// the payment amount should be reserved correctly
		// the amount + incentive should be removed from the sender account
		assert_eq!(
			Balances::free_balance( &PAYMENT_CREATOR),
			creator_initial_balance - payment_amount - expected_incentive_amount
		);
		// the incentive amount should be reserved in the sender account
		assert_eq!(
			Balances::free_balance(&PAYMENT_CREATOR).saturating_add(Balances::reserved_balance(&PAYMENT_CREATOR)),
			creator_initial_balance - payment_amount
		);
		assert_eq!(Balances::free_balance(&PAYMENT_RECIPENT), 1);
		// the transferred amount should be reserved in the recipent account
		assert_eq!(Balances::free_balance(&PAYMENT_RECIPENT).saturating_add(Balances::reserved_balance(&PAYMENT_RECIPENT)), payment_amount.saturating_add(Balances::free_balance(&PAYMENT_RECIPENT)));

		// the payment should not be overwritten
		assert_noop!(
			PaymentModule::pay(
				Origin::signed(PAYMENT_CREATOR),
				PAYMENT_RECIPENT,
				payment_amount,
				None
			),
			crate::Error::<Test>::PaymentAlreadyInProcess
		);

		assert_eq!(
			PaymentStore::<Test>::get(PAYMENT_CREATOR, PAYMENT_RECIPENT),
			Some(PaymentDetail {
				amount: payment_amount,
				incentive_amount: 200,
				state: PaymentState::Created,
				resolver_account: RESOLVER_ACCOUNT,
				fee_detail: Some((FEE_RECIPIENT_ACCOUNT, 0)),
			})
		);
	});
}
