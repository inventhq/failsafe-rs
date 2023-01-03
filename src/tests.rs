use std::thread::sleep;
use std::time::Duration;
use rand::distributions::{Alphanumeric, DistString};
use rand::{random, thread_rng};
use crate::{
    failsafe_error::FailsafeError,
    failsafe::Failsafe,
    policies::{
        fallback::FallbackPolicy,
        retry::RetryPolicy,
        timeout::TimeoutPolicy,
    },
    run_state::PolicyActionState,
    policies::Policy,
};
use crate::person::{Person, PersonError};
use super::*;

fn check_expected_error(r: Result<(), FailsafeError>, expected: &str) -> bool {
    match r {
        Ok(_) => {
            false
        }
        Err(e) => {
            let s = format!("{:?}", e);
            println!("[{}]", s);
            &s == expected
        }
    }
}

#[test]
fn test_policy_flow() {
    let safe: Failsafe = Failsafe::builder()
        .push(FallbackPolicy::new(Box::new(|| Box::new(Person::new(Some("No Name".to_string()))))))
        .push(RetryPolicy::new(3, Duration::from_millis(50)))
        .build();
    assert_eq!(safe.policy().name(), "FallbackPolicy");
    let mut p = safe.policy().as_ref().clone();
    let k = p.inner().as_ref().unwrap().name();
    assert_eq!(
        &k,
        "RetryPolicy"
    );
}

#[test]
fn test_fallback() {
    let mut safe: Failsafe = Failsafe::builder()
        .push(FallbackPolicy::new(Box::new(|| Box::new(Person::new(Some("No Name".to_string()))))))
        .build();
    let mut person = Person::new(None);
    person.set_always_fail(true);
    let person_result = { safe.run(&mut person) };
    assert!(check_expected_error(person_result, "UsedFallback"));
}

#[test]
fn test_retry_policy_with_always_failing() {
    let mut safe: Failsafe = Failsafe::builder()
        .push(RetryPolicy::new(3, Duration::from_millis(50)))
        .build();
    let mut person = Person::new(None);
    person.set_always_fail(true);
    let person_result = { safe.run(&mut person) };
    assert!(check_expected_error(person_result, "RetryError"));
}

#[test]
fn test_retry_policy_working_after_few_retries() {
    let mut safe: Failsafe = Failsafe::builder()
        .push(FallbackPolicy::new(Box::new(|| Box::new(Person::new(Some("No Name".to_string()))))))
        .push(RetryPolicy::new(3, Duration::from_millis(50)))
        .build();
    let mut person = Person::new(None);
    person.set_fail_pattern(vec![false, true, true]);
    let person_result = { safe.run(&mut person) };
    assert!(person_result.is_ok());
}

#[test]
fn test_if_retry_policy_multiple_run_correctly_reset() {
    let mut safe: Failsafe = Failsafe::builder()
        .push(RetryPolicy::new(3, Duration::from_millis(50)))
        .build();
    let mut person = Person::new(None);
    person.set_fail_pattern(vec![false, true, true]);
    let person_result = { safe.run(&mut person) };
    assert!(person_result.is_ok());
    // should be same
    let mut person = Person::new(None);
    person.set_fail_pattern(vec![false, true, true]);
    let person_result = { safe.run(&mut person) };
    assert!(person_result.is_ok());
}

#[test]
fn test_using_different_value_from_fallback() {
    let mut k = 0;
    let name_list = vec!["", "Picard", "Riker", "Data"];
    let mut safe: Failsafe = Failsafe::builder()
        .push(RetryPolicy::new(1, Duration::from_millis(50)))
        .push(FallbackPolicy::new(Box::new(move || {
            k += 1;
            if k >= name_list.len() { k = 0 }
            Box::new(Person::new(Some(name_list[k].to_string())))
        })))
        .push(RetryPolicy::new(3, Duration::from_millis(50)))
        .build();
    let mut person = Person::new(None);
    person.set_always_fail(true);
    let person_result = { safe.run(&mut person) };
    assert_eq!("Picard", person.name());
    let person_result = { safe.run(&mut person) };
    assert_eq!("Riker", person.name());
    let person_result = { safe.run(&mut person) };
    assert_eq!("Data", person.name());
}
