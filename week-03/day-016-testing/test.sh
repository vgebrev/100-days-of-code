### disable parallelism
#cargo test -- --test-threads=1

### show stdout of all tests, including passing
#cargo test -- --show-output

### run specific tests
#cargo test it_works

### run only ignored tests
cargo test -- --ignored