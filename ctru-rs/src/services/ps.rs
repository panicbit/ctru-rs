use libctru::services::ps::*;
use Result;

// TODO: Implement crypto functions

/// ProcessService
pub struct PS(());

impl PS {
    /// Initialize the ProcessService
    pub fn init() -> Result<Self> {
        unsafe {
            let r = psInit();
            if r < 0 {
                Err(r.into())
            } else {
                Ok(PS(()))
            }
        }
    }

    /// Get the local friend code seed
    pub fn local_friend_code_seed(&self) -> Result<u64> {
        unsafe {
            let mut seed = 0;
            let r = PS_GetLocalFriendCodeSeed(&mut seed);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(seed)
            }
        }
    }

    /// Get device id
    pub fn device_id(&self) -> Result<u32> {
        unsafe {
            let mut id = 0;
            let r = PS_GetDeviceId(&mut id);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(id)
            }
        }
    }

    /// Fill `buf` with `buf.len()` cryptographically secure random bytes
    pub fn generate_random_bytes(&self, buf: &mut [u8]) -> Result<()> {
        unsafe {
            let r = PS_GenerateRandomBytes(buf.as_ptr() as _, buf.len());
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }
}

impl Drop for PS {
    fn drop(&mut self) {
        unsafe { psExit() };
    }
}
