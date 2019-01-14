use bitflags::bitflags;
use cocoa::base::{id, BOOL};
use cocoa::foundation::NSUInteger;
use core_graphics::image::CGImageRef;
use libc::{c_int, c_longlong};
use objc::*;

/// Image Capture Device Types
bitflags! {
    pub struct ICDeviceType: NSUInteger {
        /// Camera device.
        const ICDeviceTypeCamera = 0x00000001;
        /// Scanner device.
        const ICDeviceTypeScanner = 0x00000002;
    }
}

/// Image Capture Device Location Types
bitflags! {
    pub struct ICDeviceLocationType: NSUInteger {
        /// Device found directly attached to the Macintosh via its USB or FireWire port.
        const ICDeviceLocationTypeLocal = 0x00000100;
        /// Device found over the network by searching for devices shared by other Macintosh hosts.
        const ICDeviceLocationTypeShared = 0x00000200;
        /// Device found over the network by searching for Bonjour services supported by Image Capture.
        const ICDeviceLocationTypeBonjour = 0x00000400;
        /// Device found as a paired Bluetooth device.
        const ICDeviceLocationTypeBluetooth = 0x00000800;
    }
}

/// Image Capture Device Type Mask
bitflags! {
    pub struct ICDeviceTypeMask: NSUInteger {
        /// Mask to detect a camera device.
        const ICDeviceTypeMaskCamera = 0x00000001;
        /// Mask to detect a scanner device.
        const ICDeviceTypeMaskScanner = 0x00000002;
    }
}

/// Image Capture Device Location Type Mask
bitflags! {
    pub struct ICDeviceLocationTypeMask: NSUInteger {
        /// Mask to detect a local (e.g., USB or FireWire) device.
        const ICDeviceLocationTypeMaskLocal = 0x00000100;
        /// Mask to detect a device by another Macintosh host.
        const ICDeviceLocationTypeMaskShared = 0x00000200;
        /// Mask to detect a network device that publishes a Bonjour service.
        const ICDeviceLocationTypeMaskBonjour = 0x00000400;
        /// Mask to detect paired Bluetooth device.
        const ICDeviceLocationTypeMaskBluetooth = 0x00000800;
        /// Mask to detect a remote (shared, Bonjour, Bluetooth) device.
        const ICDeviceLocationTypeMaskRemote = 0x0000FE00;
    }
}

pub trait ICDevice: Sized {
    /// Get the delegate.
    unsafe fn delegate(self) -> id;
    /// Set the delegate.
    unsafe fn setDelegate(self, delegate: id);
    /// The type of the device as defined by ICDeviceType OR'd with its ICDeviceLocationType.
    /// The type of this device can be obtained by AND'ing the value retuned by this property with an appropriate ICDeviceTypeMask.
    /// The location type of this device can be obtained by AND'ing the value retuned by this property with an appropriate ICDeviceLocationTypeMask.
    unsafe fn type_(self) -> ICDeviceType;
    /// Name of the device as reported by the device module or by the device transport when a device module is not in control of this device.
    /// This name may change if the device module overrides the default name of the device reported by the device's transport, or if the name of the filesystem volume mounted by the device is changed by the user.
    unsafe fn name(self) -> id;
    /// Icon image for the device.
    unsafe fn icon(self) -> CGImageRef;
    /// ￼The capabilities of the device as reported by the device module.
    unsafe fn capabilities(self) -> id;
    /// Filesystem path of the device module that is associated with this device.
    unsafe fn modulePath(self) -> id;
    ///The bundle version of the device module associated with this device.
    unsafe fn moduleVersion(self) -> id;
    /// Indicates whether the device is a remote device published by Image Capture device sharing facility.
    unsafe fn isRemote(self) -> BOOL;
    ///  ￼The transport type used by the device.
    unsafe fn transportType(self) -> id;
    /// The USB location ID of a USB device in the IOKit registry. This will be 0 for non-USB devices.
    unsafe fn usbLocationID(self) -> c_int;
    /// The USB product ID of a USB device in the IOKit registry. This will be 0 for non-USB devices.
    unsafe fn usbProductID(self) -> c_int;
    /// The USB vendor ID of a USB device in the IOKit registry. This will be 0 for non-USB devices.
    unsafe fn usbVendorID(self) -> c_int;
    /// The FireWire GUID of a FireWire device in the IOKit registry. This will be 0 for non-FireWire devices.
    unsafe fn fwGUID(self) -> c_longlong;
    /// The serial number of the device. This will be NULL if the device does not provide a serial number.
    unsafe fn serialNumberString(self) -> id;
    /// A non-localized location description string for the device.
    unsafe fn locationDescription(self) -> id;
    /// Indicates whether the device has an open session.
    unsafe fn hasOpenSession(self) -> BOOL;
    /// A string representation of the Universally Unique ID of the device.
    unsafe fn UUIDString(self) -> id;
    /// A string representation of the persistent ID of the device.
    unsafe fn persistentIDString(self) -> id;
    /// ￼A string object with one of the ICButtonType values.
    unsafe fn buttonPressed(self) -> id;
    /// Filesystem path of an application that is to be automatically launched when this device is added.
    unsafe fn autolaunchApplicationPath(self) -> id;
    /// Set the filesystem path of an application that is to be automatically launched when this device is added.
    unsafe fn setAutolaunchApplicationPath(self, autolaunchApplicationPath: id);
    /// A mutable dictionary to store arbitrary key-value pairs associated with a device object. This can be used by view objects that bind to this object to store "house-keeping" information.
    unsafe fn userData(self) -> id;
    /// This message requests to open a session on the device.
    unsafe fn requestOpenSession(self);
    /// This message requests to close a previously opened session on this device.
    unsafe fn requestCloseSession(self);
    /// This message requests the device module in control of this device to yield control.
    unsafe fn requestYield(self);
    /// This method asynchronously sends an arbitrary message with optional data to a device.
    unsafe fn requestSendMessage(
        self,
        messageCode: u64,
        outData: id,
        maxReturnedDataSize: u64,
        sendMessageDelegate: id,
        didSendMessageSelector: id,
        contextInfo: id,
    );
    /// Eject the media if permitted by the device, or disconnect from a remote device.
    unsafe fn requestEjectOrDisconnect(self);
}

impl ICDevice for id {
    unsafe fn delegate(self) -> id {
        msg_send![self, delegate]
    }

    unsafe fn setDelegate(self, delegate: id) {
        msg_send![self, setDelegate: delegate]
    }

    unsafe fn type_(self) -> ICDeviceType {
        msg_send![self, type]
    }

    unsafe fn name(self) -> id {
        msg_send![self, name]
    }

    unsafe fn icon(self) -> CGImageRef {
        msg_send![self, icon]
    }

    unsafe fn capabilities(self) -> id {
        msg_send![self, capabilities]
    }

    unsafe fn modulePath(self) -> id {
        msg_send![self, modulePath]
    }

    unsafe fn moduleVersion(self) -> id {
        msg_send![self, moduleVersion]
    }

    unsafe fn isRemote(self) -> BOOL {
        msg_send![self, isRemote]
    }

    unsafe fn transportType(self) -> id {
        msg_send![self, transportType]
    }

    unsafe fn usbLocationID(self) -> c_int {
        msg_send![self, usbLocationID]
    }

    unsafe fn usbProductID(self) -> c_int {
        msg_send![self, usbProductID]
    }

    unsafe fn usbVendorID(self) -> c_int {
        msg_send![self, usbVendorID]
    }

    unsafe fn fwGUID(self) -> c_longlong {
        msg_send![self, fwGUID]
    }

    unsafe fn serialNumberString(self) -> id {
        msg_send![self, serialNumberString]
    }

    unsafe fn locationDescription(self) -> id {
        msg_send![self, locationDescription]
    }

    unsafe fn hasOpenSession(self) -> BOOL {
        msg_send![self, hasOpenSession]
    }

    unsafe fn UUIDString(self) -> id {
        msg_send![self, UUIDString]
    }

    unsafe fn persistentIDString(self) -> id {
        msg_send![self, persistentIDString]
    }

    unsafe fn buttonPressed(self) -> id {
        msg_send![self, buttonPressed]
    }

    unsafe fn autolaunchApplicationPath(self) -> id {
        msg_send![self, autolaunchApplicationPath]
    }

    unsafe fn setAutolaunchApplicationPath(self, autolaunchApplicationPath: id) {
        msg_send![
            self,
            setAutolaunchApplicationPath: autolaunchApplicationPath
        ]
    }

    unsafe fn userData(self) -> id {
        msg_send![self, userData]
    }

    unsafe fn requestOpenSession(self) {
        msg_send![self, requestOpenSession]
    }

    unsafe fn requestCloseSession(self) {
        msg_send![self, requestCloseSession]
    }

    unsafe fn requestYield(self) {
        msg_send![self, requestYield]
    }

    unsafe fn requestSendMessage(
        self,
        messageCode: u64,
        outData: id,
        maxReturnedDataSize: u64,
        sendMessageDelegate: id,
        didSendMessageSelector: id,
        contextInfo: id,
    ) {
        msg_send![self, requestSendMessage:messageCode outData:outData maxReturnedDataSize:maxReturnedDataSize sendMessageDelegate:sendMessageDelegate didSendMessageSelector:didSendMessageSelector contextInfo:contextInfo]
    }

    unsafe fn requestEjectOrDisconnect(self) {
        msg_send![self, requestEjectOrDisconnect]
    }
}

#[link(name = "ImageCaptureCore", kind = "framework")]
extern "C" {
    // Constants used to identify the transport type used by a device. (NSString *const)

    /// Indicates that the device uses USB transport
    pub static ICTransportTypeUSB: id;
    /// Indicates that the device uses FireWire transport.
    pub static ICTransportTypeFireWire: id;
    /// Indicates that the device uses Bluetooth transport.
    pub static ICTransportTypeBluetooth: id;
    /// Indicates that the device uses TCP/IP transport. These devices are discovered using Bonjour.
    pub static ICTransportTypeTCPIP: id;
    /// Indicates that the device use mounts as a mass-storage volume.
    pub static ICTransportTypeMassStorage: id;

    // Constants used for device location description. (NSString *const)

    /// This description is returned for locationDescription property of a device connected to a USB port.
    pub static ICDeviceLocationDescriptionUSB: id;
    /// This description is returned for locationDescription property of a device connected to a FireWire port.
    pub static ICDeviceLocationDescriptionFireWire: id;
    /// This description is returned for locationDescription property of a device connected via Bluetooth.
    pub static ICDeviceLocationDescriptionBluetooth: id;
    /// This description is returned for locationDescription property of a device that is mounted as a mass-storage volume.
    pub static ICDeviceLocationDescriptionMassStorage: id;

    // Constants used to identify button-press on a device.

    ///Indicates that the "Scan" button on the device was pressed.
    pub static ICButtonTypeScan: id;
    /// Indicates that the "Mail" button on the device was pressed.
    pub static ICButtonTypeMail: id;
    /// Indicates that the "Copy" button on the device was pressed.
    pub static ICButtonTypeCopy: id;
    /// Indicates that the "Web" button on the device was pressed.
    pub static ICButtonTypeWeb: id;
    /// Indicates that the "Print" button on the device was pressed.
    pub static ICButtonTypePrint: id;
    /// Indicates that the "Transfer" button on the device was pressed.
    pub static ICButtonTypeTransfer: id;

    // Constants used for device status notifications.

    ///Key for a non-localized notification string.
    pub static ICStatusNotificationKey: id;
    /// One of values defined in ICReturnCode.
    pub static ICStatusCodeKey: id;
    /// Key for a localized notification string.
    pub static ICLocalizedStatusNotificationKey: id;

    // Constants used to describe capabilities of a device

    ///Indicates either the device is mounted as a mass-storage volume and can be ejected or the it is a remote device with an active connection that can be disconnected.
    pub static ICDeviceCanEjectOrDisconnect: id;
}
