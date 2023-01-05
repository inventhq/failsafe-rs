# Failsafe

Failsafe is a lightweight rust library for handling failures.

## How to use

A failsafe client must implement the `Runnable` trait
```rust
#[derive(Clone, Debug, PartialEq)]
pub struct Person {
    name: Option<String>,
    pub url: String,
}

impl Runnable for Person {
    fn run(&mut self) -> Result<(), Box<dyn Any>> {
        println!("I am a person, getting my name!");
        let name_response: Result<String, SomeNetworkError> = remote_request_that_might_fail(self.url);
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

    fn update(&mut self, other: &Box<dyn FallbackAble>) {
        let n: &Person = other.as_any().downcast_ref().unwrap();
        self.url = n.url;
    }
}
```

Client that will use `FallbackPolicy`, must implement `FallbackAble` Trait
```rust
impl FallbackAble for Person {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

```

There's two way we can create `Failsafe` Object composing of multiple policies

## Using Macro
```rust
fn using_macro() -> FailSafe {
    let mut k = 0;
    let url_list = vec!["", "google.com", "bing.com", "duckduckgo.com"];
    failsafe!([
        RetryPolicy; [1, Duration::from_millis(50)],
        FallbackPolicy; [on_fallback!({
            k += 1;
            if k >= url_list.len() {
                k = 0
            }
            Person::with_url(url_list[k])
        })],
        RetryPolicy; [3, Duration::from_millis(50)]
    ])
}
```

## Using Builder
```rust
fn using_builder() -> FailSafe {
    let mut k = 0;
    let url_list = vec!["", "google.com", "bing.com", "duckduckgo.com"];
    Failsafe::builder()
        .push(RetryPolicy::new(1, Duration::from_millis(50)))
        .push(FallbackPolicy::new(on_fallback!({
            k += 1;
            if k >= url_list.len() {
                k = 0
            }
            Person::with_url(url_list[k])
        })))
        .push(RetryPolicy::new(3, Duration::from_millis(50)))
        .build()
}
```

Once the `FailSafe` object is created, we can pass any `Runnable` client and run it. 


In this following example, using the above policy set, the process is as followed:

1. Failsafe runs the error-prone client,
2. The client fails, and returns an error
3. Retry policy waits for 50 ms, and tries again
4. In case the client success, retry policy resets and returns Ok
5. In case the client fails, retry policy tries 2 more times
6. If all the attempts are failed, retry policy hands the runnable to FallbackPolicy
7. FallbackPolicy, assigns a fallback url, and hands the runnable to next policy, which is another RetryPolicy
8. RetryPolicy starts the execution process from the beginning, with a new url

```rust
fn main() {
    let mut safe = using_macro();
    let mut person = Person::new();
    let person_result: Result<(), FailsafeError> = safe.run(&mut person);

    let mut another = AnotherClient::new();
    let another_result: Result<(), FailsafeError> = safe.run(&mut another);
}
```


