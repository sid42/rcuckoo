use rcuckoo::Cuckoo;

#[test]
fn simple() {
    let mut c = Cuckoo::new(2_i32.pow(10), 4, 500);

    assert_eq!(c.insert("hi".as_bytes()), Ok(()));
    assert_eq!(c.exists("hi".as_bytes()), true);
    assert_eq!(c.exists("hello".as_bytes()), false);
    assert_eq!(c.delete("hi".as_bytes()), Ok(()));
    assert_eq!(c.exists("hi".as_bytes()), false);
    // delete item that does not exist
    assert!(c.delete("hola".as_bytes()).is_err());
}