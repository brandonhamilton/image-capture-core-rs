use cocoa::base::id;
use objc::*;

/// Transfer mode to be used when transferring scan data from the scanner functional unit.
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICScannerTransferMode {
    /// Save the scan as a file.
    ICScannerTransferModeFileBased = 0,
    /// Transfer the scan as data.
    ICScannerTransferModeMemoryBased = 1,
}

pub trait ICScannerDevice: Sized {
    /// An array of functional unit types available on this scanner device.
    /// This is an array of NSNumber objects whose values are of type ICScannerFunctionalUnitType.
    unsafe fn availableFunctionalUnitTypes(self) -> id;
    /// The currently selected functional unit on the scanner device.
    unsafe fn selectedFunctionalUnit(self) -> id;
    /// The transfer mode for scanned document.
    unsafe fn transferMode(self) -> ICScannerTransferMode;
    /// Set the transfer mode for scanned document.
    unsafe fn setTransferMode(self, transferMode: ICScannerTransferMode);
    /// ￼The total maximum band size requested when performing a ICScannerTransferModeMemoryBased.
    unsafe fn maxMemoryBandSize(self) -> u32;
    /// ￼Set the total maximum band size requested when performing a ICScannerTransferModeMemoryBased.
    unsafe fn setMaxMemoryBandSize(self, maxMemoryBandSize: u32);
    /// The downloads directory.
    unsafe fn downloadsDirectory(self) -> id;
    /// Set the downloads directory.
    unsafe fn setDownloadsDirectory(self, downloadsDirectory: id);
    /// The document name.
    unsafe fn documentName(self) -> id;
    /// Set the document name.
    unsafe fn setDocumentName(self, documentName: id);
    /// The document UTI.
    /// Currently supported UTIs are: kUTTypeJPEG, kUTTypeJPEG2000, kUTTypeTIFF, kUTTypePNG etc.
    unsafe fn documentUTI(self) -> id;
    /// Set the document UTI.
    unsafe fn setDocumentUTI(self, documentUTI: id);
    /// If the device is protected, instead of prompting the user for a username, this property can be set to default to a specific username as a convience.
    /// The value will persist until reset by setting it to nil.
    unsafe fn defaultUsername(self) -> id;
    /// Set the default username
    unsafe fn setDefaultUsername(self, defaultUsername: id);
    /// This message requests to open a session on the protected device with the authorized username and passcode.
    /// If the device reports back a failure of credentials, they can be provided here for the launch.
    /// A client MUST open a session on a device in order to use the device.
    unsafe fn requestOpenSessionWithCredentials(self, username: id, password: id);
    /// Requests the scanner device to select a functional unit.
    unsafe fn requestSelectFunctionalUnit(self, type_: id);
    /// Starts an overview scan on selectedFunctionalUnit.
    unsafe fn requestOverviewScan(self);
    /// Starts a scan on selectedFunctionalUnit.
    unsafe fn requestScan(self);
    /// cancelScan
    unsafe fn cancelScan(self);
}

impl ICScannerDevice for id {
    unsafe fn availableFunctionalUnitTypes(self) -> id {
        msg_send![self, availableFunctionalUnitTypes]
    }

    unsafe fn selectedFunctionalUnit(self) -> id {
        msg_send![self, selectedFunctionalUnit]
    }

    unsafe fn transferMode(self) -> ICScannerTransferMode {
        msg_send![self, transferMode]
    }

    unsafe fn setTransferMode(self, transferMode: ICScannerTransferMode) {
        msg_send![self, setTransferMode: transferMode]
    }

    unsafe fn maxMemoryBandSize(self) -> u32 {
        msg_send![self, maxMemoryBandSize]
    }

    unsafe fn setMaxMemoryBandSize(self, maxMemoryBandSize: u32) {
        msg_send![self, setMaxMemoryBandSize: maxMemoryBandSize]
    }

    unsafe fn downloadsDirectory(self) -> id {
        msg_send![self, downloadsDirectory]
    }

    unsafe fn setDownloadsDirectory(self, downloadsDirectory: id) {
        msg_send![self, setDownloadsDirectory: downloadsDirectory]
    }

    unsafe fn documentName(self) -> id {
        msg_send![self, documentName]
    }

    unsafe fn setDocumentName(self, documentName: id) {
        msg_send![self, setDocumentName: documentName]
    }

    unsafe fn documentUTI(self) -> id {
        msg_send![self, documentUTI]
    }

    unsafe fn setDocumentUTI(self, documentUTI: id) {
        msg_send![self, setDocumentUTI: documentUTI]
    }

    unsafe fn defaultUsername(self) -> id {
        msg_send![self, defaultUsername]
    }

    unsafe fn setDefaultUsername(self, defaultUsername: id) {
        msg_send![self, setDefaultUsername: defaultUsername]
    }

    unsafe fn requestOpenSessionWithCredentials(self, username: id, password: id) {
        msg_send![self, requestOpenSessionWithCredentials:username password:password]
    }

    unsafe fn requestSelectFunctionalUnit(self, type_: id) {
        msg_send![self, requestSelectFunctionalUnit: type_]
    }

    unsafe fn requestOverviewScan(self) {
        msg_send![self, requestOverviewScan]
    }

    unsafe fn requestScan(self) {
        msg_send![self, requestScan]
    }

    unsafe fn cancelScan(self) {
        msg_send![self, cancelScan]
    }
}

#[link(name = "ImageCaptureCore", kind = "framework")]
extern "C" {
    // Constants used for device status notifications.

    /// A non-localized notification string to indicate that the scanner is warming up.
    pub static ICScannerStatusWarmingUp: id;
    /// A non-localized notification string to indicate that the scanner has warmed up.
    pub static ICScannerStatusWarmUpDone: id;
    /// A non-localized notification string to indicate that the scanner is requesting an overview scan to be performed.
    pub static ICScannerStatusRequestsOverviewScan: id;
}
