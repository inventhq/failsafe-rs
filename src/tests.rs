use super::*;
use crate::person::{Person, PersonError};
use crate::{
    failsafe::Failsafe,
    failsafe_error::FailsafeError,
    policies::Policy,
    policies::{fallback::FallbackPolicy, retry::RetryPolicy, timeout::TimeoutPolicy},
    run_state::PolicyActionState,
};
use std::time::Duration;

fn check_expected_error(r: Result<(), FailsafeError>, expected: &str) -> bool {
    match r {
        Ok(_) => false,
        Err(e) => {
            let s = format!("{:?}", e);
            println!("[{}]", s);
            &s == expected
        }
    }
}

#[test]
fn fallback_callback() {
    let mut k = 0;
    let mut safe = failsafe!([
        RetryPolicy; [1, Duration::from_millis(50)],
        FallbackPolicy; [on_fallback!({
            k += 1;
            let s = format!("Person {}", k);
            let mut p = Person::new();
            p.set_name(&s);
            p
        })],
        RetryPolicy; [3, Duration::from_millis(50)]
    ]);
    let mut person = Person::new();
    person.set_always_fail(true);
    let person_result = { safe.run(&mut person) };
    assert_eq!("Person 1", person.name());
    let person_result = { safe.run(&mut person) };
    assert_eq!("Person 2", person.name());
    let person_result = { safe.run(&mut person) };
    assert_eq!("Person 3", person.name());
}

#[test]
fn test_fallback() {
    let mut safe = failsafe!([FallbackPolicy; [on_fallback!({ Person::with_name("No Name") })]]);
    let mut person = Person::new();
    person.set_always_fail(true);
    let person_result = { safe.run(&mut person) };
    assert!(check_expected_error(person_result, "UsedFallback"));
}

#[test]
fn test_retry_policy_with_always_failing() {
    let mut safe = failsafe!([RetryPolicy; [3, Duration::from_millis(50)]]);
    let mut person = Person::new();
    person.set_always_fail(true);
    let person_result = { safe.run(&mut person) };
    assert!(check_expected_error(person_result, "RetryError"));
}

#[test]
fn test_retry_policy_working_after_few_retries() {
    let mut safe = failsafe!([
        FallbackPolicy; [on_fallback!({Person::with_name("No Name")})],
        RetryPolicy; [3, Duration::from_millis(50)]
    ]);
    let mut person = Person::new();
    person.set_fail_pattern(vec![false, true, true]);
    let person_result = { safe.run(&mut person) };
    assert!(person_result.is_ok());
}

#[test]
fn test_if_retry_policy_multiple_run_correctly_reset() {
    let mut safe = failsafe!([RetryPolicy; [3, Duration::from_millis(50)]]);
    let mut person = Person::new();
    person.set_fail_pattern(vec![false, true, true]);
    let person_result = { safe.run(&mut person) };
    assert!(person_result.is_ok());
    // should be same
    let mut person = Person::new();
    person.set_fail_pattern(vec![false, true, true]);
    let person_result = { safe.run(&mut person) };
    assert!(person_result.is_ok());
}

#[test]
fn test_using_different_value_from_fallback() {
    let mut k = 0;
    let name_list = vec!["", "Picard", "Riker", "Data"];
    let mut safe = failsafe!([
        RetryPolicy; [1, Duration::from_millis(50)],
        FallbackPolicy; [on_fallback!({
            k += 1;
            if k >= name_list.len() {
                k = 0
            }
            Person::with_name(name_list[k])
        })],
        RetryPolicy; [3, Duration::from_millis(50)]
    ]);
    let mut person = Person::new();
    person.set_always_fail(true);
    let person_result = { safe.run(&mut person) };
    assert_eq!("Picard", person.name());
    let person_result = { safe.run(&mut person) };
    assert_eq!("Riker", person.name());
    let person_result = { safe.run(&mut person) };
    assert_eq!("Data", person.name());
}

#[test]
fn failsafe_builder_marco() {
    let safe = failsafe!([
        FallbackPolicy; [on_fallback!({
            Person::with_name("No Name")
        })],
        RetryPolicy; [3, Duration::from_millis(50)]
    ]);
    assert_eq!(safe.policy().name(), "FallbackPolicy");
    let mut p = safe.policy().as_ref().clone();
    let k = p.inner().as_ref().unwrap().name();
    assert_eq!(&k, "RetryPolicy");
}

#[test]
fn failsafe_builder() {
    let safe = Failsafe::builder()
        .push(FallbackPolicy::new(on_fallback!({
            Person::with_name("No Name")
        })))
        .push(RetryPolicy::new(3, Duration::from_millis(50)))
        .build();
    assert_eq!(safe.policy().name(), "FallbackPolicy");
    let mut p = safe.policy().as_ref().clone();
    let k = p.inner().as_ref().unwrap().name();
    assert_eq!(&k, "RetryPolicy");
}
