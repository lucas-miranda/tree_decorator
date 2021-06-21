#[cfg(not(feature = "no_log"))]
#[test]
fn test_output() {
    use simple_logger::SimpleLogger;

    use tree_decorator::{
        decorator::Entry,
        DecoratorBuilder,
        end_all_tree_items,
        tree_item_info,
        tree_item_debug,
        tree_item_error,
        tree_item_trace,
        tree_item_warn
    };

    SimpleLogger::new().init().unwrap();

    DecoratorBuilder::default()
                     .build();

    tree_decorator::handle_styles!(block:true);

    tree_item_info!(block, "Root");
    tree_item_info!(entry, "Item A");
    tree_item_debug!(entry: Entry::None, "Small Description");
    tree_item_trace!(dashed, "Other Small Description");
    tree_item_info!(last; block, "Item B");
    tree_item_info!("Item Ba");
    tree_item_error!(block, "Item Bb");
    tree_item_warn!(last, "Item Bba");
    tree_item_trace!(last, "Item Bc");
    end_all_tree_items!();

    tree_decorator::shutdown();
}
