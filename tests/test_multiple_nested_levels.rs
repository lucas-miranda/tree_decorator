use std::cell::RefCell;

use tree_decorator::{
    close_tree_item,
    DecoratorBuilder,
    tree_item
};

static mut BUFFER: Option<RefCell<String>> = None;

#[test]
fn test_multiple_nested_levels_output() {
    unsafe {
        BUFFER = Some(RefCell::new(String::new()));
    }

    DecoratorBuilder::default()
                     .default_handler(|m| {
                         unsafe {
                             if let Some(b) = &BUFFER {
                                 b.borrow_mut().push_str(&m);
                                 b.borrow_mut().push('\n');
                             }
                         }
                     })
                     .build();

    assert_eq!(0, tree_decorator::level());

    tree_item!(block, "Very Nested Items");
    assert_eq!(1, tree_decorator::level());
    tree_item!("A");
    tree_item!(block, "B");
    assert_eq!(2, tree_decorator::level());
        tree_item!(block; last, "B.a");
            assert_eq!(3, tree_decorator::level());
            tree_item!("B.a.a");
            tree_item!("B.a.b");
            tree_item!("B.a.c");
            tree_item!(block, "B.a.c");
                assert_eq!(4, tree_decorator::level());
                tree_item!("B.a.c.a");
                tree_item!("B.a.c.b");
                tree_item!(last, "B.a.c.c");
            assert_eq!(3, tree_decorator::level());
            tree_item!(last, "B.a.d");
        assert_eq!(2, tree_decorator::level());
        close_tree_item!();
    assert_eq!(1, tree_decorator::level());
    tree_item!("C");
    tree_item!("D");
    tree_item!("E");
    tree_item!(last, "F");

    assert_eq!(0, tree_decorator::level());

    unsafe { 
        if let Some(b) = &BUFFER {
            println!("{}", b.borrow());

            assert_eq!(
                "Very Nested Items
├  A
├  B
│  └  B.a
│     ├  B.a.a
│     ├  B.a.b
│     ├  B.a.c
│     ├  B.a.c
│     │  ├  B.a.c.a
│     │  ├  B.a.c.b
│     │  └  B.a.c.c
│     └  B.a.d
├  C
├  D
├  E
└  F
",
                b.borrow().as_str()
            )
        } else {
            panic!("Expected BUFFER isn't defined.")
        }
    }

    tree_decorator::shutdown();

    unsafe {
        BUFFER = None;
    }
}
