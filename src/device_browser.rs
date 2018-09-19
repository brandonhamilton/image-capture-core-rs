use cocoa::base::{id, BOOL};
use cocoa::foundation::NSUInteger;

pub trait ICDeviceBrowser: Sized {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(ICDeviceBrowser), alloc]
    }
    unsafe fn new(_: Self) -> id {
        msg_send![class!(ICDeviceBrowser), new]
    }
    /// Get the delegate.
    unsafe fn delegate(self) -> id;
    /// Set the delegate.
    unsafe fn setDelegate(self, delegate: id);
    /// Indicates whether the device browser is browsing for devices.
    unsafe fn isBrowsing(self) -> BOOL;
    /// A mask whose set bits indicate the type of device(s) being browsed after the receiver receives the start message
    unsafe fn browsedDeviceTypeMask(self) -> NSUInteger;
    /// Set the mask whose set bits indicate the type of device(s) being browsed after the receiver receives the start message
    unsafe fn setBrowsedDeviceTypeMask(self, mask: NSUInteger);
    /// All devices found by the browser. This property will change as devices appear and disappear.
    /// This array is empty before the first invocation of the delegate method 'deviceBrowser:didAddDevice:moreComing:'.
    unsafe fn devices(self) -> id /* (NSArray *) */;
    /// This method returns a device object that should be selected by the client application when it is launched.
    unsafe fn preferredDevice(self) -> id;
    /// This is the designated initializer.
    unsafe fn init(self) -> id;
    /// This message tells the receiver to start looking for devices.
    unsafe fn start(self);
    /// This method tells the receiver to stop looking for devices.
    unsafe fn stop(self);
}

impl ICDeviceBrowser for id {
    unsafe fn delegate(self) -> id {
        msg_send![self, delegate]
    }

    unsafe fn setDelegate(self, delegate: id) {
        msg_send![self, setDelegate: delegate]
    }

    unsafe fn browsedDeviceTypeMask(self) -> NSUInteger {
        msg_send![self, browsedDeviceTypeMask]
    }

    unsafe fn setBrowsedDeviceTypeMask(self, mask: NSUInteger) {
        msg_send![self, setBrowsedDeviceTypeMask: mask]
    }

    unsafe fn isBrowsing(self) -> BOOL {
        msg_send![self, isBrowsing]
    }

    unsafe fn devices(self) -> id /* (NSArray *) */ {
        msg_send![self, devices]
    }

    unsafe fn preferredDevice(self) -> id {
        msg_send![self, preferredDevice]
    }

    unsafe fn init(self) -> id {
        msg_send![self, init]
    }

    unsafe fn start(self) {
        msg_send![self, start]
    }

    unsafe fn stop(self) {
        msg_send![self, stop]
    }
}

#[link(name = "ImageCaptureCore", kind = "framework")]
extern "C" {}
