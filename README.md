# Missing Value Error
This error was caused becase I was using different environments.
Thanks to @dmytro_stellar that showed me the way : https://discord.com/channels/897514728459468821/1162452486104293396/1162453048644337714

This error was fixed in https://github.com/paltalabs/missing-value-error/commit/615a2662aa0e87c4b22bf0aea5eb83aa3b63c4eb

## How to replicate
0.- Clone this project and go to a commit before the fix:

```bash
git clone https://github.com/paltalabs/missing-value-error
cd missing-value-error
git checkout bf7edf9d903ad4bf33af333330681cf72a826b47
```

1.- To run into a soroban-preview 11 docker container
```bash
bash run.sh
```
2.-bash
```
make build
make test
```

## What was breaking:
We are testing the following function:
```rust
fn repeat_address(address: Address) -> Address {
        address
    }
```
Like this:
```rust
#[test]
fn test_is_failing() {
    let e: Env = Default::default();
    e.mock_all_auths(); // TODO: can we test otherwise?
    
    let admin = Address::random(&e);
    let (token, _token_admin_client) = create_token_contract(&e, &admin);
    let test = MissingValueContractTest::setup();

   let repeat_address = test.contract.repeat_address(&token.address);

   assert_eq!(repeat_address, token.address);
    
}

```

## What was the error:
In the test we where setting one environment to create the token contract, and another environment to create the test.
Check the commit that fixed the test: https://github.com/paltalabs/missing-value-error/commit/615a2662aa0e87c4b22bf0aea5eb83aa3b63c4eb


## The error
```bash
---- test::test_is_failing stdout ----
thread 'test::test_is_failing' panicked at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/soroban-sdk-20.0.0-rc2/src/unwrap.rs:34:14:
called `Result::unwrap()` on an `Err` value: HostError: Error(Object, MissingValue)

Event log (newest first):
   0: [Diagnostic Event] topics:[error, Error(Object, MissingValue)], data:["unknown object reference", 219043332173]

Backtrace (newest first):
   0: soroban_env_host::host_object::<impl soroban_env_host::host::Host>::visit_obj_untyped
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/soroban-env-host-20.0.0-rc2/src/host_object.rs:379:17
   1: soroban_env_host::host_object::<impl soroban_env_host::host::Host>::check_obj_integrity
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/soroban-env-host-20.0.0-rc2/src/host_object.rs:404:9
   2: soroban_env_host::host_object::<impl soroban_env_host::host::Host>::check_val_integrity
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/soroban-env-host-20.0.0-rc2/src/host_object.rs:397:13
   3: <soroban_env_host::host::Host as soroban_env_common::env::EnvBase>::vec_new_from_slice
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/soroban-env-host-20.0.0-rc2/src/host.rs:1208:13
   4: <soroban_sdk::env::Env as soroban_env_common::env::EnvBase>::vec_new_from_slice
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/soroban-sdk-20.0.0-rc2/src/env.rs:1413:12
   5: soroban_sdk::vec::Vec<T>::from_array
             at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/soroban-sdk-20.0.0-rc2/src/vec.rs:382:19
      soroswap_library_contract::SoroswapLibraryClient::repeat_address
             at src/lib.rs:15:1
   6: soroswap_library_contract::test::test_is_failing
             at src/test.rs:77:25
   7: soroswap_library_contract::test::test_is_failing::{{closure}}
             at src/test.rs:69:22
   8: core::ops::function::FnOnce::call_once
             at /rustc/cc66ad468955717ab92600c770da8c1601a4ff33/library/core/src/ops/function.rs:250:5


note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


```
