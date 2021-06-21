use std::cell::RefCell;

use tree_decorator::{
    decorator::Entry,
    DecoratorBuilder,
    end_all_tree_items,
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
                                 b.borrow_mut().push_str(&m);
                                 b.borrow_mut().push('\n');
                             }
                         }
                     })
                     .build();

    tree_decorator::handle_styles!(block:true);

    tree_item!(block, "Root");
    tree_item!(entry, "Item A");
    tree_item!(entry: Entry::None, "Small Description");
    tree_item!(dashed, "Other Small Description");
    tree_item!(last; block, "Item B");
    tree_item!("Item Ba");
    tree_item!(block, "Item Bb");
    tree_item!(last, "Item Bba");
    tree_item!(last, "Item Bc");
    end_all_tree_items!();

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
