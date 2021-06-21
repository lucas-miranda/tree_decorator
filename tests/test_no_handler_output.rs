use tree_decorator::{
    decorator::Entry,
    DecoratorBuilder,
    end_all_tree_items,
    tree_item
};

#[test]
fn test_no_handler_output() {
    DecoratorBuilder::default()
                     .build();

    assert_eq!("Root", tree_item!(block, "Root"));
    assert_eq!("├  Item A", tree_item!(entry, "Item A"));
    assert_eq!("│  Small Description", tree_item!(entry: Entry::None, "Small Description"));
    assert_eq!("┊- Other Small Description", tree_item!(dashed, "Other Small Description"));
    assert_eq!("└  Item B", tree_item!(last; block, "Item B"));
    assert_eq!("   ├  Item Ba", tree_item!("Item Ba"));
    assert_eq!("   ├  Item Bb", tree_item!(block, "Item Bb"));
    assert_eq!("   │  └  Item Bba", tree_item!(last, "Item Bba"));
    assert_eq!("   └  Item Bc", tree_item!(last, "Item Bc"));

    end_all_tree_items!();

    assert_eq!("Root", format!("{}Root", tree_item!(block)));
    assert_eq!("├  Item A", format!("{}Item A", tree_item!(entry)));
    assert_eq!("│  Small Description", format!("{}Small Description", tree_item!(entry: Entry::None)));
    assert_eq!("┊- Other Small Description", format!("{}Other Small Description", tree_item!(dashed)));
    assert_eq!("└  Item B", format!("{}Item B", tree_item!(last; block)));
    assert_eq!("   ├  Item Ba", format!("{}Item Ba", tree_item!()));
    assert_eq!("   ├  Item Bb", format!("{}Item Bb", tree_item!(block)));
    assert_eq!("   │  └  Item Bba", format!("{}Item Bba", tree_item!(last)));
    assert_eq!("   └  Item Bc", format!("{}Item Bc", tree_item!(last)));

    end_all_tree_items!();

    tree_decorator::shutdown();
}
