use std::thread::sleep;
use std::time::Duration;
use crate::{
    failsafe_error::FailsafeError,
    failsafe::Failsafe,
    policies::{
        fallback::Fallback,
        retry::RetryPolicy,
        timeout::TimeoutPolicy,
    },
    run_state::RunState,
    policies::fallback::FallbackAble,
    policies::Policy,
};
use super::*;

struct Person {
    name: Option<String>,
    default: String,
}

impl Person {
    fn new(def: &str) -> Self {
        Person {
            name: None,
            default: def.to_string(),
        }
    }
}

impl Runnable for Person {
    fn run(&mut self) {
        println!("I am a person, getting my name!");
        sleep(Duration::from_millis(300));
        println!("Done walking!");
    }
}

impl FallbackAble for Person {
    fn default(&mut self) {
        self.name = Some("Dave".to_string());
    }
}

#[test]
fn running_under_policies() {
    let mut safe: Failsafe<Person> = Failsafe::builder()
        .push(Fallback::new(Person::new("Dave")))
        .push(RetryPolicy::new(3, Duration::from_millis(50)))
        .push(TimeoutPolicy::new(Duration::from_secs(10)))
        .build();
    safe.run();
    // The above works, need a test


    // will work as expected
    // safe.call(a_function_to_fail, (false, None)).expect("TODO: panic message");
    // assert_eq!(safe.state(), RunState::Stable);
    // TODO: Make the above work


    // safe.get_result(a_function_to_fail, true, Some(true));
    // assert_eq!(safe.state(), RunState::UsingFallback);

    // // fail for the first time
    // // as par policy, once the failure occurs we go through following stages
    // // Execution -> Timeout Error -> Retry -> Try again
    // // Execution -> Timeout Error -> Retry -> Try again
    // // Execution -> Timeout Error -> Retry -> Try again
    // // Execution -> Timeout Error -> Retry -> Retry Error -> Fallback -> returns 0

    // safe.get_result(a_function_to_fail, true, None);
    // assert_eq!(safe.state(), RunState::UsingFallback);
    // assert_eq!(safe.fail_point(), FailsafeError::TimeoutError);
    // // will work
    // safe.get_result(a_function_to_fail, true, true);
    // assert_eq!(safe.state(), RunState::Stable);
}
