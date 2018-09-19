use cocoa::base::{id, BOOL};
use cocoa::foundation::NSUInteger;
use scanner_functional_units::ICScannerPixelDataType;

/// ICScannerBandData
pub trait ICScannerBandData: Sized {
    /// Describes the full image width of the banded image.
    unsafe fn fullImageWidth(self) -> NSUInteger;
    /// Describes the full image height of the banded image.
    unsafe fn fullImageHeight(self) -> NSUInteger;
    /// Describes the number of bits per pixel for banded the image.
    unsafe fn bitsPerPixel(self) -> NSUInteger;
    /// Describes the number of bits per component for the banded image.
    unsafe fn bitsPerComponent(self) -> NSUInteger;
    /// Describes how many components are contained within the banded image.
    unsafe fn numComponents(self) -> NSUInteger;
    /// Describes if the banded image data is reported in big endian.
    unsafe fn isBigEndian(self) -> BOOL;
    /// Type of pixel data that is contained in the band.
    unsafe fn pixelDataType(self) -> ICScannerPixelDataType;
    /// Returns the path to the color profile matching the banded data.
    unsafe fn colorSyncProfilePath(self) -> id;
    /// Describes how many bytes are in each image band row.
    unsafe fn bytesPerRow(self) -> NSUInteger;
    /// Describes the start row of the image band.
    unsafe fn dataStartRow(self) -> NSUInteger;
    /// Describes the number of rows contained in the image band.
    unsafe fn dataNumRows(self) -> NSUInteger;
    /// Describes the actual data size of the image band buffer.
    unsafe fn dataSize(self) -> NSUInteger;
    /// The pointer to the data buffer object.
    unsafe fn dataBuffer(self) -> id;
}

impl ICScannerBandData for id {
    unsafe fn fullImageWidth(self) -> NSUInteger {
        msg_send![self, fullImageWidth]
    }

    unsafe fn fullImageHeight(self) -> NSUInteger {
        msg_send![self, fullImageHeight]
    }

    unsafe fn bitsPerPixel(self) -> NSUInteger {
        msg_send![self, bitsPerPixel]
    }

    unsafe fn bitsPerComponent(self) -> NSUInteger {
        msg_send![self, bitsPerComponent]
    }

    unsafe fn numComponents(self) -> NSUInteger {
        msg_send![self, numComponents]
    }

    unsafe fn isBigEndian(self) -> BOOL {
        msg_send![self, isBigEndian]
    }

    unsafe fn pixelDataType(self) -> ICScannerPixelDataType {
        msg_send![self, pixelDataType]
    }

    unsafe fn colorSyncProfilePath(self) -> id {
        msg_send![self, colorSyncProfilePath]
    }

    unsafe fn bytesPerRow(self) -> NSUInteger {
        msg_send![self, bytesPerRow]
    }

    unsafe fn dataStartRow(self) -> NSUInteger {
        msg_send![self, dataStartRow]
    }

    unsafe fn dataNumRows(self) -> NSUInteger {
        msg_send![self, dataNumRows]
    }

    unsafe fn dataSize(self) -> NSUInteger {
        msg_send![self, dataSize]
    }

    unsafe fn dataBuffer(self) -> id {
        msg_send![self, dataBuffer]
    }
}
