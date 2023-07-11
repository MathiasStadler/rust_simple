
# rust_simple_math

## new project simple error

```bash

mkdir rust_simple_math
cd $_
touch COMMAND_PATH_Of_PROJECT.md
```

## init project

```bash
pwd # just for control
cargo init --bin  .  # with bin  folder
mkdir src/lib.rs # create lib.rs
```

## create example folder

```bash
mkdir  ${PWD}/examples
cd $_
```

## create new examples

```bash
touch simple_math_addition.rs
touch simple_math_subtraction
```

## run examples

```bash
cargo run  --example simple_math_addition
cargo run  --example simple_math_subtraction
```

## run test

```bash
cargo test  --example simple_math_addition
```

## add criterion lib in Cargo.toml dev-dependencies section

```bash
cd ~/rust_simple # project root
cargo add --dev criterion
cargo build
```

## add  lib to test

```bash
touch  ~/rust_simple/src/lib.rs

tee -a  ~/rust_simple/src/lib.rs << END
//need for testing criterion
#[inline]
pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}
END

```

## add first demo test

```bash
tee -a  ~/rust_simple/benches/test_criterion.rs << END
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_simple::fibonacci;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
END
```





mkdir benches
trapapa@trapapa-ThinkPad-T430:~/rust_simple$ touch benches/my_benchmark.rs
