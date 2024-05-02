use anyid::AnyId;
use uuid::Uuid;

fn downcast() {
    let uuid = Uuid::new_v4();
    let copied = uuid;
    let id: AnyId = uuid.into();

    let id = id.downcast::<Uuid>().unwrap();

    assert_eq!(copied, id);

    let str_id = "abc123";
    let copied = str_id;
    let id: AnyId = str_id.into();

    let id = id.downcast::<&str>().unwrap();

    assert_eq!(copied, id);
}

#[test]
fn main() {
    downcast();
}
