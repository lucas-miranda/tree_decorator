use tree_decorator::{
    close_tree_item,
    DecoratorBuilder,
    tree_item
};

#[test]
fn test_items_level() {
    DecoratorBuilder::default().build();

    assert_eq!(0, tree_decorator::level());

    tree_item!(block, "Root");
        assert_eq!(1, tree_decorator::level());
        tree_item!("Item A");
        tree_item!("Small Description");
        tree_item!("Other Small Description");
        tree_item!(last; block, "Item B");
            assert_eq!(2, tree_decorator::level());
            tree_item!("Item Ba");
            tree_item!(block, "Item Bb");
                assert_eq!(3, tree_decorator::level());
                tree_item!(last, "Item Bba");
            assert_eq!(2, tree_decorator::level());
            tree_item!(last, "Item Bc");
        assert_eq!(1, tree_decorator::level());
        close_tree_item!();

    assert_eq!(0, tree_decorator::level());

    tree_decorator::shutdown();
}
