use std::c_str::CString;
use ffi::*;

#[repr(u32)]
#[deriving(FromPrimitive, Show, Copy)]
pub enum Status {
    Success = CAIRO_STATUS_SUCCESS,
    NoMemory = CAIRO_STATUS_NO_MEMORY,
    InvalidRestore = CAIRO_STATUS_INVALID_RESTORE,
    InvalidPopGroup = CAIRO_STATUS_INVALID_POP_GROUP,
    NoCurrentPoint = CAIRO_STATUS_NO_CURRENT_POINT,
    InvalidMatrix = CAIRO_STATUS_INVALID_MATRIX,
    InvalidStatus = CAIRO_STATUS_INVALID_STATUS,
    NullPointer = CAIRO_STATUS_NULL_POINTER,
    InvalidString = CAIRO_STATUS_INVALID_STRING,
    InvalidPathData = CAIRO_STATUS_INVALID_PATH_DATA,
    ReadError = CAIRO_STATUS_READ_ERROR,
    WriteError = CAIRO_STATUS_WRITE_ERROR,
    SurfaceFinished = CAIRO_STATUS_SURFACE_FINISHED,
    SurfaceTypeMismatch = CAIRO_STATUS_SURFACE_TYPE_MISMATCH,
    PatternTypeMismatch = CAIRO_STATUS_PATTERN_TYPE_MISMATCH,
    InvalidContent = CAIRO_STATUS_INVALID_CONTENT,
    InvalidFormat = CAIRO_STATUS_INVALID_FORMAT,
    InvalidVisual = CAIRO_STATUS_INVALID_VISUAL,
    FileNotFound = CAIRO_STATUS_FILE_NOT_FOUND,
    InvalidDash = CAIRO_STATUS_INVALID_DASH,
    InvalidDscComment = CAIRO_STATUS_INVALID_DSC_COMMENT,
    InvalidIndex = CAIRO_STATUS_INVALID_INDEX,
    ClipNotRepresentable = CAIRO_STATUS_CLIP_NOT_REPRESENTABLE,
    TempFileError = CAIRO_STATUS_TEMP_FILE_ERROR,
    InvalidStride = CAIRO_STATUS_INVALID_STRIDE,
    FontTypeMismatch = CAIRO_STATUS_FONT_TYPE_MISMATCH,
    UserFontImmutable = CAIRO_STATUS_USER_FONT_IMMUTABLE,
    UserFontError = CAIRO_STATUS_USER_FONT_ERROR,
    NegativeCount = CAIRO_STATUS_NEGATIVE_COUNT,
    InvalidClusters = CAIRO_STATUS_INVALID_CLUSTERS,
    InvalidSlant = CAIRO_STATUS_INVALID_SLANT,
    InvalidWeight = CAIRO_STATUS_INVALID_WEIGHT,
    InvalidSize = CAIRO_STATUS_INVALID_SIZE,
    UserFontNotImplemented = CAIRO_STATUS_USER_FONT_NOT_IMPLEMENTED,
    DeviceTypeMismatch = CAIRO_STATUS_DEVICE_TYPE_MISMATCH,
    DeviceError = CAIRO_STATUS_DEVICE_ERROR,
    InvalidMeshConstruction = CAIRO_STATUS_INVALID_MESH_CONSTRUCTION,
    DeviceFinished = CAIRO_STATUS_DEVICE_FINISHED,
    Jbig2GlobalMissing = CAIRO_STATUS_JBIG2_GLOBAL_MISSING,
    LastStatus = CAIRO_STATUS_LAST_STATUS,
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
