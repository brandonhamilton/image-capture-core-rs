#![allow(non_snake_case)]
#[macro_use]
extern crate bitflags;
extern crate cocoa;
extern crate core_foundation;
extern crate core_graphics;
#[macro_use]
extern crate objc;
extern crate libc;

#[cfg(target_os = "macos")]
pub mod camera_device;
#[cfg(target_os = "macos")]
pub mod camera_item;
#[cfg(target_os = "macos")]
pub mod device;
#[cfg(target_os = "macos")]
pub mod device_browser;
#[cfg(target_os = "macos")]
pub mod scanner_band_data;
#[cfg(target_os = "macos")]
pub mod scanner_device;
#[cfg(target_os = "macos")]
pub mod scanner_functional_units;

pub mod constants {
    /// Type representing EXIF Orientation tag value
    #[repr(u64)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum ICEXIFOrientationType {
        /// Normal
        ICEXIFOrientation1 = 1,
        /// Flipped horizontally    
        ICEXIFOrientation2 = 2,
        /// Rotated 180°
        ICEXIFOrientation3 = 3,
        /// Flipped vertically
        ICEXIFOrientation4 = 4,
        // Rotated 90° CCW and flipped vertically
        ICEXIFOrientation5 = 5,
        // Rotated 90° CCW
        ICEXIFOrientation6 = 6,
        // Rotated 90° CW and flipped vertically
        ICEXIFOrientation7 = 7,
        // Rotated 90° CW
        ICEXIFOrientation8 = 8,
    }

    /// Definition of codes returned by APIs in ImageCaptureCore framework
    #[repr(i64)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum ICReturnCode {
        ICReturnSuccess = 0,
        ICReturnInvalidParam = -9922,
        ICReturnCommunicationTimedOut = -9923,
        ICReturnScanOperationCanceled = -9924,
        ICReturnScannerInUseByLocalUser = -9925,
        ICReturnScannerInUseByRemoteUser = -9926,
        ICReturnDeviceFailedToOpenSession = -9927,
        ICReturnDeviceFailedToCloseSession = -9928,
        ICReturnScannerFailedToSelectFunctionalUnit = -9929,
        ICReturnScannerFailedToCompleteOverviewScan = -9930,
        ICReturnScannerFailedToCompleteScan = -9931,
        ICReturnReceivedUnsolicitedScannerStatusInfo = -9932,
        ICReturnReceivedUnsolicitedScannerErrorInfo = -9933,
        ICReturnDownloadFailed = -9934,
        ICReturnUploadFailed = -9935,
        ICReturnFailedToCompletePassThroughCommand = -9936,
        ICReturnDownloadCanceled = -9937,
        ICReturnFailedToEnabeTethering = -9938,
        ICReturnFailedToDisabeTethering = -9939,
        ICReturnFailedToCompleteSendMessageRequest = -9940,
        ICReturnDeleteFilesFailed = -9941,
        ICReturnDeleteFilesCanceled = -9942,
        ICReturnDeviceIsPasscodeLocked = -9943,
        ICReturnDeviceFailedToTakePicture = -9944,
        ICReturnDeviceSoftwareNotInstalled = -9945,
        ICReturnDeviceSoftwareIsBeingInstalled = -9946,
        ICReturnDeviceSoftwareInstallationCompleted = -9947,
        ICReturnDeviceSoftwareInstallationCanceled = -9948,
        ICReturnDeviceSoftwareInstallationFailed = -9949,
        ICReturnDeviceSoftwareNotAvailable = -9950,
        ICReturnDeviceCouldNotPair = -9951,
        ICReturnDeviceCouldNotUnpair = -9952,
        ICReturnDeviceNeedsCredentials = -9953,
        ICReturnDeviceIsBusyEnumerating = -9954,
        ICReturnDeviceCommandGeneralFailure = -9955,
    }
}
