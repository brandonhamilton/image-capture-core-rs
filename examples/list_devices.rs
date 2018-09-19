extern crate cocoa;
extern crate image_capture_core;
extern crate libc;
#[macro_use]
extern crate objc;

use cocoa::appkit::{
    NSApp, NSApplication, NSApplicationActivateIgnoringOtherApps,
    NSApplicationActivationPolicyRegular, NSRunningApplication,
};
use cocoa::base::{id, nil, BOOL};
use cocoa::foundation::NSAutoreleasePool;
use image_capture_core::device::{ICDevice, ICDeviceLocationTypeMask, ICDeviceTypeMask};
use image_capture_core::device_browser::ICDeviceBrowser;
use objc::declare::ClassDecl;
use objc::runtime::{Object, Sel};
use std::ffi::CStr;

/// Convert an NSString object into a Rust String
pub fn nsstring_decode(str: id) -> String {
    unsafe {
        let cstr: *const libc::c_char = msg_send![str, UTF8String];
        let rstr = CStr::from_ptr(cstr).to_string_lossy().into_owned();
        rstr
    }
}

fn main() {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);

        // Create the device browser delegate
        let superclass = class!(NSObject);
        let mut decl = ClassDecl::new("BrowserDelegate", superclass).unwrap();

        extern "C" fn device_browser_did_add_device(
            _: &Object,
            _: Sel,
            _: id,
            device: id,
            _more_coming: BOOL,
        ) {
            let name = nsstring_decode(unsafe { ICDevice::name(device) });
            println!("Found device '{}'", name);
        }

        extern "C" fn device_browser_did_remove_device(
            _: &Object,
            _: Sel,
            _: id,
            device: id,
            _more_going: BOOL,
        ) {
            let name = nsstring_decode(unsafe { ICDevice::name(device) });
            println!("Device removed '{}'", name);
        }

        decl.add_method(
            sel!(deviceBrowser:didAddDevice:moreComing:),
            device_browser_did_add_device as extern "C" fn(&Object, Sel, id, id, BOOL),
        );
        decl.add_method(
            sel!(deviceBrowser:didRemoveDevice:moreGoing:),
            device_browser_did_remove_device as extern "C" fn(&Object, Sel, id, id, BOOL),
        );

        let delegate_class = decl.register();
        let delegate_object = msg_send![delegate_class, new];

        // Create the device browser
        let browser = ICDeviceBrowser::new(nil).autorelease();
        ICDeviceBrowser::setDelegate(browser, delegate_object);
        let types_mask =
            ICDeviceTypeMask::ICDeviceTypeMaskCamera | ICDeviceTypeMask::ICDeviceTypeMaskScanner;
        let locations_mask = ICDeviceLocationTypeMask::ICDeviceLocationTypeMaskLocal
            | ICDeviceLocationTypeMask::ICDeviceLocationTypeMaskRemote;
        browser.setBrowsedDeviceTypeMask(types_mask.bits() | locations_mask.bits());
        browser.start();

        // Run the application
        let app = NSApp();
        app.setActivationPolicy_(NSApplicationActivationPolicyRegular);
        let current_app = NSRunningApplication::currentApplication(nil);
        current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
        app.run();
    }
}
