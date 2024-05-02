use anyid::AnyId;
use uuid::Uuid;

fn init() {
    let uuid = Uuid::new_v4();
    let id: AnyId = uuid.into();

    println!("{}", id);

    let str_id = "abc123";
    let id: AnyId = str_id.into();

    println!("{}", id);

    let uuid = Uuid::new_v4();
    let id = AnyId::new(uuid);

    println!("{}", id);

    let str_id = "abc456";
    let id = AnyId::new(str_id);

    println!("{}", id);
}

#[test]
fn main() {
    init()
}
