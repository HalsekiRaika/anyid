use anyid::AnyId;
use uuid::Uuid;

fn eq_unmatched_value() {
    let uuid = Uuid::new_v4();
    let other_uuid = Uuid::new_v4();
    let id: AnyId = uuid.into();

    assert_ne!(id, other_uuid);

    let id = id.downcast::<Uuid>().unwrap();

    assert_ne!(id, other_uuid);
}

#[test]
fn main() {
    eq_unmatched_value()
}
