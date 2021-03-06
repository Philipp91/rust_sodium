// crypto_box.h

pub const crypto_box_SEEDBYTES: usize = crypto_box_curve25519xsalsa20poly1305_SEEDBYTES;
pub const crypto_box_PUBLICKEYBYTES: usize = crypto_box_curve25519xsalsa20poly1305_PUBLICKEYBYTES;
pub const crypto_box_SECRETKEYBYTES: usize = crypto_box_curve25519xsalsa20poly1305_SECRETKEYBYTES;
pub const crypto_box_BEFORENMBYTES: usize = crypto_box_curve25519xsalsa20poly1305_BEFORENMBYTES;
pub const crypto_box_NONCEBYTES: usize = crypto_box_curve25519xsalsa20poly1305_NONCEBYTES;
pub const crypto_box_ZEROBYTES: usize = crypto_box_curve25519xsalsa20poly1305_ZEROBYTES;
pub const crypto_box_BOXZEROBYTES: usize = crypto_box_curve25519xsalsa20poly1305_BOXZEROBYTES;
pub const crypto_box_MACBYTES: usize = crypto_box_curve25519xsalsa20poly1305_MACBYTES;
pub const crypto_box_PRIMITIVE: *const c_char = (b"curve25519xsalsa20poly1305\0" as *const u8) as
                                                *const c_char;
pub const crypto_box_SEALBYTES: usize = (crypto_box_PUBLICKEYBYTES + crypto_box_MACBYTES);


extern "C" {
    pub fn crypto_box_seedbytes() -> size_t;
    pub fn crypto_box_publickeybytes() -> size_t;
    pub fn crypto_box_secretkeybytes() -> size_t;
    pub fn crypto_box_beforenmbytes() -> size_t;
    pub fn crypto_box_noncebytes() -> size_t;
    pub fn crypto_box_zerobytes() -> size_t;
    pub fn crypto_box_boxzerobytes() -> size_t;
    pub fn crypto_box_macbytes() -> size_t;
    pub fn crypto_box_primitive() -> *const c_char;
    pub fn crypto_box_sealbytes() -> size_t;

    pub fn crypto_box_seed_keypair(pk: *mut u8, sk: *mut u8, seed: *const u8) -> c_int;
    pub fn crypto_box_keypair(pk: *mut u8, sk: *mut u8) -> c_int;
    pub fn crypto_box_beforenm(k: *mut u8, pk: *const u8, sk: *const u8) -> c_int;
    pub fn crypto_box_afternm(c: *mut u8,
                              m: *const u8,
                              mlen: c_ulonglong,
                              n: *const u8,
                              k: *const u8)
                              -> c_int;
    pub fn crypto_box_open_afternm(m: *mut u8,
                                   c: *const u8,
                                   clen: c_ulonglong,
                                   n: *const u8,
                                   k: *const u8)
                                   -> c_int;
    pub fn crypto_box(c: *mut u8,
                      m: *const u8,
                      mlen: c_ulonglong,
                      n: *const u8,
                      pk: *const u8,
                      sk: *const u8)
                      -> c_int;
    pub fn crypto_box_open(m: *mut u8,
                           c: *const u8,
                           clen: c_ulonglong,
                           n: *const u8,
                           pk: *const u8,
                           sk: *const u8)
                           -> c_int;
    pub fn crypto_box_easy(c: *mut u8,
                           m: *const u8,
                           mlen: c_ulonglong,
                           n: *const u8,
                           pk: *const u8,
                           sk: *const u8)
                           -> c_int;
    pub fn crypto_box_open_easy(m: *mut u8,
                                c: *const u8,
                                clen: c_ulonglong,
                                n: *const u8,
                                pk: *const u8,
                                sk: *const u8)
                                -> c_int;
    pub fn crypto_box_detached(c: *mut u8,
                               mac: *mut u8,
                               m: *const u8,
                               mlen: c_ulonglong,
                               n: *const u8,
                               pk: *const u8,
                               sk: *const u8)
                               -> c_int;
    pub fn crypto_box_open_detached(m: *mut u8,
                                    c: *const u8,
                                    mac: *const u8,
                                    clen: c_ulonglong,
                                    n: *const u8,
                                    pk: *const u8,
                                    sk: *const u8)
                                    -> c_int;
    pub fn crypto_box_seal(c: *mut u8, m: *const u8, mlen: c_ulonglong, pk: *const u8) -> c_int;
    pub fn crypto_box_seal_open(m: *mut u8,
                                c: *const u8,
                                clen: c_ulonglong,
                                pk: *const u8,
                                sk: *const u8)
                                -> c_int;
}

#[test]
fn test_crypto_box_seedbytes() {
    assert!(unsafe { crypto_box_seedbytes() as usize } == crypto_box_SEEDBYTES)
}
#[test]
fn test_crypto_box_publickeybytes() {
    assert!(unsafe { crypto_box_publickeybytes() as usize } == crypto_box_PUBLICKEYBYTES)
}
#[test]
fn test_crypto_box_secretkeybytes() {
    assert!(unsafe { crypto_box_secretkeybytes() as usize } == crypto_box_SECRETKEYBYTES)
}
#[test]
fn test_crypto_box_beforenmbytes() {
    assert!(unsafe { crypto_box_beforenmbytes() as usize } == crypto_box_BEFORENMBYTES)
}
#[test]
fn test_crypto_box_noncebytes() {
    assert!(unsafe { crypto_box_noncebytes() as usize } == crypto_box_NONCEBYTES)
}
#[test]
fn test_crypto_box_zerobytes() {
    assert!(unsafe { crypto_box_zerobytes() as usize } == crypto_box_ZEROBYTES)
}
#[test]
fn test_crypto_box_boxzerobytes() {
    assert!(unsafe { crypto_box_boxzerobytes() as usize } == crypto_box_BOXZEROBYTES)
}
#[test]
fn test_crypto_box_macbytes() {
    assert!(unsafe { crypto_box_macbytes() as usize } == crypto_box_MACBYTES)
}
#[test]
fn test_crypto_box_primitive() {
    use std::ffi::CStr;
    unsafe {
        assert_eq!(CStr::from_ptr(crypto_box_PRIMITIVE),
                   CStr::from_ptr(crypto_box_primitive()));
    }
}
#[test]
fn test_crypto_box_sealbytes() {
    assert!(unsafe { crypto_box_sealbytes() as usize } == crypto_box_SEALBYTES)
}
