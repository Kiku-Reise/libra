// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

mod driver;
use driver::*;

fn std_mvir(b: &str) -> String {
    format!("../../stdlib/modules/{}.mvir", b)
}

#[test]
fn test3() {
    test(&["test_mvir/test3.mvir"]);
}

#[test]
fn test_arithmetic() {
    test(&["test_mvir/test-arithmetic.mvir"]);
}

#[test]
fn test_control_flow() {
    test(&["test_mvir/test-control-flow.mvir"]);
}

#[test]
fn test_func_call() {
    test(&["test_mvir/test-func-call.mvir"]);
}

#[test]
fn test_reference() {
    test(&["test_mvir/test-reference.mvir"]);
}

#[test]
fn test_struct() {
    test(&["test_mvir/test-struct.mvir"]);
}

#[test]
fn test_lib() {
    test(&[
        &std_mvir("vector"),
        &std_mvir("u64_util"),
        &std_mvir("address_util"),
        &std_mvir("bytearray_util"),
        &std_mvir("hash"),
        &std_mvir("signature"),
        &std_mvir("gas_schedule"),
        &std_mvir("validator_config"),
        &std_mvir("libra_coin"),
        &std_mvir("libra_account"),
        // TODO(wrwg): this currently fails with boogie compilation errors
        //   call to undeclared procedure: Vector_contains (etc)
        // &std_mvir("libra_system"),
        "test_mvir/test-lib.mvir",
    ]);
}

#[test]
fn test_generics() {
    test(&[&std_mvir("vector"), "test_mvir/test-generics.mvir"]);
}

#[test]
fn test_specs() {
    test(&["test_mvir/test-specs.mvir"]);
}
