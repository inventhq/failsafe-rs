# Failsafe

Failsafe is a lightweight rust library for handling failures.

## How to use

A failsafe client must implement the Runnable trait
```rust
#[derive(Clone, Debug, PartialEq)]
pub struct Person {
    name: Option<String>,
}

impl Runnable for Person {
    fn run(&mut self) -> Result<(), Box<dyn Any>> {
        println!("I am a person, getting my name!");
        let name_response: Result<String, SomeNetworkError> = remote_request_that_might_fail();
        match name_response {
            Ok(name) => {
                println!("Got a name! {}", name);
                self.name = Some(name);
            }
            Err(_) => return Err(
                Box::new(
                    PersonError::as_any(&PersonError::NameFindingError)
                )
            )
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any { self }

    fn update(&mut self, other: &Box<dyn FallbackAble>) {
        let n: &Person = other.as_any().downcast_ref().unwrap();
        self.name = Some(n.name().clone());
    }
}
```

Client that will use FallbackPolicy, must implement FallbackAble Trait
```rust
impl FallbackAble for Person {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

```

There's two way we can create Failsafe Object composing of multiple policies

## Using Macro
```rust
fn using_macro() -> FailSafe {
    p!(
        [
            // if failed, retry 3 times, after waiting for 50 milliseconds
            RetryPolicy; [3, Duration::from_millis(50)],
            // if retry attempt fails, returns a default value
            FallbackPolicy;
            [on_fallback!({
                Person::new(
                    Some("No Name".to_string())
                )
            })]
        ]
    )
}
```

## Using Builder
```rust
fn using_builder() -> FailSafe {
    Failsafe::builder()
        .push(FallbackPolicy::new(on_fallback!({
            Person::with_name("No Name")
        })))
        .push(RetryPolicy::new(3, Duration::from_millis(50)))
        .build()
}

```

Once the failsafe object is created, we can pass any Runnable client and run it

```rust
fn main() {
    let mut safe = using_macro();
    let mut person = Person::new();
    let person_result: Result<(), FailsafeError> = safe.run(&mut person);

    let mut another = AnotherClient::new();
    let another_result: Result<(), FailsafeError> = safe.run(&mut another);
}
```




