extern crate patternize;

use patternize::builder::*;

#[test]
fn simple_struct() {
    #[derive(Debug, PartialEq)]
    #[generate_builder]
    struct SimpleStruct {
        data: Box<std::string::String>,
    }

    let s = SimpleStructBuilder::new()
        .with_data(Box::new(String::from("simple_struct")))
        .build()
        .unwrap();

    assert_eq!(
        s,
        SimpleStruct {
            data: Box::new(String::from("simple_struct"))
        }
    );
}

#[test]
fn struct_with_generics() {
    #[derive(Debug, PartialEq)]
    #[generate_builder]
    struct GenericsStruct<T: std::fmt::Debug>
    where
        T: PartialEq,
    {
        data1: Box<std::string::String>,
        data2: T,
    }

    let s = GenericsStructBuilder::new()
        .with_data1(Box::new(String::from("struct with generics")))
        .with_data2(Box::new(String::from("struct with generics")))
        .build()
        .unwrap();

    assert_eq!(
        s,
        GenericsStruct {
            data1: Box::new(String::from("struct with generics")),
            data2: Box::new(String::from("struct with generics"))
        }
    )
}

#[test]
fn struct_with_traits() {
    #[derive(Debug, PartialEq)]
    #[generate_builder(Clone, Debug, PartialEq)]
    struct Struct<T: std::fmt::Debug, U: Clone>
    where
        T: Clone,
        U: std::fmt::Debug,
    {
        data1: T,
        data2: U,
    }

    let s = StructBuilder::new()
        .with_data1(std::collections::BTreeMap::<usize, usize>::new())
        .with_data2(std::collections::BTreeSet::<usize>::new());

    assert_eq!(s, s.clone());

    let s1 = s.clone().build().unwrap();
    let s2 = s.clone().build().unwrap();

    assert_eq!(s1, s2);
}
