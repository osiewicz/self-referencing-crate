use old_client::
fn main() {
    assert_eq!(old_client::getter(), new_client::getter());
}
