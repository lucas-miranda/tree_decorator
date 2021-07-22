#[cfg(not(feature = "no_log"))]
#[test]
fn test_output() {
    use simple_logger::SimpleLogger;

    use tree_decorator::{
        decorator::Entry,
        DecoratorBuilder,
        tree_item_info,
        tree_item_debug,
        tree_item_error,
        tree_item_trace,
        tree_item_warn
    };

    SimpleLogger::new().init().unwrap();

    DecoratorBuilder::default()
                     .build();

    assert_eq!(0, tree_decorator::level());

    tree_item_info!(block, "Root");
        assert_eq!(1, tree_decorator::level());
        tree_item_info!(entry, "Item A");
        tree_item_debug!(entry: Entry::None, "Small Description");
        tree_item_trace!(dashed, "Other Small Description");
        tree_item_info!(last; block, "Item B");
            assert_eq!(2, tree_decorator::level());
            tree_item_info!("Item Ba");
            tree_item_error!(block, "Item Bb");
                assert_eq!(3, tree_decorator::level());
                tree_item_warn!(last, "Item Bba");
            assert_eq!(2, tree_decorator::level());
            tree_item_trace!(last, "Item Bc");
    assert_eq!(0, tree_decorator::level());

    tree_decorator::shutdown();
}
