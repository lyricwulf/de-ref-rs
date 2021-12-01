#![cfg(test)]

use de_ref::{Deref, DerefMut};

#[derive(Deref, DerefMut)]

struct DerefStructTest {
    _value: u32,
}
#[test]
fn test_deref_struct() {
    // Use Deref in test because macro usage should work without the `use`
    use std::ops::{Deref, DerefMut};
    let mut deref_struct = DerefStructTest { _value: 0 };
    assert_eq!(deref_struct.deref(), &0);
    assert_eq!(deref_struct.deref_mut(), &mut 0);
}

#[derive(Deref)]
struct DerefStructRefTest<'a> {
    _value: &'a u32,
}
#[test]
fn test_deref_struct_ref() {
    use std::ops::Deref;
    let deref_struct = DerefStructRefTest { _value: &0 };
    assert_eq!(deref_struct.deref(), &0);
}

#[derive(Deref, DerefMut)]
struct DerefStructRefMutTest<'a> {
    _value: &'a mut u32,
}
#[test]
fn test_deref_struct_ref_mut() {
    use std::ops::{Deref, DerefMut};
    let mut deref_struct = DerefStructRefMutTest { _value: &mut 0 };
    assert_eq!(deref_struct.deref(), &0);
    assert_eq!(deref_struct.deref_mut(), &mut 0);
}

#[derive(Deref, DerefMut)]
struct DerefGenericStructTest<T> {
    _value: T,
}
#[test]
fn test_deref_generic_struct() {
    use std::ops::{Deref, DerefMut};
    let mut deref_struct = DerefGenericStructTest { _value: 0 };
    assert_eq!(deref_struct.deref(), &0);
    assert_eq!(deref_struct.deref_mut(), &mut 0);
}

#[derive(Deref)]
struct DerefGenericStructRefTest<'a, T> {
    _value: &'a T,
}
#[test]
fn test_deref_generic_struct_ref() {
    use std::ops::Deref;
    let deref_struct = DerefGenericStructRefTest { _value: &0 };
    assert_eq!(deref_struct.deref(), &0);
}

#[derive(Deref, DerefMut)]
struct DerefGenericStructRefMutTest<'a, T> {
    _value: &'a mut T,
}
#[test]
fn test_deref_generic_struct_ref_mut() {
    use std::ops::{Deref, DerefMut};
    let mut deref_struct = DerefGenericStructRefMutTest { _value: &mut 0 };
    assert_eq!(deref_struct.deref(), &0);
    assert_eq!(deref_struct.deref_mut(), &mut 0);
}

#[derive(Deref, DerefMut)]
enum DerefEnumTest {
    _Variant(u32),
}
#[test]
fn test_deref_enum() {
    use std::ops::{Deref, DerefMut};
    let mut deref_enum = DerefEnumTest::_Variant(0);
    assert_eq!(deref_enum.deref(), &0);
    assert_eq!(deref_enum.deref_mut(), &mut 0);
}

#[derive(Deref)]
enum DerefEnumRefTest<'a> {
    _Variant(&'a u32),
}
#[test]
fn test_deref_enum_ref() {
    use std::ops::Deref;
    let deref_enum = DerefEnumRefTest::_Variant(&0);
    assert_eq!(deref_enum.deref(), &0);
}

#[derive(Deref, DerefMut)]
enum DerefEnumRefMutTest<'a> {
    _Variant(&'a mut u32),
}
#[test]
fn test_deref_enum_ref_mut() {
    use std::ops::{Deref, DerefMut};
    let mut inner = 0;
    let mut deref_enum = DerefEnumRefMutTest::_Variant(&mut inner);
    assert_eq!(deref_enum.deref(), &0);
    assert_eq!(deref_enum.deref_mut(), &mut 0);
}

#[derive(Deref, DerefMut)]
struct DerefTupleTest(u32);
#[test]
fn test_deref_tuple() {
    use std::ops::{Deref, DerefMut};
    let mut deref_tuple = DerefTupleTest(0);
    assert_eq!(deref_tuple.deref(), &0);
    assert_eq!(deref_tuple.deref_mut(), &mut 0);
}

#[derive(Deref)]
struct DerefTupleRefTest<'a>(&'a u32);
#[test]
fn test_deref_tuple_ref() {
    use std::ops::Deref;
    let deref_tuple = DerefTupleRefTest(&0);
    assert_eq!(deref_tuple.deref(), &0);
}

#[derive(Deref, DerefMut)]
struct DerefTupleRefMutTest<'a>(&'a mut u32);
#[test]
fn test_deref_tuple_ref_mut() {
    use std::ops::{Deref, DerefMut};
    let mut inner = 0;
    let mut deref_tuple = DerefTupleRefMutTest(&mut inner);
    assert_eq!(deref_tuple.deref(), &0);
    assert_eq!(deref_tuple.deref_mut(), &mut 0);
}
