use std::mem::MaybeUninit;

pub fn hash_sha512(bytes: &[u8]) -> [u8; 64] {
    let mut out: MaybeUninit<[u8; 64]> = MaybeUninit::uninit();
    unsafe {
        tweetnacl_sys::crypto_hash_sha512_tweet(
            out.as_mut_ptr().cast(),
            bytes.as_ptr(),
            bytes.len() as u64,
        );

        out.assume_init()
    }
}

#[cfg(test)]
mod test {
    use crate::hash_sha512;

    #[test]
    fn it_hashes() {
        let bytes = b"Hello, world!";

        let the_hash = hash_sha512(bytes);
        assert_eq!(
            the_hash,
            [
                0xc1, 0x52, 0x7c, 0xd8, 0x93, 0xc1, 0x24, 0x77, 0x3d, 0x81, 0x19, 0x11, 0x97, 0xc,
                0x8f, 0xe6, 0xe8, 0x57, 0xd6, 0xdf, 0x5d, 0xc9, 0x22, 0x6b, 0xd8, 0xa1, 0x60, 0x61,
                0x4c, 0xc, 0xd9, 0x63, 0xa4, 0xdd, 0xea, 0x2b, 0x94, 0xbb, 0x7d, 0x36, 0x2, 0x1e,
                0xf9, 0xd8, 0x65, 0xd5, 0xce, 0xa2, 0x94, 0xa8, 0x2d, 0xd4, 0x9a, 0xb, 0xb2, 0x69,
                0xf5, 0x1f, 0x6e, 0x7a, 0x57, 0xf7, 0x94, 0x21,
            ]
        );
    }
}
