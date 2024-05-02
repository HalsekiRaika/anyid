use anyid::AnyId;
use uuid::Uuid;

fn downcast_ref() {
    let uuid = Uuid::new_v4();
    let id: AnyId = uuid.into();

    let id = id.downcast_ref::<Uuid>().unwrap();
    assert_eq!(id, &uuid);
}

#[test]
fn main() {
    downcast_ref();
}
