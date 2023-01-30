use super::*;
use crate::person::{Person, PersonError};
use crate::policies::circuit_breaker::{CircuitBreakerPolicy, CircuitBreakerState};
use crate::policies::rate_limiter::{LimiterType, RateLimiter};
use crate::{
    failsafe::Failsafe,
    failsafe_error::FailsafeError,
    policies::Policy,
    policies::{fallback::FallbackPolicy, retry::RetryPolicy, timeout::TimeoutPolicy},
};
use std::thread::sleep;
use std::time::{Duration, Instant};

fn check_expected_error(r: Result<(), FailsafeError>, expected: &str) -> bool {
    match r {
        Ok(_) => false,
        Err(e) => {
            let s = format!("{:?}", e);
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
    let mut safe = failsafe!([RetryPolicy; [3, Duration::from_millis(50)]]);
    let mut person = Person::new();
    person.set_fail_pattern(vec![false, true, true, false]);
    let person_result = { safe.run(&mut person) };
    assert!(person_result.is_ok());
}

#[test]
fn retry_policy_on_fail() {
    let mut safe = failsafe!([RetryPolicy; [3, Duration::from_millis(50)]]);
    let mut person = Person::new();
    person.set_always_fail(true);
    let person_result = { safe.run(&mut person) };
    assert_eq!(
        person_result.expect_err("What error!").to_string(),
        FailsafeError::RetryError.to_string()
    );
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
    let mut safe = {
        let mut k = 0;
        let name_list = vec!["", "Picard", "Riker", "Data"];
        failsafe!([
            RetryPolicy; [1, Duration::from_millis(50)],
            FallbackPolicy; [on_fallback!({
                k += 1;
                if k >= name_list.len() {
                    k = 0
                }
                Person::with_name(name_list[k])
            })],
            RetryPolicy; [3, Duration::from_millis(50)]
        ])
    };
    let mut person = Person::new();
    person.set_always_fail(true);
    let _ = { safe.run(&mut person) };
    assert_eq!("Picard", person.name());
    let _ = { safe.run(&mut person) };
    assert_eq!("Riker", person.name());
    let _ = { safe.run(&mut person) };
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

#[test]
fn timeout_policy_test() {
    let mut safe = failsafe!([TimeoutPolicy; [Duration::from_millis(1000)]]);
    let mut person = Person::new();
    let person_result = { safe.run(&mut person) };
    assert!(person_result.is_ok());

    person.set_wait_for(Duration::from_millis(2100));
    let person_result = { safe.run(&mut person) };
    assert!(person_result.is_err());
    check_expected_error(person_result, "TimeoutError");

    person.set_wait_for(Duration::from_millis(100));
    person.set_always_fail(true);
    let person_result = { safe.run(&mut person) };
    assert!(person_result.is_err());
    if let Err(FailsafeError::RunnableError(_e)) = person_result {
        assert_eq!(&PersonError::NameFindingError, PersonError::from_any(&_e));
    }
}

#[test]
fn circuit_breaker_impl() {
    let mut person = Person::new();
    person.set_fail_pattern(vec![
        false, // check if normal run possible
        true, true, true, true, true, false, // now reach the error threshold
        false, false, // this should be circuit breaker open error
        false, false, false, // now let's check if circuit breaker closes on success
        false, // now it should be all open
    ]);

    let mut policy = CircuitBreakerPolicy::new(5, Duration::from_millis(20), 2);
    let mut policy_errors = vec![];
    // check if normal run possible
    assert!(policy
        .run(&mut Box::new(&mut person), &mut policy_errors)
        .is_ok());
    assert_eq!(policy_errors.len(), 0);
    policy_errors.clear();
    println!("Error runs ...");
    for _ in 0..=5 {
        let e = policy.run(&mut Box::new(&mut person), &mut policy_errors);
        if let Err(FailsafeError::RunnableError(_e)) = e {
            assert_eq!(&PersonError::NameFindingError, PersonError::from_any(&_e));
        }
    }
    println!("> {:?}", policy_errors);
    policy_errors.clear();
    assert_eq!(policy.circuit_breaker_state(), &CircuitBreakerState::Open);
    for _ in 0..=2 {
        println!("Running ...");
        let r = policy.run(&mut Box::new(&mut person), &mut policy_errors);
        if let Err(FailsafeError::CircuitBreakerOpen) = r {
            continue;
        }
        assert!(false)
    }
    sleep(Duration::from_millis(22));
    for _ in 0..1 {
        assert!(policy
            .run(&mut Box::new(&mut person), &mut policy_errors)
            .is_ok());
        assert_eq!(
            policy.circuit_breaker_state(),
            &CircuitBreakerState::HalfOpen
        );
    }
    assert!(policy
        .run(&mut Box::new(&mut person), &mut policy_errors)
        .is_ok());
    assert_eq!(policy.circuit_breaker_state(), &CircuitBreakerState::Closed);
    person.set_fail_pattern(vec![]);
    person.set_always_fail(true);
    for _ in 0..=5 {
        let e = policy.run(&mut Box::new(&mut person), &mut policy_errors);
        if let Err(FailsafeError::RunnableError(_e)) = e {
            assert_eq!(&PersonError::NameFindingError, PersonError::from_any(&_e));
        }
    }
    println!("> {:?}", policy_errors);
    policy_errors.clear();
    if let Err(FailsafeError::CircuitBreakerOpen) =
        policy.run(&mut Box::new(&mut person), &mut policy_errors)
    {
        assert!(true);
    } else {
        assert!(false);
    }
    sleep(Duration::from_millis(22));
    person.set_fail_pattern(vec![false, true]);
    assert!(policy
        .run(&mut Box::new(&mut person), &mut policy_errors)
        .is_ok());
    assert!(policy
        .run(&mut Box::new(&mut person), &mut policy_errors)
        .is_err());
    assert_eq!(policy.circuit_breaker_state(), &CircuitBreakerState::Open);
    println!("{:?}", policy_errors);
}

#[test]
fn rate_limiter_impl() {
    let mut policy = RateLimiter::new(LimiterType::Smooth, 100, Duration::from_secs(1));
    let mut p = Person::new();
    let mut policy_errors = vec![];
    let start = Instant::now();
    for _ in 0..200 {
        match policy.run(&mut Box::new(&mut p), &mut policy_errors) {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}
