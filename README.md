# EVM tests
## Huff
EVM bytecodes generated from Huff lang.
https://docs.huff.sh/
```
huffc ./huff/addition.huff --bin-runtime
```

# Gen Tests
```
cargo run
```

# directory
```
+ resources/ + testgroup_0/ + test_0/        + test_0.huff
             |              |                + state.json
             |              + stateless_test_1.huff
             |              + stateless_test_2.bytecode
             |              + testcase.json
             + testgroup_1/ + test_1/        + test_1.huff
             |              |                + state.json
             |              + stateless_test_3.bytecode
             |              + testcase.json
+ artifacts/ + move/        + testgroup_0.move
             |              + testgroup_1.move
             + json/        + testgroup_1/   + test_0.json
                            |                + stateless_test_1.json
                            |                + stateless_test_2.json
                            + testgroup_2/   + test_1.json
                                             + stateless_test_2.json
```
- each testgroup becomes one move file.
- each testgroup becomes one json folder.