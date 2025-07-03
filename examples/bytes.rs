use anyhow::Result;
use bytes::{BufMut, Bytes, BytesMut};

fn main() -> Result<()> {
    let mut buf = BytesMut::with_capacity(1024);
    buf.extend_from_slice(b"hello world\n");
    buf.put(&b"goodbye world"[..]);
    buf.put_u8(b'\n');
    buf.put_i64(1234567890);
    println!("buf: {buf:?}");

    let buf1 = buf.split();
    println!("buf1: {buf1:?}");
    let mut buf2 = buf1.freeze();
    println!("buf2: {buf2:?}");

    let buf3 = buf2.split_to(12);
    println!("buf3: {buf3:?}");
    println!("buf2: {buf2:?}");

    let mut bytes = BytesMut::new();
    bytes.extend_from_slice(b"hello");

    println!("bytes: {bytes:?}");

    let bytes = Bytes::from(b"hello".to_vec());
    assert_eq!(BytesMut::from(bytes), BytesMut::from(&b"hello"[..]));
    Ok(())
}
