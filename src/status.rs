use std::c_str::CString;
use ffi::cairo_status_to_string;

#[repr(u32)]
#[deriving(FromPrimitive, Show, Copy)]
pub enum Status {
    Success = 0,
    NoMemory = 1,
    InvalidRestore = 2,
    InvalidPopGroup = 3,
    NoCurrentPoint = 4,
    InvalidMatrix = 5,
    InvalidStatus = 6,
    NullPointer = 7,
    InvalidString = 8,
    InvalidPathData = 9,
    ReadError = 10,
    WriteError = 11,
    SurfaceFinished = 12,
    SurfaceTypeMismatch = 13,
    PatternTypeMismatch = 14,
    InvalidContent = 15,
    InvalidFormat = 16,
    InvalidVisual = 17,
    FileNotFound = 18,
    InvalidDash = 19,
    InvalidDscComment = 20,
    InvalidIndex = 21,
    ClipNotRepresentable = 22,
    TempFileError = 23,
    InvalidStride = 24,
    FontTypeMismatch = 25,
    UserFontImmutable = 26,
    UserFontError = 27,
    NegativeCount = 28,
    InvalidClusters = 29,
    InvalidSlant = 30,
    InvalidWeight = 31,
    InvalidSize = 32,
    UserFontNotImplemented = 33,
    DeviceTypeMismatch = 34,
    DeviceError = 35,
    InvalidMeshConstruction = 36,
    DeviceFinished = 37,
    Jbig2GlobalMissing = 38,
    LastStatus = 39,
}

impl Status {
    pub fn to_string(&self) -> String {
        let this = *self as u32;
        let status = unsafe { cairo_status_to_string(this) };
        let status = unsafe { CString::new(status, true) };
        match status.as_str() {
            Some(stat) => stat.into_string(),
            None => panic!("Error creating string from status {}", this)
        }
    }
}
