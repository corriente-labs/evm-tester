use string_builder::Builder;

use crate::core::{NormalizedAccount, TestCase};

pub(crate) fn to_move_test(testcase: &TestCase) -> String {
    let mut b = Builder::default();

    b.append("    #[test(admin=@0xff, core_framework=@aptos_framework)]\n");
    b.append(format!(
        "    public entry fun test_{}(admin: signer, core_framework: signer) {{\n",
        testcase.id
    ));

    b.append("        let addr = signer::address_of(&admin);\n");
    b.append("        let vm_id = vm::init_test(&admin);\n");
    b.append(
        "        let (burn_cap, mint_cap) = aptos_coin::initialize_for_test(&core_framework);\n",
    );
    b.append("        aptos_account::create_account(addr);\n");
    b.append(format!(
        "        coin::deposit(addr, coin::mint({:?}, &mint_cap));\n",
        testcase.value
    ));
    b.append(format!(
        "        assert!(coin::balance<AptosCoin>(addr) == {:?}, 0);\n",
        testcase.value
    ));
    b.append(format!(
        "        let code = x\"{}\";\n",
        hex::encode(&testcase.code)
    ));
    b.append(format!(
        "        let calldata = x\"{}\";\n\n",
        hex::encode(&testcase.calldata)
    ));

    deploy_account(&mut b, &testcase.accounts_input);

    b.append("\n        let caller = 0xc000;\n");
    b.append("        let to = 0xc001;\n");
    b.append(format!("        let val = {};\n", testcase.value));
    b.append("        let output = vm::execute(vm_id, caller, to, val, &calldata, &code);\n");
    b.append(format!(
        "        assert!(output == x\"{}\", 0);\n\n",
        hex::encode(&testcase.output)
    ));

    assert_accounts_output(&mut b, &testcase.accounts_output);

    b.append("\n        coin::destroy_mint_cap<AptosCoin>(mint_cap);\n");
    b.append("        coin::destroy_burn_cap<AptosCoin>(burn_cap);\n");

    b.append("    }\n\n");

    let s = b.string().unwrap();
    s
}

fn deploy_account(b: &mut Builder, accounts: &[NormalizedAccount]) {
    for acct in accounts {
        let address = acct.address.as_fixed_bytes();
        let address = hex::encode(address);
        let code = hex::encode(&acct.code);
        b.append(format!(
            "        vm::deploy_account(vm_id, x\"{}\", {:?}, x\"{}\", {:?});\n",
            &address, acct.balance, &code, acct.nonce
        ));
    }
}

fn assert_accounts_output(b: &mut Builder, accounts: &[NormalizedAccount]) {
    for acct in accounts {
        let nonce = acct.nonce.as_u128();
        let balance = acct.balance.as_u128();
        let code = hex::encode(&acct.code);
        let address = acct.address.as_fixed_bytes();
        let address = hex::encode(address);
        b.append(format!(
            "        let nonce = vm::nonce(vm_id, x\"{}\");\n",
            address
        ));
        b.append(format!("        assert!(nonce == {:?}, 0);\n", nonce));
        b.append(format!(
            "        let balance = vm::balance(vm_id, x\"{}\");\n",
            address
        ));
        b.append(format!("        assert!(balance == {:?}, 0);\n", balance));
        b.append(format!(
            "        let code = vm::code(vm_id, x\"{}\");\n",
            address
        ));
        b.append(format!("        assert!(code == x\"{}\", 0);\n", code));

        for (key, value) in &acct.storage {
            let key = key.as_fixed_bytes();
            let key = hex::encode(key);
            let val = value.as_fixed_bytes();
            let val = hex::encode(val);
            b.append(format!(
                "        let value = vm::storage(vm_id, x\"{}\", x\"{}\");\n",
                address, key
            ));
            b.append(format!("        assert!(value == x\"{}\", 0);\n", val));
        }
    }
}
