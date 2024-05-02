use anyid::{AnyId, Error};
use uuid::Uuid;

fn downcast_ref_failure() -> Result<(), Error> {
    let uuid = Uuid::new_v4();
    let id: AnyId = uuid.into();

    let _panic = id.downcast_ref::<&str>()?;

    Ok(())
}

#[test]
fn main() {
    assert!(downcast_ref_failure().is_err())
}
