# Mock Generator

## Objective

This is to illustrate a functionality which I'm using in my Rust code to
generating objects for `struct`s to be able to use them for testing or other
reason. This is to minimize developer's efforts to create objects to test.

## Design

```rust
struct Object {
    pub id: usize,
    pub name: String,
    pub index: i32,
}

impl Object {
    pub fn mock(&self) -> Object {
        // Generate a mock for object which each field will be randomized based on type.
    }

    pub fn mocks(&self, count: i32) -> (Vec<Object>, Duration) {
        // Generate a list of mocks for Object which each field will be randomized based on type.
    }
}
```
