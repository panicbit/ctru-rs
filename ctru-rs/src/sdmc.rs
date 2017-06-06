pub struct Sdmc(());

impl Sdmc {
    pub fn init() -> ::Result<Sdmc> {
        unsafe {
            let r = ::libctru::sdmcInit();
            if r < 0 {
                Err(r.into())
            } else {
                Ok(Sdmc(()))
            }
        }
    }
}

impl Drop for Sdmc {
    fn drop(&mut self) {
        unsafe { ::libctru::sdmcExit() };
    }
}
