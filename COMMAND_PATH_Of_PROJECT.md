
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
