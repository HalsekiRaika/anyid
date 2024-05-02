use anyid::{AnyId, Identifier};
use std::collections::HashSet;
use uuid::Uuid;

fn eq_refs() {
    let uuid = Uuid::new_v4();
    let copied = uuid;

    println!("copy: {}", copied);

    let id: AnyId = uuid.into();

    let mut set = HashSet::new();
    set.insert(id);
    let set = Set(set);

    let id = set.find(&copied);

    println!("find: {}", id);

    assert_eq!(id, &copied);

    pub struct Set(HashSet<AnyId>);

    impl Set {
        pub fn find(&self, id: &impl Identifier) -> &AnyId {
            self.0.iter().find(|item| PartialEq::eq(item, &id)).unwrap()
        }
    }
}

#[test]
fn main() {
    eq_refs()
}
