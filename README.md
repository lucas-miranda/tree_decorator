# Tree Decorator

An utility rust lib to render pleasing tree structures at terminal programs.

[![Latest Version](https://img.shields.io/crates/v/tree_decorator)](https://crates.io/crates/tree_decorator)

It's goal is to simplify tree structure display while ensuring a good looking to it. So it just handle strings, returning the expected result (with current level and supplied styles) and nothing more.

Already comes with a standard visual implementation, so is ready to use, but a custom implementation is very easy to set.

See [Documentation](https://docs.rs/tree_decorator).

## Dependencies

As little, or none, as needed.
Until now, only [log](https://crates.io/crates/log) will be added, in the near future, to simplify macro calls.

## Usage

Before any use, decorator must be build at least once.

```rust
use tree::{
    DecoratorBuilder,
    tree_item
};

fn main() {
    DecoratorBuilder::default()
                     .default_handler(|m| println!("{}", m))
                     .build();
    
    tree_item!(block, "Root item");
    tree_item!("Item A");
    tree_item!("Item B");
    tree_item!("Item C");
    tree_item!(last, "Item D");
}
```

Outputs:

```
Root item
├  Item A
├  Item B
├  Item C
└  Item D
```

## Explanation

Everything works around `tree_item!` macro, it has some specific features that must be explained.
Each section is optional, as long the order is preserved.

```
tree_item!(
    [styles],
    "A literal string",
    [string interpolation args]
)
```

### Styles

A style which will be applied to that tree item.

Every opt-in style must match the name defined at Style struct fields (please check [Documentation](https://docs.rs/tree_decorator)) and multiple ones can be declared using `;` as separator to them.
Declaring only it's name will apply a style item default enable value. (Not to be confused with `std::default::Default` it could not be the same value)

#### Example

```rust
use tree_decorator::tree_item;

tree_item!(block; dashed, "Dashed block starting tree item");
```

Some style items could support more values than enable or disable.
Such as `entry`:

```rust
use tree_decorator::{
    decorator::Entry,
    tree_item
};

tree_item!(block; dashed; entry: Entry::Double, "Dashed block starting with double entry tree item");
```

**Note**: Order doesn't matter at styles defination.

### Literal String and Interpolation Args

Both of these is the same as defined at [std::format](https://doc.rust-lang.org/std/macro.format.html).

## License

Everything is licensed under [MIT License](/LICENSE).
