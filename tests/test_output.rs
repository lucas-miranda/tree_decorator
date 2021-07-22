use std::cell::RefCell;

use tree_decorator::{
    close_tree_item,
    decorator::Entry,
    DecoratorBuilder,
    tree_item
};

static mut BUFFER: Option<RefCell<String>> = None;

#[test]
fn test_output() {
    unsafe {
        BUFFER = Some(RefCell::new(String::new()));
    }

    DecoratorBuilder::default()
                     .default_handler(|m| {
                         unsafe {
                             if let Some(b) = &BUFFER {
                                 let mut buffer = b.borrow_mut();
                                 buffer.push_str(&m);
                                 buffer.push('\n');
                             }
                         }
                     })
                     .build();

    assert_eq!(0, tree_decorator::level());

    tree_item!(block, "Root");
        assert_eq!(1, tree_decorator::level());
        tree_item!(entry, "Item A");
        tree_item!(entry: Entry::None, "Small Description");
        tree_item!(dashed, "Other Small Description");
        tree_item!(last; block, "Item B");
            assert_eq!(2, tree_decorator::level());
            tree_item!("Item Ba");
            tree_item!(block, "Item Bb");
                assert_eq!(3, tree_decorator::level());
                tree_item!(last, "Item Bba");
            assert_eq!(2, tree_decorator::level());
            tree_item!(last, "Item Bc");
    assert_eq!(0, tree_decorator::level());

    unsafe { 
        if let Some(b) = &BUFFER {
            println!("{}", b.borrow());

            assert_eq!(
                "Root
├  Item A
│  Small Description
┊- Other Small Description
└  Item B
   ├  Item Ba
   ├  Item Bb
   │  └  Item Bba
   └  Item Bc
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
