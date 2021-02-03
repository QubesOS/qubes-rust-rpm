use openpgp_parser::{signature::read_signature, Error, buffer::Reader};
use std::os::raw::{c_int, c_uint};
enum RpmPgpDigParams {}

#[repr(transparent)]
pub(super) struct Signature(*mut RpmPgpDigParams);

impl Signature {
    pub fn parse(buffer: Reader, time: u32) -> Result<Self, Error> {
        super::init();
        let mut cp = buffer.clone();
        // Check that the signature is valid
        read_signature(&mut cp, time)?;
        // We can now pass the buffer to RPM, since it is a valid signature
        let slice = buffer.as_untrusted_slice();
        let mut params = Signature(std::ptr::null_mut());
        let r = unsafe { pgpPrtParams(slice.as_ptr(), slice.len(), 2, &mut params) };
        assert!(r == 0, "we accepted a signature RPM rejected");
        assert!(!params.0.is_null());
        Ok(params)
    }

    /// Retrieve the hash algorithm of the signature
    pub fn hash_algorithm(&self) -> u8 {
        let alg = unsafe { pgpDigParamsAlgo(self.0, 9) };
        assert!(alg <= 255, "invalid hash algorithm not rejected earlier?");
        alg as _
    }

    /// Retrieve the public key algorithm of the signature
    pub fn public_key_algorithm(&self) -> u8 {
        use std::convert::TryInto;
        let alg = unsafe { pgpDigParamsAlgo(self.0, 6) };
        alg.try_into()
            .expect("OpenPGP public key algorithms always fit in a u8; qed")
    }
}

impl Drop for Signature {
    fn drop(&mut self) {
        if !self.0.is_null() {
            self.0 = unsafe { pgpDigParamsFree(self.0) }
        }
    }
}

#[link(name = "rpmio")]
extern "C" {
    fn pgpPrtParams(pkts: *const u8, pktlen: usize, pkttype: c_uint, ret: &mut Signature) -> c_int;
    fn pgpDigParamsFree(digp: *mut RpmPgpDigParams) -> *mut RpmPgpDigParams;
    fn pgpDigParamsAlgo(digp: *const RpmPgpDigParams, algotype: c_uint) -> c_uint;
}
