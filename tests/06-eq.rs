use anyid::AnyId;
use uuid::Uuid;

fn eq() {
    let uuid = Uuid::new_v4();
    let copied = uuid;
    let id: AnyId = uuid.into();

    assert_eq!(id, copied);
}

#[test]
fn main() {
    eq()
}
