# AnyId (p_-)

This library provides `anyid::AnyId`. a struct can be used as an identifier for HashMap keys, etc.,
while keeping the type information ambiguous.

---

## Why is this necessary?
All data often has an identifier to identify it. Often it is tightly constrained by the type. (e.g, using the NewType pattern such as `ProductId(uuid::Uuid)` instead of just `uuid::Uuid`).
However, when these data are lumped together and treated as just data in an integrated manner, such type restrictions seem a bit cumbersome. Therefore, we can solve this problem by providing a TraitObject that uses a Trait that is a collection of Traits that would be implemented at a minimum when used as an identifier.
Note that this crate is intended to facilitate data handling at the application level and is not intended for ignore type processing such as (De)Serialize. Also, if you use this crate as an identifier for data, it can be very troublesome later on, since it only implements what is needed to handle the data in an organized manner.

## Details

### Inner structure
`anyid::AnyId` internally uses Trait, `Identifier` wrapped in `Arc<T>` as a trait-object.

```rust
pub struct AnyId(Arc<dyn Identifier>);
```


### Trait-bound
Trait, `Identifier`, is automatically implemented for structures with several Trait implementations.

* `Copy`
  * Why is this required: When there is a scope that requires a `'static` lifetime, such as in `tokio::spawn`, or when inserting data into a `HashMap`, etc., the data is usually duplicated, since it consumes values and can be used over and over again. In most cases, duplicated data is used. In addition, we thought that `Copy` would be more appropriate than `Clone` because the identifier should be “less resource-intensive to duplicate”.
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