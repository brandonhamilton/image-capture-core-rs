use cocoa::base::{id, BOOL};
use cocoa::foundation::{NSRect, NSSize, NSUInteger};
use crate::constants::ICEXIFOrientationType;
use core_graphics::base::CGFloat;
use core_graphics::image::CGImageRef;
use libc::c_uchar;
use objc::*;

/// Scanner Functional Unit Types
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICScannerFunctionalUnitType {
    /// Flatbed functional unit.
    ICScannerFunctionalUnitTypeFlatbed = 0,
    /// Transparency functional unit for scanning positives.
    ICScannerFunctionalUnitTypePositiveTransparency = 1,
    /// Transparency functional unit for scanning negatives.
    ICScannerFunctionalUnitTypeNegativeTransparency = 2,
    /// Document feeder functional unit.
    ICScannerFunctionalUnitTypeDocumentFeeder = 3,
}

/// Unit of measurement used by the scanner.
/// This corresponds to values used for ICAP_UNITS as defined in the TWAIN Specification.
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICScannerMeasurementUnit {
    ICScannerMeasurementUnitInches = 0,
    ICScannerMeasurementUnitCentimeters = 1,
    ICScannerMeasurementUnitPicas = 2,
    ICScannerMeasurementUnitPoints = 3,
    ICScannerMeasurementUnitTwips = 4,
    ICScannerMeasurementUnitPixels = 5,
}

/// Bits per channel in the scanned image.
/// This corresponds to values used for ICAP_UNITS as defined in the TWAIN Specification.
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICScannerBitDepth {
    ICScannerBitDepth1Bit = 1,
    ICScannerBitDepth8Bits = 8,
    ICScannerBitDepth16Bits = 16,
}

/// Bits per channel in the scanned image.
/// This corresponds to values used for ICAP_UNITS as defined in the TWAIN Specification.
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICScannerColorDataFormatType {
    /// For multi-channel data (e.g., RGB) data from all channels are interleaved.
    ICScannerColorDataFormatTypeChunky = 0,
    /// For multi-channel data (e.g., RGB) each channel is transferred sequentially.
    ICScannerColorDataFormatTypePlanar = 1,
}

/// Pixel data types.
/// Corresponds to "ICAP_PIXELTYPE" of the TWAIN Specification.
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICScannerPixelDataType {
    /// Monochrome 1 bit pixel image.
    ICScannerPixelDataTypeBW = 0,
    /// 8 bit pixel Gray color space.
    ICScannerPixelDataTypeGray = 1,
    /// Color image RGB color space.
    ICScannerPixelDataTypeRGB = 2,
    /// Indexed Color image.
    ICScannerPixelDataTypePalette = 3,
    /// Color image in CMY color space.
    ICScannerPixelDataTypeCMY = 4,
    /// Color image in CMYK color space.
    ICScannerPixelDataTypeCMYK = 5,
    /// Color image in YUV color space.
    ICScannerPixelDataTypeYUV = 6,
    /// Color image in YUVK color space.
    ICScannerPixelDataTypeYUVK = 7,
    /// Color image in CIEXYZ color space.
    ICScannerPixelDataTypeCIEXYZ = 8,
}

/// Document size types.
/// Corresponds to "ICAP_SUPPORTEDSIZES" used by the Image Catpure scanner modules.
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICScannerDocumentType {
    ICScannerDocumentTypeDefault = 0,
    ICScannerDocumentTypeA4 = 1,
    ICScannerDocumentTypeB5 = 2,
    ICScannerDocumentTypeUSLetter = 3,
    ICScannerDocumentTypeUSLegal = 4,
    ICScannerDocumentTypeA5 = 5,
    ICScannerDocumentTypeISOB4 = 6,
    ICScannerDocumentTypeISOB6 = 7,
    ICScannerDocumentTypeUSLedger = 9,
    ICScannerDocumentTypeUSExecutive = 10,
    ICScannerDocumentTypeA3 = 11,
    ICScannerDocumentTypeISOB3 = 12,
    ICScannerDocumentTypeA6 = 13,
    ICScannerDocumentTypeC4 = 14,
    ICScannerDocumentTypeC5 = 15,
    ICScannerDocumentTypeC6 = 16,
    ICScannerDocumentType4A0 = 17,
    ICScannerDocumentType2A0 = 18,
    ICScannerDocumentTypeA0 = 19,
    ICScannerDocumentTypeA1 = 20,
    ICScannerDocumentTypeA2 = 21,
    ICScannerDocumentTypeA7 = 22,
    ICScannerDocumentTypeA8 = 23,
    ICScannerDocumentTypeA9 = 24,
    ICScannerDocumentType10 = 25,
    ICScannerDocumentTypeISOB0 = 26,
    ICScannerDocumentTypeISOB1 = 27,
    ICScannerDocumentTypeISOB2 = 28,
    ICScannerDocumentTypeISOB5 = 29,
    ICScannerDocumentTypeISOB7 = 30,
    ICScannerDocumentTypeISOB8 = 31,
    ICScannerDocumentTypeISOB9 = 32,
    ICScannerDocumentTypeISOB10 = 33,
    ICScannerDocumentTypeJISB0 = 34,
    ICScannerDocumentTypeJISB1 = 35,
    ICScannerDocumentTypeJISB2 = 36,
    ICScannerDocumentTypeJISB3 = 37,
    ICScannerDocumentTypeJISB4 = 38,
    ICScannerDocumentTypeJISB6 = 39,
    ICScannerDocumentTypeJISB7 = 40,
    ICScannerDocumentTypeJISB8 = 41,
    ICScannerDocumentTypeJISB9 = 42,
    ICScannerDocumentTypeJISB10 = 43,
    ICScannerDocumentTypeC0 = 44,
    ICScannerDocumentTypeC1 = 45,
    ICScannerDocumentTypeC2 = 46,
    ICScannerDocumentTypeC3 = 47,
    ICScannerDocumentTypeC7 = 48,
    ICScannerDocumentTypeC8 = 49,
    ICScannerDocumentTypeC9 = 50,
    ICScannerDocumentTypeC10 = 51,
    ICScannerDocumentTypeUSStatement = 52,
    ICScannerDocumentTypeBusinessCard = 53,
    ICScannerDocumentTypeE = 60,
    ICScannerDocumentType3R = 61,
    ICScannerDocumentType4R = 62,
    ICScannerDocumentType5R = 63,
    ICScannerDocumentType6R = 64,
    ICScannerDocumentType8R = 65,
    ICScannerDocumentTypeS8R = 66,
    ICScannerDocumentType10R = 67,
    ICScannerDocumentTypeS10R = 68,
    ICScannerDocumentType11R = 69,
    ICScannerDocumentType12R = 70,
    ICScannerDocumentTypeS12R = 71,
    ICScannerDocumentType110 = 72,
    ICScannerDocumentTypeAPSH = 73,
    ICScannerDocumentTypeAPSC = 74,
    ICScannerDocumentTypeAPSP = 75,
    ICScannerDocumentType135 = 76,
    ICScannerDocumentTypeMF = 77,
    ICScannerDocumentTypeLF = 78,
}

/// A flag to indicate the scanner functional unit's state
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICScannerFunctionalUnitState {
    /// The scanner functional unit is ready for operation.
    ICScannerFunctionalUnitStateReady = (1 << 0),
    /// The scanner functional unit is performing a scan.
    ICScannerFunctionalUnitStateScanInProgress = (1 << 1),
    /// The scanner functional unit is performing an overview scan.
    ICScannerFunctionalUnitStateOverviewScanInProgress = (1 << 2),
}

/// Scanner Feature Types
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICScannerFeatureType {
    /// This feature can have one of several discrete values, strings or numbers.
    ICScannerFeatureTypeEnumeration = 0,
    /// This value of this feature lies within a range.
    ICScannerFeatureTypeRange = 1,
    /// The value of this feature can be YES or NO.
    ICScannerFeatureTypeBoolean = 2,
    ICScannerFeatureTypeTemplate = 3,
}

/// ICScannerFeature class is an abstract base class used to describe a scanner feature.
pub trait ICScannerFeature: Sized {
    /// Scanner feature type.
    unsafe fn type_(self) -> ICScannerFeatureType;
    /// The internal name of this feature.
    unsafe fn internalName(self) -> id;
    /// The human readable name of this feature.
    unsafe fn humanReadableName(self) -> id;
    /// Tooltip text describing the feature.
    unsafe fn tooltip(self) -> id;
}

impl ICScannerFeature for id {
    unsafe fn type_(self) -> ICScannerFeatureType {
        msg_send![self, type]
    }

    unsafe fn internalName(self) -> id {
        msg_send![self, internalName]
    }
    unsafe fn humanReadableName(self) -> id {
        msg_send![self, humanReadableName]
    }
    unsafe fn tooltip(self) -> id {
        msg_send![self, tooltip]
    }
}

/// ICScannerFeature class is an abstract base class used to describe a scanner feature.
pub trait ICScannerFeatureEnumeration: Sized {
    /// The current value. The current value can be set to one of the possible values in the "values" property below￼.
    unsafe fn currentValue(self) -> id;
    /// Set the current value
    unsafe fn setCurrentValue(self, currentValue: id);
    /// The default value. The default value can be set to one of the possible values in the "values" property below.
    unsafe fn defaultValue(self) -> id;
    /// An array of possible values. All items in this array must be of same type￼.
    unsafe fn values(self) -> id;
    /// The human readable menu item labels to be used in a menu to allow the user to select the current value from an array of possible values.
    unsafe fn menuItemLabels(self) -> id;
    /// Tooltip text associated with the menu items.
    unsafe fn menuItemLabelsTooltips(self) -> id;
}

impl ICScannerFeatureEnumeration for id {
    unsafe fn currentValue(self) -> id {
        msg_send![self, currentValue]
    }

    unsafe fn setCurrentValue(self, currentValue: id) {
        msg_send![self, setCurrentValue: currentValue]
    }

    unsafe fn defaultValue(self) -> id {
        msg_send![self, defaultValue]
    }

    unsafe fn values(self) -> id {
        msg_send![self, values]
    }

    unsafe fn menuItemLabels(self) -> id {
        msg_send![self, menuItemLabels]
    }

    unsafe fn menuItemLabelsTooltips(self) -> id {
        msg_send![self, menuItemLabelsTooltips]
    }
}

/// ICScannerFeatureRange object is used to represent a property of a scanner functional unit whose value lies within a range.
pub trait ICScannerFeatureRange: Sized {
    /// The current value. Attempting to set the current value to a value that is not coincident with a step will result in a value corresponding to the nearest step being assigned to the current value.
    unsafe fn currentValue(self) -> CGFloat;
    /// Set the current value
    unsafe fn setCurrentValue(self, currentValue: CGFloat);
    /// The default value￼. Attempting to set the default value to a value that is not coincident with a step will result in a value corresponding to the nearest step being assigned to the default value.
    unsafe fn defaultValue(self) -> CGFloat;
    /// The minimum value.
    unsafe fn minValue(self) -> CGFloat;
    /// The maximum value.
    unsafe fn maxValue(self) -> CGFloat;
    ///  The step size.
    unsafe fn stepSize(self) -> CGFloat;
}

impl ICScannerFeatureRange for id {
    unsafe fn currentValue(self) -> CGFloat {
        msg_send![self, currentValue]
    }

    unsafe fn setCurrentValue(self, currentValue: CGFloat) {
        msg_send![self, setCurrentValue: currentValue]
    }

    unsafe fn defaultValue(self) -> CGFloat {
        msg_send![self, defaultValue]
    }

    unsafe fn minValue(self) -> CGFloat {
        msg_send![self, minValue]
    }

    unsafe fn maxValue(self) -> CGFloat {
        msg_send![self, maxValue]
    }

    unsafe fn stepSize(self) -> CGFloat {
        msg_send![self, stepSize]
    }
}

/// ICScannerFeatureBoolean object is used to represent a property of a scanner functional unit whose value can be YES or NO.
pub trait ICScannerFeatureBoolean: Sized {
    /// The value of this feature.
    unsafe fn value(self) -> BOOL;
    /// Set the value of this feature.
    unsafe fn setValue(self, value: BOOL);
}

impl ICScannerFeatureBoolean for id {
    unsafe fn value(self) -> BOOL {
        msg_send![self, value]
    }

    unsafe fn setValue(self, value: BOOL) {
        msg_send![self, setValue: value]
    }
}

/// ICScannerFeatureTemplate object is used to define a group of one or more rectangular scan areas that can be used with a scanner functional unit.
pub trait ICScannerFeatureTemplate: Sized {
    unsafe fn targets(self) -> id;
}

impl ICScannerFeatureTemplate for id {
    unsafe fn targets(self) -> id {
        msg_send![self, targets]
    }
}

/// ICScannerFunctionalUnit is an abstract class that represents a scanner functiona unit.
/// ImageCaptureCore defines three concrete subclasses of ICScannerFunctionalUnit: ICScannerFunctionalUnitFlatbed, ICScannerFunctionalUnitPositiveTransparency, ICScannerFunctionalUnitNegativeTransparency and ICScannerFunctionalUnitDocumentFeeder.
/// ICScannerDevice creates instances of these concrete subclasses.
pub trait ICScannerFunctionalUnit: Sized {
    /// Functional unit type.
    unsafe fn type_(self) -> ICScannerFunctionalUnitType;
    /// The pixel data type.
    unsafe fn pixelDataType(self) -> ICScannerPixelDataType;
    /// Set the pixel data type.
    unsafe fn setPixelDataType(self, pixelDataType: ICScannerPixelDataType);
    /// Supported bit depths. The values in this set are valid values defined by ICScannerBitDepth.
    unsafe fn supportedBitDepths(self) -> id;
    /// The bit depth to use when performing the final scan. This will always be one of the supported bit depths.
    unsafe fn bitDepth(self) -> ICScannerBitDepth;
    /// Set the bit depth to use when performing the final scan.
    unsafe fn setBitDepth(self, bitDepth: ICScannerBitDepth);
    /// Supported measurement units. The values in this set are valid values defined by ICScannerMeasurementUnit.
    unsafe fn supportedMeasurementUnits(self) -> id;
    /// Current measurement unit. This will always be one of the supported measurement units.
    unsafe fn measurementUnit(self) -> ICScannerMeasurementUnit;
    /// Set current measurement unit.
    unsafe fn setMeasurementUnit(self, measurementUnit: ICScannerMeasurementUnit);
    /// Supported scan resolutions in DPI.
    unsafe fn supportedResolutions(self) -> id;
    /// Preferred scan resolutions in DPI.
    unsafe fn preferredResolutions(self) -> id;
    /// Current scan resolution. This will always be one of the supported resolution values.
    unsafe fn resolution(self) -> NSUInteger;
    /// Set the current scan resolution.
    unsafe fn setResolution(self, resolution: NSUInteger);
    /// Optical resolution along the X axis.
    unsafe fn nativeXResolution(self) -> NSUInteger;
    /// ￼Optical resolution along the Y axis.
    unsafe fn nativeYResolution(self) -> NSUInteger;
    /// Supported scale factors in percentage.
    unsafe fn supportedScaleFactors(self) -> id;
    /// Preferred scale factors in percentage.
    unsafe fn preferredScaleFactors(self) -> id;
    /// Current scale factor. This will always be one of the supported scale factor values.
    unsafe fn scaleFactor(self) -> NSUInteger;
    /// Set the current scale factor.
    unsafe fn setScaleFactor(self, scaleFactor: NSUInteger);
    /// An array of objects of type ICScannerFeatureTemplate.
    unsafe fn templates(self) -> id;
    /// An array of objects of type ICScannerFeature.
    unsafe fn vendorFeatures(self) -> id;
    /// Physical size of the scan area in current measurement unit.
    unsafe fn physicalSize(self) -> NSSize;
    /// This property along with scanAreaOrientation describes the area to be scanned.
    unsafe fn scanArea(self) -> NSRect;
    /// Set the area to be scanned.
    unsafe fn setScanArea(self, scanArea: NSRect);
    /// Desired orientation of the scan area. This property along with scanArea describes the area to be scanned.
    unsafe fn scanAreaOrientation(self) -> ICEXIFOrientationType;
    /// Set the orientation of the scan area.
    unsafe fn setScanAreaOrientation(self, scanAreaOrientation: ICEXIFOrientationType);
    /// Indicates if this functional unit accepts threshold value to be used when performing a scan in black & white.
    unsafe fn acceptsThresholdForBlackAndWhiteScanning(self) -> BOOL;
    /// Indicates if this functional unit uses threshold value to be used when performing a scan in black & white.
    unsafe fn usesThresholdForBlackAndWhiteScanning(self) -> BOOL;
    /// Indicate if this functional unit uses threshold value to be used when performing a scan in black & white.
    unsafe fn setUsesThresholdForBlackAndWhiteScanning(
        self,
        usesThresholdForBlackAndWhiteScanning: BOOL,
    );
    /// Default threshold value used when performing a scan in black & white. This value is from 0 to 255.
    unsafe fn defaultThresholdForBlackAndWhiteScanning(self) -> c_uchar;
    /// Threshold value to be used when performing a scan in black & white. This value should be from 0 to 255.
    unsafe fn thresholdForBlackAndWhiteScanning(self) -> c_uchar;
    /// Set the threshold value to be used when performing a scan in black & white.
    unsafe fn setThresholdForBlackAndWhiteScanning(
        self,
        thresholdForBlackAndWhiteScanning: c_uchar,
    );
    /// Indicates the current state of the functional unit.
    unsafe fn state(self) -> ICScannerFunctionalUnitState;
    /// Indicates if a scan is in progress.
    unsafe fn scanInProgress(self) -> BOOL;
    /// Indicates percentage of scan completed.
    unsafe fn scanProgressPercentDone(self) -> CGFloat;
    /// Indicates if this functional unit can perfrom an overview scan. Not all functional units can perform an overview scan. For example, a document feeder or a sheet feeder unit cannot perform an overview scan.
    unsafe fn canPerformOverviewScan(self) -> BOOL;
    /// Indicates if an overview scan is in progress.
    unsafe fn overviewScanInProgress(self) -> BOOL;
    /// Overview scan image. This property will be NULL for functional units that do not support overview scans.
    unsafe fn overviewImage(self) -> CGImageRef;
    /// Overview image resolution. Value assigned to this will be contrained by resolutions allowed by the device.
    unsafe fn overviewResolution(self) -> NSUInteger;
    /// Set the overview image resolution.
    unsafe fn setOverviewResolution(self, overviewResolution: NSUInteger);
}

impl ICScannerFunctionalUnit for id {
    unsafe fn type_(self) -> ICScannerFunctionalUnitType {
        msg_send![self, type]
    }

    unsafe fn pixelDataType(self) -> ICScannerPixelDataType {
        msg_send![self, pixelDataType]
    }

    unsafe fn setPixelDataType(self, pixelDataType: ICScannerPixelDataType) {
        msg_send![self, setPixelDataType: pixelDataType]
    }

    unsafe fn supportedBitDepths(self) -> id {
        msg_send![self, supportedBitDepths]
    }

    unsafe fn bitDepth(self) -> ICScannerBitDepth {
        msg_send![self, bitDepth]
    }

    unsafe fn setBitDepth(self, bitDepth: ICScannerBitDepth) {
        msg_send![self, setBitDepth: bitDepth]
    }

    unsafe fn supportedMeasurementUnits(self) -> id {
        msg_send![self, supportedMeasurementUnits]
    }

    unsafe fn measurementUnit(self) -> ICScannerMeasurementUnit {
        msg_send![self, measurementUnit]
    }

    unsafe fn setMeasurementUnit(self, measurementUnit: ICScannerMeasurementUnit) {
        msg_send![self, setMeasurementUnit: measurementUnit]
    }

    unsafe fn supportedResolutions(self) -> id {
        msg_send![self, supportedResolutions]
    }

    unsafe fn preferredResolutions(self) -> id {
        msg_send![self, preferredResolutions]
    }

    unsafe fn resolution(self) -> NSUInteger {
        msg_send![self, resolution]
    }

    unsafe fn setResolution(self, resolution: NSUInteger) {
        msg_send![self, setResolution: resolution]
    }

    unsafe fn nativeXResolution(self) -> NSUInteger {
        msg_send![self, nativeXResolution]
    }

    unsafe fn nativeYResolution(self) -> NSUInteger {
        msg_send![self, nativeYResolution]
    }

    unsafe fn supportedScaleFactors(self) -> id {
        msg_send![self, supportedScaleFactors]
    }

    unsafe fn preferredScaleFactors(self) -> id {
        msg_send![self, preferredScaleFactors]
    }

    unsafe fn scaleFactor(self) -> NSUInteger {
        msg_send![self, scaleFactor]
    }

    unsafe fn setScaleFactor(self, scaleFactor: NSUInteger) {
        msg_send![self, setScaleFactor: scaleFactor]
    }

    unsafe fn templates(self) -> id {
        msg_send![self, templates]
    }

    unsafe fn vendorFeatures(self) -> id {
        msg_send![self, vendorFeatures]
    }

    unsafe fn physicalSize(self) -> NSSize {
        msg_send![self, physicalSize]
    }

    unsafe fn scanArea(self) -> NSRect {
        msg_send![self, scanArea]
    }

    unsafe fn setScanArea(self, scanArea: NSRect) {
        msg_send![self, setScanArea: scanArea]
    }

    unsafe fn scanAreaOrientation(self) -> ICEXIFOrientationType {
        msg_send![self, scanAreaOrientation]
    }

    unsafe fn setScanAreaOrientation(self, scanAreaOrientation: ICEXIFOrientationType) {
        msg_send![self, setScanAreaOrientation: scanAreaOrientation]
    }

    unsafe fn acceptsThresholdForBlackAndWhiteScanning(self) -> BOOL {
        msg_send![self, acceptsThresholdForBlackAndWhiteScanning]
    }

    unsafe fn usesThresholdForBlackAndWhiteScanning(self) -> BOOL {
        msg_send![self, usesThresholdForBlackAndWhiteScanning]
    }

    unsafe fn setUsesThresholdForBlackAndWhiteScanning(
        self,
        usesThresholdForBlackAndWhiteScanning: BOOL,
    ) {
        msg_send![
            self,
            setUsesThresholdForBlackAndWhiteScanning: usesThresholdForBlackAndWhiteScanning
        ]
    }

    unsafe fn defaultThresholdForBlackAndWhiteScanning(self) -> c_uchar {
        msg_send![self, defaultThresholdForBlackAndWhiteScanning]
    }

    unsafe fn thresholdForBlackAndWhiteScanning(self) -> c_uchar {
        msg_send![self, thresholdForBlackAndWhiteScanning]
    }

    unsafe fn setThresholdForBlackAndWhiteScanning(
        self,
        thresholdForBlackAndWhiteScanning: c_uchar,
    ) {
        msg_send![
            self,
            setThresholdForBlackAndWhiteScanning: thresholdForBlackAndWhiteScanning
        ]
    }

    unsafe fn state(self) -> ICScannerFunctionalUnitState {
        msg_send![self, state]
    }

    unsafe fn scanInProgress(self) -> BOOL {
        msg_send![self, scanInProgress]
    }

    unsafe fn scanProgressPercentDone(self) -> CGFloat {
        msg_send![self, scanProgressPercentDone]
    }

    unsafe fn canPerformOverviewScan(self) -> BOOL {
        msg_send![self, canPerformOverviewScan]
    }

    unsafe fn overviewScanInProgress(self) -> BOOL {
        msg_send![self, overviewScanInProgress]
    }

    unsafe fn overviewImage(self) -> CGImageRef {
        msg_send![self, overviewImage]
    }

    unsafe fn overviewResolution(self) -> NSUInteger {
        msg_send![self, overviewResolution]
    }

    unsafe fn setOverviewResolution(self, overviewResolution: NSUInteger) {
        msg_send![self, setOverviewResolution: overviewResolution]
    }
}

/// ICScannerFunctionalUnitFlatbed is a concrete subclass of ICScannerFunctionalUnit class.
/// ICScannerDevice creates instances of this class.
pub trait ICScannerFunctionalUnitFlatbed: Sized {
    /// Supported document types. The values in this set are valid values defined by ICScannerDocumentType.
    unsafe fn supportedDocumentTypes(self) -> id;
    /// Current document type. This will always be one of the supported document types.
    unsafe fn documentType(self) -> ICScannerDocumentType;
    /// Set the current document type.
    unsafe fn setDocumentType(self, documentType: ICScannerDocumentType);
    /// ￼Document size of the current document type expressed in current measurement unit.
    unsafe fn documentSize(self) -> NSSize;
}

impl ICScannerFunctionalUnitFlatbed for id {
    unsafe fn supportedDocumentTypes(self) -> id {
        msg_send![self, supportedDocumentTypes]
    }

    unsafe fn documentType(self) -> ICScannerDocumentType {
        msg_send![self, documentType]
    }

    unsafe fn setDocumentType(self, documentType: ICScannerDocumentType) {
        msg_send![self, setDocumentType: documentType]
    }

    unsafe fn documentSize(self) -> NSSize {
        msg_send![self, documentSize]
    }
}

/// ICScannerFunctionalUnitPositiveTransparency is a concrete subclass of ICScannerFunctionalUnit class.
/// ICScannerDevice creates instances of this class.
pub trait ICScannerFunctionalUnitPositiveTransparency: Sized {
    /// Supported document types. The values in this set are valid values defined by ICScannerDocumentType.
    unsafe fn supportedDocumentTypes(self) -> id;
    /// Current document type. This will always be one of the supported document types.
    unsafe fn documentType(self) -> ICScannerDocumentType;
    /// Set the current document type.
    unsafe fn setDocumentType(self, documentType: ICScannerDocumentType);
    /// ￼Document size of the current document type expressed in current measurement unit.
    unsafe fn documentSize(self) -> NSSize;
}

impl ICScannerFunctionalUnitPositiveTransparency for id {
    unsafe fn supportedDocumentTypes(self) -> id {
        msg_send![self, supportedDocumentTypes]
    }

    unsafe fn documentType(self) -> ICScannerDocumentType {
        msg_send![self, documentType]
    }

    unsafe fn setDocumentType(self, documentType: ICScannerDocumentType) {
        msg_send![self, setDocumentType: documentType]
    }

    unsafe fn documentSize(self) -> NSSize {
        msg_send![self, documentSize]
    }
}

/// ICScannerFunctionalUnitNegativeTransparency is a concrete subclass of ICScannerFunctionalUnit class.
/// ICScannerDevice creates instances of this class.
pub trait ICScannerFunctionalUnitNegativeTransparency: Sized {
    /// Supported document types. The values in this set are valid values defined by ICScannerDocumentType.
    unsafe fn supportedDocumentTypes(self) -> id;
    /// Current document type. This will always be one of the supported document types.
    unsafe fn documentType(self) -> ICScannerDocumentType;
    /// Set the current document type.
    unsafe fn setDocumentType(self, documentType: ICScannerDocumentType);
    /// ￼Document size of the current document type expressed in current measurement unit.
    unsafe fn documentSize(self) -> NSSize;
}

impl ICScannerFunctionalUnitNegativeTransparency for id {
    unsafe fn supportedDocumentTypes(self) -> id {
        msg_send![self, supportedDocumentTypes]
    }

    unsafe fn documentType(self) -> ICScannerDocumentType {
        msg_send![self, documentType]
    }

    unsafe fn setDocumentType(self, documentType: ICScannerDocumentType) {
        msg_send![self, setDocumentType: documentType]
    }

    unsafe fn documentSize(self) -> NSSize {
        msg_send![self, documentSize]
    }
}

/// ICScannerFunctionalUnitDocumentFeeder is a concrete subclass of ICScannerFunctionalUnit class.
/// ICScannerDevice creates instances of this class.
pub trait ICScannerFunctionalUnitDocumentFeeder: Sized {
    /// Supported document types. The values in this set are valid values defined by ICScannerDocumentType.
    unsafe fn supportedDocumentTypes(self) -> id;
    /// Current document type. This will always be one of the supported document types.
    unsafe fn documentType(self) -> ICScannerDocumentType;
    /// Set the current document type.
    unsafe fn setDocumentType(self, documentType: ICScannerDocumentType);
    /// ￼Document size of the current document type expressed in current measurement unit.
    unsafe fn documentSize(self) -> NSSize;
    /// Indicates whether duplex scanning is supported.
    unsafe fn supportsDuplexScanning(self) -> BOOL;
    /// Indicates whether duplex scanning is enabled.
    unsafe fn duplexScanningEnabled(self) -> BOOL;
    /// Set whether duplex scanning is enabled.
    unsafe fn setDuplexScanningEnabled(self, duplexScanningEnabled: BOOL);
    /// Indicates whether the feeder has documents to scan.
    /// This value will change when the document is loaded or removed from the feeder, if the scanner module has the capability to detect this state.
    unsafe fn documentLoaded(self) -> BOOL;
    /// Desired orientation of the odd pages of the scanned document.
    unsafe fn oddPageOrientation(self) -> ICEXIFOrientationType;
    /// Set the desired orientation of the odd pages of the scanned document.
    unsafe fn setOddPageOrientation(self, oddPageOrientation: ICEXIFOrientationType);
    /// Desired orientation of the even pages of the scanned document.
    unsafe fn evenPageOrientation(self) -> ICEXIFOrientationType;
    /// Set the desired orientation of the even pages of the scanned document.
    unsafe fn setEvenPageOrientation(self, evenPageOrientation: ICEXIFOrientationType);
    /// Indicates whether the document feeder reads pages from back to front.
    unsafe fn reverseFeederPageOrder(self) -> BOOL;
}

impl ICScannerFunctionalUnitDocumentFeeder for id {
    unsafe fn supportedDocumentTypes(self) -> id {
        msg_send![self, supportedDocumentTypes]
    }

    unsafe fn documentType(self) -> ICScannerDocumentType {
        msg_send![self, documentType]
    }

    unsafe fn setDocumentType(self, documentType: ICScannerDocumentType) {
        msg_send![self, setDocumentType: documentType]
    }

    unsafe fn documentSize(self) -> NSSize {
        msg_send![self, documentSize]
    }

    unsafe fn supportsDuplexScanning(self) -> BOOL {
        msg_send![self, supportsDuplexScanning]
    }

    unsafe fn duplexScanningEnabled(self) -> BOOL {
        msg_send![self, duplexScanningEnabled]
    }

    unsafe fn setDuplexScanningEnabled(self, duplexScanningEnabled: BOOL) {
        msg_send![self, setDuplexScanningEnabled: duplexScanningEnabled]
    }

    unsafe fn documentLoaded(self) -> BOOL {
        msg_send![self, documentLoaded]
    }

    unsafe fn oddPageOrientation(self) -> ICEXIFOrientationType {
        msg_send![self, oddPageOrientation]
    }

    unsafe fn setOddPageOrientation(self, oddPageOrientation: ICEXIFOrientationType) {
        msg_send![self, setOddPageOrientation: oddPageOrientation]
    }

    unsafe fn evenPageOrientation(self) -> ICEXIFOrientationType {
        msg_send![self, evenPageOrientation]
    }

    unsafe fn setEvenPageOrientation(self, evenPageOrientation: ICEXIFOrientationType) {
        msg_send![self, setEvenPageOrientation: evenPageOrientation]
    }

    unsafe fn reverseFeederPageOrder(self) -> BOOL {
        msg_send![self, reverseFeederPageOrder]
    }
}
