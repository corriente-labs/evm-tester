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
+ resources/ + huff/ 
             + solidity/
             + bytecode/
+ artifacts/ + stateless/
             + stateful/
             + move/
```
- resouces: testcases written in `huff`, `solidity`, `bytecode`
- artifacts: folder generated
    - stateless: generated stateless testcases
    - stateful: generated stateful testcases
    - move: generated test move codes