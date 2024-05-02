use anyid::AnyId;
use uuid::Uuid;

fn downcast_failure() -> Result<(), anyid::Error> {
    let uuid = Uuid::new_v4();
    let id: AnyId = uuid.into();

    let _panic = id.downcast::<&str>()?;

    Ok(())
}

#[test]
fn main() {
    assert!(downcast_failure().is_err())
}
