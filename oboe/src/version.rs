use oboe_sys as ffi;
use std::str::from_utf8_unchecked;

/**
 * The version info
 */
pub struct Version;

impl Version {
    /**
     * The major version number
     */
    pub const MAJOR: u8 = ffi::oboe_Version_Major;

    /**
     * The minor version number
     */
    pub const MINOR: u8 = ffi::oboe_Version_Minor;

    /**
     * The patch version number
     */
    pub const PATCH: u16 = ffi::oboe_Version_Patch;

    /**
     * The version as 32-bit number
     */
    pub const NUMBER: u32 = ffi::oboe_Version_Number;

    /**
     * The version as text
     */
    pub fn text() -> &'static str {
        unsafe { from_utf8_unchecked(ffi::oboe_Version_Text.as_ref()) }
    }
}
