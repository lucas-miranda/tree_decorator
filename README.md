# Tree Decorator

An utility rust lib to render pleasing tree structures at terminal programs.

[![Latest Version](https://img.shields.io/crates/v/tree_decorator)](https://crates.io/crates/tree_decorator)

It's goal is to simplify tree structure display while ensuring a good looking to it. 
So it just handle strings, returning the expected result (with current level and supplied styles) and nothing more.

Already comes with a standard visual implementation, so is ready to use, but a custom implementation is very easy to set.

## Dependencies

As little as possible.

* [log](https://crates.io/crates/log): to simplify macro calls.

## Usage

Please, check [Documentation](https://docs.rs/tree_decorator) to see a detailed explanation.

## Examples

### Simple

```rust
use tree_decorator::tree_item;

tree_item!(block, "Items");
tree_item!("A");
tree_item!(block, "B");
tree_item!(last, "B.1");
tree_item!("C");
tree_item!(last, "D");
```

```
Items
├  A
├  B
│  └  B.1
├  C
└  D
```

### More Complex

```rust
use tree_decorator::tree_item;

tree_item!(block, "Items");
tree_item!("A");
tree_item!(block, "B");
tree_item!(block, "B.1");
tree_item!(last, "B.1.a");
tree_item!(block, "B.2");
tree_item!("B.2.a");
tree_item!(last, "B.2.b");
tree_item!(last, "B.3");
tree_item!("C");
tree_item!(last; block, "D");
tree_item!("D.1");
tree_item!("D.2");
tree_item!(last, "D.3");
close_tree_item!();
```

```rust
Items
├  A
├  B
│  ├  B.1
│  │  └  B.1.a
│  ├  B.2
│  │  ├  B.2.a
│  │  └  B.2.b
│  └  B.3
├  C
└  D
   ├  D.1
   ├  D.2
   └  D.3
```

## License

Everything is licensed under [MIT License](/LICENSE).
