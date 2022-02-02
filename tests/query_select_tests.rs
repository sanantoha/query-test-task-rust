use common::do_test;

mod common;

#[test]
fn test_case_0() -> Result<(), Box<dyn std::error::Error>> {
    do_test("case-0", 0)
}

#[test]
fn test_case_1() -> Result<(), Box<dyn std::error::Error>> {
    do_test("case-1", 1)
}

#[test]
fn test_case_2() -> Result<(), Box<dyn std::error::Error>> {
    do_test("case-2", 2)
}

#[test]
fn test_case_3() -> Result<(), Box<dyn std::error::Error>> {
    do_test("case-3", 3)
}