use cocoa::base::{id, BOOL};
use crate::constants::ICEXIFOrientationType;
use core_graphics::image::CGImageRef;
use libc::c_uint;
use libc::{c_double, off_t};

/// ICCameraItem is an abstract class that represents an item in an ICCameraDevice object
pub trait ICCameraItem: Sized {
    /// Parent device of this folder.
    unsafe fn device(self) -> id;
    /// Parent folder of this folder.
    unsafe fn parentFolder(self) -> id;
    /// Name of this folder.
    unsafe fn name(self) -> id;
    /// Item UTI. This is an Uniform Type Identifier string.
    unsafe fn UTI(self) -> id;
    /// The file system path of the item for items on a device with transportType of ICTransportTypeMassStorage.
    unsafe fn fileSystemPath(self) -> id;
    /// Indicates the protection state of this folder.
    unsafe fn isLocked(self) -> BOOL;
    /// Indicates if the file is a raw image file.
    unsafe fn isRaw(self) -> BOOL;
    /// Indicates if this folder is in a temporary store.
    unsafe fn isInTemporaryStore(self) -> BOOL;
    /// Creation date of this file.
    unsafe fn creationDate(self) -> id;
    /// Modification date of this file.
    unsafe fn modificationDate(self) -> id;
    /// Thumbnail for the item if one is readily available.
    unsafe fn thumbnailIfAvailable(self) -> CGImageRef;
    /// Large thumbnail for the item if one is readily available.
    unsafe fn largeThumbnailIfAvailable(self) -> CGImageRef;
    /// Metadata for the file if one is readily available.
    unsafe fn metadataIfAvailable(self) -> id;
    /// A mutable dictionary to store arbitrary key-value pairs associated with a camera item object.
    unsafe fn userData(self) -> id;
    /// PTP object handle value if the item is on a camera that uses PTP protocol.
    unsafe fn ptpObjectHandle(self) -> c_uint;
    /// This property is set if the file is captured on the device after the device's content is fully enumerated.
    unsafe fn wasAddedAfterContentCatalogCompleted(self) -> BOOL;
}

impl ICCameraItem for id {
    unsafe fn device(self) -> id {
        msg_send![self, device]
    }

    unsafe fn parentFolder(self) -> id {
        msg_send![self, parentFolder]
    }

    unsafe fn name(self) -> id {
        msg_send![self, name]
    }

    unsafe fn UTI(self) -> id {
        msg_send![self, UTI]
    }

    unsafe fn fileSystemPath(self) -> id {
        msg_send![self, fileSystemPath]
    }

    unsafe fn isLocked(self) -> BOOL {
        msg_send![self, isLocked]
    }

    unsafe fn isRaw(self) -> BOOL {
        msg_send![self, isRaw]
    }

    unsafe fn isInTemporaryStore(self) -> BOOL {
        msg_send![self, isInTemporaryStore]
    }

    unsafe fn creationDate(self) -> id {
        msg_send![self, creationDate]
    }

    unsafe fn modificationDate(self) -> id {
        msg_send![self, modificationDate]
    }

    unsafe fn thumbnailIfAvailable(self) -> CGImageRef {
        msg_send![self, thumbnailIfAvailable]
    }

    unsafe fn largeThumbnailIfAvailable(self) -> CGImageRef {
        msg_send![self, largeThumbnailIfAvailable]
    }

    unsafe fn metadataIfAvailable(self) -> id {
        msg_send![self, metadataIfAvailable]
    }

    unsafe fn userData(self) -> id {
        msg_send![self, userData]
    }

    unsafe fn ptpObjectHandle(self) -> c_uint {
        msg_send![self, ptpObjectHandle]
    }

    unsafe fn wasAddedAfterContentCatalogCompleted(self) -> BOOL {
        msg_send![self, wasAddedAfterContentCatalogCompleted]
    }
}

/// This class represents a folder on an ICCameraDevice object.
pub trait ICCameraFolder: Sized {
    /// A list of items contained by this folder.
    unsafe fn contents(self) -> id;
}

impl ICCameraFolder for id {
    unsafe fn contents(self) -> id {
        msg_send![self, contents]
    }
}

/// This class represents a folder on an ICCameraDevice object.
pub trait ICCameraFile: Sized {
    /// Size of file in bytes.
    unsafe fn fileSize(self) -> off_t;
    /// Desired orientation of image to use when it is downloaded.
    unsafe fn orientation(self) -> ICEXIFOrientationType;
    /// Duration of audio/video file in seconds.
    unsafe fn duration(self) -> c_double;
    /// This property is NULL if there are no sidecar files associated with this file.
    /// Otherwise it is an array of ICCameraFile instances of sidecar files associated with this file.
    /// An example of a sidecar file is a file with the same base name as this file and having an extension XMP.
    unsafe fn sidecarFiles(self) -> id;
}

impl ICCameraFile for id {
    unsafe fn fileSize(self) -> off_t {
        msg_send![self, fileSize]
    }

    unsafe fn orientation(self) -> ICEXIFOrientationType {
        msg_send![self, orientation]
    }

    unsafe fn duration(self) -> c_double {
        msg_send![self, duration]
    }

    unsafe fn sidecarFiles(self) -> id {
        msg_send![self, sidecarFiles]
    }
}
