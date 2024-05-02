# AnyId (p_-)

This library provides `anyid::AnyId`. a struct can be used as an identifier for HashMap keys, etc.,
while keeping the type information ambiguous.

---

## Details

### Inner structure
`anyid::AnyId` internally uses Trait, `Identifier` wrapped in `Arc<T>` as a trait-object.

```rust
pub struct AnyId(Arc<dyn Identifier>);
```


### Trait-bound
Trait, `Identifier`, is automatically implemented for structures with several Trait implementations.

* `Copy`
  * Because most of the time, keys are used in K/V arrays. In addition, identifiers are desirable because they should be less costly to duplicate.
* `PartialEq`/`Eq`
* `Display`/`Debug`
* `Sync`/`Send`
* `Hash`

## Usage

### construct
`anyid::AnyId` is provided new just like a general structure, 
argument requires an `impl Identifier`.

```rust
use anyid::AnyId;

fn main() {
    let str_id: &str = "abc123";
    let id = AnyId::new(str_id);
}
```

Also, If the default implementation conditions for Identifier are met, 
a From<T> implementation is automatically provided.

```rust
use anyid::AnyId;

fn main() {
  let str_id: &str = "abc123";
  let id: AnyId = str_id.into();
}
```

### Clone
`anyid::AnyId` itself does not provide `Copy`.  
However, it implements `Clone` using `Arc::clone` as is.

```rust
impl Clone for AnyId {
  fn clone(&self) -> Self {
    Self(Arc::clone(&self.0))
  }
}
```

Therefore, cost of `Clone` is less.

### Equal
Implementation of `Eq`/(`PartialEq`). is in `Identifier`.

```rust
fn eq(&self, other: &dyn Identifier) -> bool {
  let Some(other) = other.as_any().downcast_ref::<T>() else {
    return false;
  };

  self.eq(other)
}
```

It is quite simple, 
but it is important to note that this process does not use `Result<T, E>`. 
false is returned if the downcast fails.

---

##### license
This library is released under MIT license.  
Bug reports and improvements are welcome ;D.