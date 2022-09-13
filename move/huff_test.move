#[test_only]
module pocvm::huff_tests {
  use std::signer;
  use std::unit_test;
  use std::vector;
  use aptos_framework::coin;
  use aptos_framework::aptos_coin::{Self, AptosCoin};
  use aptos_framework::aptos_account;
  use pocvm::vm;

#[test(admin=@0xff, core_framework=@aptos_framework)]
public entry fun test_"1"(user: admin, core_framework: signer) {
  let addr = signer::address_of(&admin);
  let (burn_cap, mint_cap) = aptos_coin::initialize_for_test(&core_framework);
  aptos_account::create_account(addr);
  coin::deposit(addr, coin::mint(1000000, &mint_cap));
  assert!(coin::balance<AptosCoin>(addr) == 1000000, 0);
  let code = x"6000356010350160005260106000f3";
  let calldata = x"0000000000000000000000000000000100000000000000000000000000000002";
  let caller = 0xc000;
  let to = 0xc001;
  let output = execute(vm_id, caller, to, val, &calldata, &code);
  assert!(output == x"00000000000000000000000000000003");
}}

