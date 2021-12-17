/* automatically generated by rust-bindgen 0.59.2 */

pub const CRYPTO_PUBLICKEYBYTES: u32 = 1472;
pub const CRYPTO_SECRETKEYBYTES: u32 = 3504;
pub const CRYPTO_BYTES: u32 = 2701;
pub const CRYPTO_ALGNAME: &[u8; 10usize] = b"Dilithium\0";
extern "C" {
    pub fn crypto_sign_keypair(
        pk: *mut ::std::os::raw::c_uchar,
        sk: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_sign(
        sm: *mut ::std::os::raw::c_uchar,
        smlen: *mut ::std::os::raw::c_ulonglong,
        msg: *const ::std::os::raw::c_uchar,
        len: ::std::os::raw::c_ulonglong,
        sk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_sign_open(
        m: *mut ::std::os::raw::c_uchar,
        mlen: *mut ::std::os::raw::c_ulonglong,
        sm: *const ::std::os::raw::c_uchar,
        smlen: ::std::os::raw::c_ulonglong,
        pk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
pub type size_t = ::std::os::raw::c_ulong;
extern "C" {
    pub fn randombytes(x: *mut ::std::os::raw::c_uchar, xlen: size_t);
}
