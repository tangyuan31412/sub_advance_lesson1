use super::*;
use crate::{mock::*,Error};
use frame_support::{assert_noop,assert_ok,BoundedVec};
use crate::Proofs;

#[test]
fn create_claim_works() {
	new_test_ext().execute_with( ||{

		let claim =vec![0, 1];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

		let bounded_claim = BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();
		assert_eq!(
			Proofs::<Test>::get(&bounded_claim),
			Some((1, frame_system::Pallet::<Test>::block_number()))
		)
	});
}

#[test]
fn create_claim_failed_when_claim_is_too_long() {
	new_test_ext().execute_with(|| {
		let claim = vec![0; 513];

        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim),
            Error::<Test>::ClaimTooLong
        );
    })
}


#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim));
    })
}



#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

		let bounded_claim = BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();
        // assert_ok!(PoeModule::transfer_claim(Origin::signed(1), 2, claim.clone()));
        // assert_eq!(Proofs::<Test>::get(&claim), Some((2, frame_system::Pallet::<Test>::block_number())));
		assert_noop!(
            PoeModule::transfer_claim(Origin::signed(2), 666, claim.clone()),
            Error::<Test>::NotClaimOwner
        );
    })
}


