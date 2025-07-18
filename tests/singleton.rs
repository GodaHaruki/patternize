extern crate patternize;

use std::{any::Any, sync::OnceLock};

use patternize::singleton::*;

#[test]
fn simple_struct() {
    #[generate_singleton]
    struct SimpleStruct {
        _data1: Box<String>,
        _data2: Box<String>,
    }

    assert_eq!(
        SIMPLE_STRUCT.type_id(),
        <OnceLock<SimpleStruct> as Any>::type_id(&OnceLock::new())
    );
}

#[test]
fn simple_enum() {
    #[generate_singleton]
    enum SimpleEnum {
        #[allow(dead_code)]
        Type1,
        #[allow(dead_code)]
        Type2(usize),
    }

    assert_eq!(
        SIMPLE_ENUM.type_id(),
        <OnceLock<SimpleEnum> as Any>::type_id(&OnceLock::new())
    );
}

#[test]
fn singleton_with_expr() {
    #[derive(Debug, PartialEq)]
    #[generate_singleton({ 
        let _do_nothing = 0 + 0;
        WithExprStruct { data: "string".to_string() }
    })]
    struct WithExprStruct {
        data: String,
    }

    assert_eq!(
        &*WITH_EXPR_STRUCT,
        &WithExprStruct {
            data: "string".to_string()
        }
    );

    // TODO:
    // NOTE: may fail because closure's type-id are not same before compiler substitute them for the same one
    // I don't know how to check that they are same type appropriately
    // assert_eq!(
    //     WITH_EXPR_STRUCT.type_id(),
    //     <LazyLock<WithExprStruct> as Any>::type_id(&LazyLock::new(|| {
    //
    //         let _do_nothing = 0 + 0;
    //         WithExprStruct {
    //             data: "string".to_string(),
    //         }
    //     }))
    // )
}
