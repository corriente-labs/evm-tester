use string_builder::Builder;

use crate::core::TestCase;

pub(crate) fn to_move_test(testcase: &TestCase) -> String {
    let mut b = Builder::default();

    b.append("    #[test(admin=@0xff, core_framework=@aptos_framework)]\n");
    b.append(format!("    public entry fun test_{}(admin: signer, core_framework: signer) {{\n", testcase.id));

    b.append("        let addr = signer::address_of(&admin);\n");
    b.append("        let vm_id = vm::init_test(&admin);\n");
    b.append("        let (burn_cap, mint_cap) = aptos_coin::initialize_for_test(&core_framework);\n");
    b.append("        aptos_account::create_account(addr);\n");
    b.append(format!("        coin::deposit(addr, coin::mint({:?}, &mint_cap));\n", testcase.value));
    b.append(format!("        assert!(coin::balance<AptosCoin>(addr) == {:?}, 0);\n", testcase.value));
    b.append(format!("        let code = x\"{}\";\n", hex::encode(&testcase.code)));
    b.append(format!("        let calldata = x\"{}\";\n", hex::encode(&testcase.calldata)));
    b.append("        let caller = 0xc000;\n");
    b.append("        let to = 0xc001;\n");
    b.append(format!("        let val = {};\n", testcase.value));
    b.append("        let output = vm::execute(vm_id, caller, to, val, &calldata, &code);\n");
    b.append(format!("        assert!(output == x\"{}\", 0);\n", hex::encode(&testcase.output)));
    b.append("        coin::destroy_mint_cap<AptosCoin>(mint_cap);\n");
    b.append("        coin::destroy_burn_cap<AptosCoin>(burn_cap);\n");

    b.append("    }\n\n");

    let s =  b.string().unwrap();
    s
}