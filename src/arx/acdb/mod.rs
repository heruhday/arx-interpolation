pub mod object_id;
pub mod database;
pub mod object;
pub mod handle;
pub mod curve;

use std::os::raw::{c_int, c_void};
use crate::arx::resbuf::_resbuf;
use crate::arx::{ACHAR, ads_real, size_t, ads_name, acrx};
use crate::arx::acdb::object::Object;
use crate::arx::acdb::object_id::ObjectId;
use crate::arx::acarray::Array;
use crate::arx::acdb::database::Database;
use crate::arx::aclocale::AcLocale;
use crate::arx::acad_app::ErrorStatus;
use crate::arx::acgi;

pub type OpenMode = c_int;
pub type DxfCode = i16;
pub type DwgDataType = c_int;
pub type CoordAxis = c_int;
pub type CoordSystem = c_int;
pub type Intersect = c_int;
pub type Visibility = c_int;
pub type OsnapMask = c_int;
pub type DeepCloneType = c_int;
pub type DuplicateRecordCloning = c_int;
pub type DwgVersion = c_int;
pub type MaintenanceReleaseVersion = c_int;

pub mod open_mode{
    use crate::arx::acdb::OpenMode;
    pub const kForRead: OpenMode = 0;
    pub const kForWrite: OpenMode = 1;
    pub const kForNotify: OpenMode = 2;

}
pub mod dxf_code {
    use crate::arx::acdb::DxfCode;
    pub const kDxfInvalid: DxfCode = -9999;
    pub const kDxfXDictionary: DxfCode = -6;
    pub const kDxfPReactors: DxfCode = -5;
    pub const kDxfOperator: DxfCode = -4;
    pub const kDxfXDataStart: DxfCode = -3;
    pub const kDxfHeaderId: DxfCode = -2;
    pub const kDxfFirstEntId: DxfCode = -2;
    pub const kDxfEnd: DxfCode = -1;
    pub const kDxfStart: DxfCode = 0;
    pub const kDxfText: DxfCode = 1;
    pub const kDxfXRefPath: DxfCode = 1;
    pub const kDxfShapeName: DxfCode = 2;
    pub const kDxfBlockName: DxfCode = 2;
    pub const kDxfAttributeTag: DxfCode = 2;
    pub const kDxfSymbolTableName: DxfCode = 2;
    pub const kDxfMstyleName: DxfCode = 2;
    pub const kDxfSymTableRecName: DxfCode = 2;
    pub const kDxfAttributePrompt: DxfCode = 3;
    pub const kDxfDimStyleName: DxfCode = 3;
    pub const kDxfLinetypeProse: DxfCode = 3;
    pub const kDxfTextFontFile: DxfCode = 3;
    pub const kDxfDescription: DxfCode = 3;
    pub const kDxfDimPostStr: DxfCode = 3;
    pub const kDxfTextBigFontFile: DxfCode = 4;
    pub const kDxfDimAPostStr: DxfCode = 4;
    pub const kDxfCLShapeName: DxfCode = 4;
    pub const kDxfSymTableRecComments: DxfCode = 4;
    pub const kDxfHandle: DxfCode = 5;
    pub const kDxfDimBlk: DxfCode = 5;
    pub const kDxfDimBlk1: DxfCode = 6;
    pub const kDxfLinetypeName: DxfCode = 6;
    pub const kDxfDimBlk2: DxfCode = 7;
    pub const kDxfTextStyleName: DxfCode = 7;
    pub const kDxfLayerName: DxfCode = 8;
    pub const kDxfCLShapeText: DxfCode = 9;
    pub const kDxfXCoord: DxfCode = 10;
    pub const kDxfYCoord: DxfCode = 20;
    pub const kDxfZCoord: DxfCode = 30;
    pub const kDxfElevation: DxfCode = 38;
    pub const kDxfThickness: DxfCode = 39;
    pub const kDxfReal: DxfCode = 40;
    pub const kDxfViewportHeight: DxfCode = 40;
    pub const kDxfTxtSize: DxfCode = 40;
    pub const kDxfTxtStyleXScale: DxfCode = 41;
    pub const kDxfViewWidth: DxfCode = 41;
    pub const kDxfViewportAspect: DxfCode = 41;
    pub const kDxfTxtStylePSize: DxfCode = 42;
    pub const kDxfViewLensLength: DxfCode = 42;
    pub const kDxfViewFrontClip: DxfCode = 43;
    pub const kDxfViewBackClip: DxfCode = 44;
    pub const kDxfShapeXOffset: DxfCode = 44;
    pub const kDxfShapeYOffset: DxfCode = 45;
    pub const kDxfViewHeight: DxfCode = 45;
    pub const kDxfShapeScale: DxfCode = 46;
    pub const kDxfPixelScale: DxfCode = 47;
    pub const kDxfLinetypeScale: DxfCode = 48;
    pub const kDxfDashLength: DxfCode = 49;
    pub const kDxfMlineOffset: DxfCode = 49;
    pub const kDxfLinetypeElement: DxfCode = 49;
    pub const kDxfAngle: DxfCode = 50;
    pub const kDxfViewportSnapAngle: DxfCode = 50;
    pub const kDxfViewportTwist: DxfCode = 51;
    pub const kDxfVisibility: DxfCode = 60;
    pub const kDxfViewportGridDisplay: DxfCode = 60;
    pub const kDxfLayerLinetype: DxfCode = 61;
    pub const kDxfViewportGridMajor: DxfCode = 61;
    pub const kDxfColor: DxfCode = 62;
    pub const kDxfHasSubentities: DxfCode = 66;
    pub const kDxfViewportVisibility: DxfCode = 67;
    pub const kDxfViewportActive: DxfCode = 68;
    pub const kDxfViewportNumber: DxfCode = 69;
    pub const kDxfInt16: DxfCode = 70;
    pub const kDxfViewMode: DxfCode = 71;
    pub const kDxfCircleSides: DxfCode = 72;
    pub const kDxfViewportZoom: DxfCode = 73;
    pub const kDxfViewportIcon: DxfCode = 74;
    pub const kDxfViewportSnap: DxfCode = 75;
    pub const kDxfViewportGrid: DxfCode = 76;
    pub const kDxfViewportSnapStyle: DxfCode = 77;
    pub const kDxfViewportSnapPair: DxfCode = 78;
    pub const kDxfRegAppFlags: DxfCode = 71;
    pub const kDxfTxtStyleFlags: DxfCode = 71;
    pub const kDxfLinetypeAlign: DxfCode = 72;
    pub const kDxfLinetypePDC: DxfCode = 73;
    pub const kDxfi32: DxfCode = 90;
    pub const kDxfVertexIdentifier: DxfCode = 91;
    pub const kDxfSubclass: DxfCode = 100;
    pub const kDxfEmbeddedObjectStart: DxfCode = 101;
    pub const kDxfControlString: DxfCode = 102;
    pub const kDxfDimVarHandle: DxfCode = 105;
    pub const kDxfUCSOrg: DxfCode = 110;
    pub const kDxfUCSOriX: DxfCode = 111;
    pub const kDxfUCSOriY: DxfCode = 112;
    pub const kDxfXReal: DxfCode = 140;
    pub const kDxfViewBrightness: DxfCode = 141;
    pub const kDxfViewContrast: DxfCode = 142;
    pub const kDxfInt64: DxfCode = 160;
    pub const kDxfXInt16: DxfCode = 170;
    pub const kDxfNormalX: DxfCode = 210;
    pub const kDxfNormalY: DxfCode = 220;
    pub const kDxfNormalZ: DxfCode = 230;
    pub const kDxfXXInt16: DxfCode = 270;
    pub const kDxfInt8: DxfCode = 280;
    pub const kDxfRenderMode: DxfCode = 281;
    pub const kDxfDefaultLightingType: DxfCode = 282;
    pub const kDxfShadowFlags: DxfCode = 284;
    pub const kDxfBool: DxfCode = 290;
    pub const kDxfDefaultLightingOn: DxfCode = 292;
    pub const kDxfXTextString: DxfCode = 300;
    pub const kDxfBinaryChunk: DxfCode = 310;
    pub const kDxfArbHandle: DxfCode = 320;
    pub const kDxfSoftPointerId: DxfCode = 330;
    pub const kDxfViewBackgroundId: DxfCode = 332;
    pub const kDxfShadePlotId: DxfCode = 333;
    pub const kDxfLiveSectionId: DxfCode = 334;
    pub const kDxfLiveSectionName: DxfCode = 309;
    pub const kDxfHardPointerId: DxfCode = 340;
    pub const kDxfObjVisualStyleId: DxfCode = 345;
    pub const kDxfVpVisualStyleId: DxfCode = 346;
    pub const kDxfMaterialId: DxfCode = 347;
    pub const kDxfVisualStyleId: DxfCode = 348;
    pub const kDxfDragVisualStyleId: DxfCode = 349;
    pub const kDxfSoftOwnershipId: DxfCode = 350;
    pub const kDxfHardOwnershipId: DxfCode = 360;
    pub const kDxfSunId: DxfCode = 361;
    pub const kDxfLineWeight: DxfCode = 370;
    pub const kDxfPlotStyleNameType: DxfCode = 380;
    pub const kDxfPlotStyleNameId: DxfCode = 390;
    pub const kDxfXXXInt16: DxfCode = 400;
    pub const kDxfLayoutName: DxfCode = 410;
    pub const kDxfColorRGB: DxfCode = 420;
    pub const kDxfColorName: DxfCode = 430;
    pub const kDxfAlpha: DxfCode = 440;
    pub const kDxfGradientObjType: DxfCode = 450;
    pub const kDxfGradientPatType: DxfCode = 451;
    pub const kDxfGradientTintType: DxfCode = 452;
    pub const kDxfGradientColCount: DxfCode = 453;
    pub const kDxfGradientAngle: DxfCode = 460;
    pub const kDxfGradientShift: DxfCode = 461;
    pub const kDxfGradientTintVal: DxfCode = 462;
    pub const kDxfGradientColVal: DxfCode = 463;
    pub const kDxfGradientName: DxfCode = 470;
    pub const kDxfFaceStyleId: DxfCode = 480;
    pub const kDxfEdgeStyleId: DxfCode = 481;
    pub const kDxfComment: DxfCode = 999;
    pub const kDxfXdAsciiString: DxfCode = 1000;
    pub const kDxfRegAppName: DxfCode = 1001;
    pub const kDxfXdControlString: DxfCode = 1002;
    pub const kDxfXdLayerName: DxfCode = 1003;
    pub const kDxfXdBinaryChunk: DxfCode = 1004;
    pub const kDxfXdHandle: DxfCode = 1005;
    pub const kDxfXdXCoord: DxfCode = 1010;
    pub const kDxfXdYCoord: DxfCode = 1020;
    pub const kDxfXdZCoord: DxfCode = 1030;
    pub const kDxfXdWorldXCoord: DxfCode = 1011;
    pub const kDxfXdWorldYCoord: DxfCode = 1021;
    pub const kDxfXdWorldZCoord: DxfCode = 1031;
    pub const kDxfXdWorldXDisp: DxfCode = 1012;
    pub const kDxfXdWorldYDisp: DxfCode = 1022;
    pub const kDxfXdWorldZDisp: DxfCode = 1032;
    pub const kDxfXdWorldXDir: DxfCode = 1013;
    pub const kDxfXdWorldYDir: DxfCode = 1023;
    pub const kDxfXdWorldZDir: DxfCode = 1033;
    pub const kDxfXdReal: DxfCode = 1040;
    pub const kDxfXdDist: DxfCode = 1041;
    pub const kDxfXdScale: DxfCode = 1042;
    pub const kDxfXdInteger16: DxfCode = 1070;
    pub const kDxfXdInteger32: DxfCode = 1071;
    pub const kDxfXdMax: DxfCode = 1071;
}
pub mod dwg_datatype{
    use crate::arx::acdb::DwgDataType;
    pub const kDwgNull: DwgDataType = 0;
    pub const kDwgReal: DwgDataType = 1;
    pub const kDwgi32: DwgDataType = 2;
    pub const kDwgInt16: DwgDataType = 3;
    pub const kDwgInt8: DwgDataType = 4;
    pub const kDwgText: DwgDataType = 5;
    pub const kDwgBChunk: DwgDataType = 6;
    pub const kDwgHandle: DwgDataType = 7;
    pub const kDwgHardOwnershipId: DwgDataType = 8;
    pub const kDwgSoftOwnershipId: DwgDataType = 9;
    pub const kDwgHardPointerId: DwgDataType = 10;
    pub const kDwgSoftPointerId: DwgDataType = 11;
    pub const kDwg3Real: DwgDataType = 12;
    pub const kDwgInt64: DwgDataType = 13;
    pub const kDwgNotRecognized: DwgDataType = 19;
}
pub mod coord_axis{
    use crate::arx::acdb::CoordAxis;
    pub const kX: CoordAxis = 0;
    pub const kY: CoordAxis = 1;
    pub const kZ: CoordAxis = 2;
}
pub mod coord_system{
    use crate::arx::acdb::CoordSystem;
    pub const kWorldCS: CoordSystem = 0;
    pub const kUserCS: CoordSystem = 1;
    pub const kCurDisplayCS: CoordSystem = 2;
    pub const kPaperDisplayCS: CoordSystem = 3;
    pub const kEntityCS: CoordSystem = 4;
}
pub mod intersect{
    use crate::arx::acdb::Intersect;
    pub const kOnBothOperands: Intersect = 0;
    pub const kExtendThis: Intersect = 1;
    pub const kExtendArg: Intersect = 2;
    pub const kExtendBoth: Intersect = 3;
}
pub mod visibility{
    use crate::arx::acdb::Visibility;
    pub const kInvisible: Visibility = 1;
    pub const kVisible: Visibility = 0;
}
pub mod osnap_mask{
    use crate::arx::acdb::OsnapMask;
    pub const kOsMaskEnd: OsnapMask = 1;
    pub const kOsMaskMid: OsnapMask = 2;
    pub const kOsMaskCen: OsnapMask = 4;
    pub const kOsMaskNode: OsnapMask = 8;
    pub const kOsMaskQuad: OsnapMask = 16;
    pub const kOsMaskInt: OsnapMask = 32;
    pub const kOsMaskIns: OsnapMask = 64;
    pub const kOsMaskPerp: OsnapMask = 128;
    pub const kOsMaskTan: OsnapMask = 256;
    pub const kOsMaskNear: OsnapMask = 512;
    pub const kOsMaskQuick: OsnapMask = 1024;
    pub const kOsMaskApint: OsnapMask = 2048;
    pub const kOsMaskImmediate: OsnapMask = 65536;
    pub const kOsMaskAllowTan: OsnapMask = 131072;
    pub const kOsMaskDisablePerp: OsnapMask = 262144;
    pub const kOsMaskRelCartesian: OsnapMask = 524288;
    pub const kOsMaskRelPolar: OsnapMask = 1048576;
    pub const kOsMaskNoneOverride: OsnapMask = 2097152;
}
pub mod duplicate_record_cloning{
    use crate::arx::acdb::DuplicateRecordCloning;
    pub const kDrcNotApplicable: DuplicateRecordCloning = 0;
    pub const kDrcIgnore: DuplicateRecordCloning = 1;
    pub const kDrcReplace: DuplicateRecordCloning = 2;
    pub const kDrcXrefMangleName: DuplicateRecordCloning = 3;
    pub const kDrcMangleName: DuplicateRecordCloning = 4;
    pub const kDrcUnmangleName: DuplicateRecordCloning = 5;
}
pub mod dwg_version{
    use crate::arx::acdb::DwgVersion;
    pub const kDHL_MC0_0: DwgVersion = 0;
    pub const kDHL_AC1_2: DwgVersion = 1;
    pub const kDHL_AC1_40: DwgVersion = 2;
    pub const kDHL_AC1_50: DwgVersion = 3;
    pub const kDHL_AC2_20: DwgVersion = 4;
    pub const kDHL_AC2_10: DwgVersion = 5;
    pub const kDHL_AC2_21: DwgVersion = 6;
    pub const kDHL_AC2_22: DwgVersion = 7;
    pub const kDHL_1001: DwgVersion = 8;
    pub const kDHL_1002: DwgVersion = 9;
    pub const kDHL_1003: DwgVersion = 10;
    pub const kDHL_1004: DwgVersion = 11;
    pub const kDHL_1005: DwgVersion = 12;
    pub const kDHL_1006: DwgVersion = 13;
    pub const kDHL_1007: DwgVersion = 14;
    pub const kDHL_1008: DwgVersion = 15;
    pub const kDHL_1009: DwgVersion = 16;
    pub const kDHL_1010: DwgVersion = 17;
    pub const kDHL_1011: DwgVersion = 18;
    pub const kDHL_1012: DwgVersion = 19;
    pub const kDHL_1013: DwgVersion = 20;
    pub const kDHL_1014: DwgVersion = 21;
    pub const kDHL_1500: DwgVersion = 22;
    pub const kDHL_1015: DwgVersion = 23;
    pub const kDHL_1800a: DwgVersion = 24;
    pub const kDHL_1800: DwgVersion = 25;
    pub const kDHL_2100a: DwgVersion = 26;
    pub const kDHL_1021: DwgVersion = 27;
    pub const kDHL_2400a: DwgVersion = 28;
    pub const kDHL_1024: DwgVersion = 29;
    pub const kDHL_2700a: DwgVersion = 30;
    pub const kDHL_1027: DwgVersion = 31;
    pub const kDHL_3200a: DwgVersion = 32;
    pub const kDHL_1032: DwgVersion = 33;
    pub const kDHL_Newest: DwgVersion = 33;
    pub const kDHL_CURRENT: DwgVersion = 33;
    pub const kDHL_Unknown: DwgVersion = 254;
    pub const kDHL_Max: DwgVersion = 255;
}
pub mod maintenance_release_version{
    use crate::arx::acdb::MaintenanceReleaseVersion;

    pub const kMRelease0: MaintenanceReleaseVersion = 0;
    pub const kMRelease1: MaintenanceReleaseVersion = 1;
    pub const kMRelease2: MaintenanceReleaseVersion = 2;
    pub const kMRelease3: MaintenanceReleaseVersion = 3;
    pub const kMRelease4: MaintenanceReleaseVersion = 4;
    pub const kMRelease5: MaintenanceReleaseVersion = 5;
    pub const kMRelease6: MaintenanceReleaseVersion = 6;
    pub const kMRelease7: MaintenanceReleaseVersion = 7;
    pub const kMRelease8: MaintenanceReleaseVersion = 8;
    pub const kMRelease9: MaintenanceReleaseVersion = 9;
    pub const kMRelease10: MaintenanceReleaseVersion = 10;
    pub const kMRelease11: MaintenanceReleaseVersion = 11;
    pub const kMRelease12: MaintenanceReleaseVersion = 12;
    pub const kMRelease13: MaintenanceReleaseVersion = 13;
    pub const kMRelease14: MaintenanceReleaseVersion = 14;
    pub const kMRelease15: MaintenanceReleaseVersion = 15;
    pub const kMRelease16: MaintenanceReleaseVersion = 16;
    pub const kMRelease17: MaintenanceReleaseVersion = 17;
    pub const kMRelease18: MaintenanceReleaseVersion = 18;
    pub const kMRelease19: MaintenanceReleaseVersion = 19;
    pub const kMRelease20: MaintenanceReleaseVersion = 20;
    pub const kMRelease21: MaintenanceReleaseVersion = 21;
    pub const kMRelease22: MaintenanceReleaseVersion = 22;
    pub const kMRelease23: MaintenanceReleaseVersion = 23;
    pub const kMRelease24: MaintenanceReleaseVersion = 24;
    pub const kMRelease25: MaintenanceReleaseVersion = 25;
    pub const kMRelease26: MaintenanceReleaseVersion = 26;
    pub const kMRelease27: MaintenanceReleaseVersion = 27;
    pub const kMRelease28: MaintenanceReleaseVersion = 28;
    pub const kMRelease29: MaintenanceReleaseVersion = 29;
    pub const kMRelease30: MaintenanceReleaseVersion = 30;
    pub const kMRelease31: MaintenanceReleaseVersion = 31;
    pub const kMRelease32: MaintenanceReleaseVersion = 32;
    pub const kMRelease33: MaintenanceReleaseVersion = 33;
    pub const kMRelease34: MaintenanceReleaseVersion = 34;
    pub const kMRelease35: MaintenanceReleaseVersion = 35;
    pub const kMRelease36: MaintenanceReleaseVersion = 36;
    pub const kMRelease37: MaintenanceReleaseVersion = 37;
    pub const kMRelease38: MaintenanceReleaseVersion = 38;
    pub const kMRelease39: MaintenanceReleaseVersion = 39;
    pub const kMRelease40: MaintenanceReleaseVersion = 40;
    pub const kMRelease41: MaintenanceReleaseVersion = 41;
    pub const kMReleaseFirstValid1500: MaintenanceReleaseVersion = 41;
    pub const kMRelease42: MaintenanceReleaseVersion = 42;
    pub const kMRelease43: MaintenanceReleaseVersion = 43;
    pub const kMRelease44: MaintenanceReleaseVersion = 44;
    pub const kMRelease45: MaintenanceReleaseVersion = 45;
    pub const kMRelease46: MaintenanceReleaseVersion = 46;
    pub const kMRelease47: MaintenanceReleaseVersion = 47;
    pub const kMRelease48: MaintenanceReleaseVersion = 48;
    pub const kMRelease49: MaintenanceReleaseVersion = 49;
    pub const kMRelease50: MaintenanceReleaseVersion = 50;
    pub const kMRelease51: MaintenanceReleaseVersion = 51;
    pub const kMRelease52: MaintenanceReleaseVersion = 52;
    pub const kMRelease53: MaintenanceReleaseVersion = 53;
    pub const kMRelease54: MaintenanceReleaseVersion = 54;
    pub const kMRelease55: MaintenanceReleaseVersion = 55;
    pub const kMRelease56: MaintenanceReleaseVersion = 56;
    pub const kMRelease57: MaintenanceReleaseVersion = 57;
    pub const kMRelease58: MaintenanceReleaseVersion = 58;
    pub const kMRelease59: MaintenanceReleaseVersion = 59;
    pub const kMRelease60: MaintenanceReleaseVersion = 60;
    pub const kMRelease61: MaintenanceReleaseVersion = 61;
    pub const kMRelease62: MaintenanceReleaseVersion = 62;
    pub const kMRelease63: MaintenanceReleaseVersion = 63;
    pub const kMRelease64: MaintenanceReleaseVersion = 64;
    pub const kMRelease65: MaintenanceReleaseVersion = 65;
    pub const kMRelease66: MaintenanceReleaseVersion = 66;
    pub const kMRelease67: MaintenanceReleaseVersion = 67;
    pub const kMRelease68: MaintenanceReleaseVersion = 68;
    pub const kMRelease69: MaintenanceReleaseVersion = 69;
    pub const kMRelease70: MaintenanceReleaseVersion = 70;
    pub const kMRelease71: MaintenanceReleaseVersion = 71;
    pub const kMRelease72: MaintenanceReleaseVersion = 72;
    pub const kMRelease73: MaintenanceReleaseVersion = 73;
    pub const kMRelease74: MaintenanceReleaseVersion = 74;
    pub const kMRelease75: MaintenanceReleaseVersion = 75;
    pub const kMRelease76: MaintenanceReleaseVersion = 76;
    pub const kMRelease77: MaintenanceReleaseVersion = 77;
    pub const kMRelease78: MaintenanceReleaseVersion = 78;
    pub const kMRelease79: MaintenanceReleaseVersion = 79;
    pub const kMRelease80: MaintenanceReleaseVersion = 80;
    pub const kMRelease81: MaintenanceReleaseVersion = 81;
    pub const kMRelease82: MaintenanceReleaseVersion = 82;
    pub const kMRelease83: MaintenanceReleaseVersion = 83;
    pub const kMRelease84: MaintenanceReleaseVersion = 84;
    pub const kMRelease85: MaintenanceReleaseVersion = 85;
    pub const kMRelease86: MaintenanceReleaseVersion = 86;
    pub const kMRelease87: MaintenanceReleaseVersion = 87;
    pub const kMRelease88: MaintenanceReleaseVersion = 88;
    pub const kMRelease89: MaintenanceReleaseVersion = 89;
    pub const kMRelease90: MaintenanceReleaseVersion = 90;
    pub const kMRelease91: MaintenanceReleaseVersion = 91;
    pub const kMRelease92: MaintenanceReleaseVersion = 92;
    pub const kMRelease93: MaintenanceReleaseVersion = 93;
    pub const kMRelease94: MaintenanceReleaseVersion = 94;
    pub const kMRelease95: MaintenanceReleaseVersion = 95;
    pub const kMRelease96: MaintenanceReleaseVersion = 96;
    pub const kMRelease97: MaintenanceReleaseVersion = 97;
    pub const kMRelease98: MaintenanceReleaseVersion = 98;
    pub const kMRelease99: MaintenanceReleaseVersion = 99;
    pub const kMRelease100: MaintenanceReleaseVersion = 100;
    pub const kMRelease101: MaintenanceReleaseVersion = 101;
    pub const kMRelease102: MaintenanceReleaseVersion = 102;
    pub const kMRelease103: MaintenanceReleaseVersion = 103;
    pub const kMRelease104: MaintenanceReleaseVersion = 104;
    pub const kMRelease105: MaintenanceReleaseVersion = 105;
    pub const kMRelease106: MaintenanceReleaseVersion = 106;
    pub const kMRelease107: MaintenanceReleaseVersion = 107;
    pub const kMRelease108: MaintenanceReleaseVersion = 108;
    pub const kMRelease109: MaintenanceReleaseVersion = 109;
    pub const kMRelease110: MaintenanceReleaseVersion = 110;
    pub const kMRelease111: MaintenanceReleaseVersion = 111;
    pub const kMRelease112: MaintenanceReleaseVersion = 112;
    pub const kMRelease113: MaintenanceReleaseVersion = 113;
    pub const kMRelease114: MaintenanceReleaseVersion = 114;
    pub const kMRelease115: MaintenanceReleaseVersion = 115;
    pub const kMRelease116: MaintenanceReleaseVersion = 116;
    pub const kMRelease117: MaintenanceReleaseVersion = 117;
    pub const kMRelease118: MaintenanceReleaseVersion = 118;
    pub const kMRelease119: MaintenanceReleaseVersion = 119;
    pub const kMRelease120: MaintenanceReleaseVersion = 120;
    pub const kMRelease121: MaintenanceReleaseVersion = 121;
    pub const kMRelease122: MaintenanceReleaseVersion = 122;
    pub const kMRelease123: MaintenanceReleaseVersion = 123;
    pub const kMRelease124: MaintenanceReleaseVersion = 124;
    pub const kMRelease125: MaintenanceReleaseVersion = 125;
    pub const kMRelease126: MaintenanceReleaseVersion = 126;
    pub const kMRelease127: MaintenanceReleaseVersion = 127;
    pub const kMRelease128: MaintenanceReleaseVersion = 128;
    pub const kMRelease129: MaintenanceReleaseVersion = 129;
    pub const kMRelease130: MaintenanceReleaseVersion = 130;
    pub const kMRelease131: MaintenanceReleaseVersion = 131;
    pub const kMRelease132: MaintenanceReleaseVersion = 132;
    pub const kMRelease133: MaintenanceReleaseVersion = 133;
    pub const kMRelease134: MaintenanceReleaseVersion = 134;
    pub const kMRelease135: MaintenanceReleaseVersion = 135;
    pub const kMRelease136: MaintenanceReleaseVersion = 136;
    pub const kMRelease137: MaintenanceReleaseVersion = 137;
    pub const kMRelease138: MaintenanceReleaseVersion = 138;
    pub const kMRelease139: MaintenanceReleaseVersion = 139;
    pub const kMRelease140: MaintenanceReleaseVersion = 140;
    pub const kMRelease141: MaintenanceReleaseVersion = 141;
    pub const kMRelease142: MaintenanceReleaseVersion = 142;
    pub const kMRelease143: MaintenanceReleaseVersion = 143;
    pub const kMRelease144: MaintenanceReleaseVersion = 144;
    pub const kMRelease145: MaintenanceReleaseVersion = 145;
    pub const kMRelease146: MaintenanceReleaseVersion = 146;
    pub const kMRelease147: MaintenanceReleaseVersion = 147;
    pub const kMRelease148: MaintenanceReleaseVersion = 148;
    pub const kMRelease149: MaintenanceReleaseVersion = 149;
    pub const kMRelease150: MaintenanceReleaseVersion = 150;
    pub const kMRelease151: MaintenanceReleaseVersion = 151;
    pub const kMRelease152: MaintenanceReleaseVersion = 152;
    pub const kMRelease153: MaintenanceReleaseVersion = 153;
    pub const kMRelease154: MaintenanceReleaseVersion = 154;
    pub const kMRelease155: MaintenanceReleaseVersion = 155;
    pub const kMRelease156: MaintenanceReleaseVersion = 156;
    pub const kMRelease157: MaintenanceReleaseVersion = 157;
    pub const kMRelease158: MaintenanceReleaseVersion = 158;
    pub const kMRelease159: MaintenanceReleaseVersion = 159;
    pub const kMRelease160: MaintenanceReleaseVersion = 160;
    pub const kMRelease161: MaintenanceReleaseVersion = 161;
    pub const kMRelease162: MaintenanceReleaseVersion = 162;
    pub const kMRelease163: MaintenanceReleaseVersion = 163;
    pub const kMRelease164: MaintenanceReleaseVersion = 164;
    pub const kMRelease165: MaintenanceReleaseVersion = 165;
    pub const kMRelease166: MaintenanceReleaseVersion = 166;
    pub const kMRelease167: MaintenanceReleaseVersion = 167;
    pub const kMRelease168: MaintenanceReleaseVersion = 168;
    pub const kMRelease169: MaintenanceReleaseVersion = 169;
    pub const kMRelease170: MaintenanceReleaseVersion = 170;
    pub const kMRelease171: MaintenanceReleaseVersion = 171;
    pub const kMRelease172: MaintenanceReleaseVersion = 172;
    pub const kMRelease173: MaintenanceReleaseVersion = 173;
    pub const kMRelease174: MaintenanceReleaseVersion = 174;
    pub const kMRelease175: MaintenanceReleaseVersion = 175;
    pub const kMRelease176: MaintenanceReleaseVersion = 176;
    pub const kMRelease177: MaintenanceReleaseVersion = 177;
    pub const kMRelease178: MaintenanceReleaseVersion = 178;
    pub const kMRelease179: MaintenanceReleaseVersion = 179;
    pub const kMRelease180: MaintenanceReleaseVersion = 180;
    pub const kMRelease181: MaintenanceReleaseVersion = 181;
    pub const kMRelease182: MaintenanceReleaseVersion = 182;
    pub const kMRelease183: MaintenanceReleaseVersion = 183;
    pub const kMRelease184: MaintenanceReleaseVersion = 184;
    pub const kMRelease185: MaintenanceReleaseVersion = 185;
    pub const kMRelease186: MaintenanceReleaseVersion = 186;
    pub const kMRelease187: MaintenanceReleaseVersion = 187;
    pub const kMRelease188: MaintenanceReleaseVersion = 188;
    pub const kMRelease189: MaintenanceReleaseVersion = 189;
    pub const kMRelease190: MaintenanceReleaseVersion = 190;
    pub const kMRelease191: MaintenanceReleaseVersion = 191;
    pub const kMRelease192: MaintenanceReleaseVersion = 192;
    pub const kMRelease193: MaintenanceReleaseVersion = 193;
    pub const kMRelease194: MaintenanceReleaseVersion = 194;
    pub const kMRelease195: MaintenanceReleaseVersion = 195;
    pub const kMRelease196: MaintenanceReleaseVersion = 196;
    pub const kMRelease197: MaintenanceReleaseVersion = 197;
    pub const kMRelease198: MaintenanceReleaseVersion = 198;
    pub const kMRelease199: MaintenanceReleaseVersion = 199;
    pub const kMRelease200: MaintenanceReleaseVersion = 200;
    pub const kMRelease201: MaintenanceReleaseVersion = 201;
    pub const kMRelease202: MaintenanceReleaseVersion = 202;
    pub const kMRelease203: MaintenanceReleaseVersion = 203;
    pub const kMRelease204: MaintenanceReleaseVersion = 204;
    pub const kMRelease205: MaintenanceReleaseVersion = 205;
    pub const kMRelease206: MaintenanceReleaseVersion = 206;
    pub const kMRelease207: MaintenanceReleaseVersion = 207;
    pub const kMRelease208: MaintenanceReleaseVersion = 208;
    pub const kMRelease209: MaintenanceReleaseVersion = 209;
    pub const kMRelease210: MaintenanceReleaseVersion = 210;
    pub const kMRelease211: MaintenanceReleaseVersion = 211;
    pub const kMRelease212: MaintenanceReleaseVersion = 212;
    pub const kMRelease213: MaintenanceReleaseVersion = 213;
    pub const kMRelease214: MaintenanceReleaseVersion = 214;
    pub const kMRelease215: MaintenanceReleaseVersion = 215;
    pub const kMRelease216: MaintenanceReleaseVersion = 216;
    pub const kMRelease217: MaintenanceReleaseVersion = 217;
    pub const kMRelease218: MaintenanceReleaseVersion = 218;
    pub const kMRelease219: MaintenanceReleaseVersion = 219;
    pub const kMRelease220: MaintenanceReleaseVersion = 220;
    pub const kMRelease221: MaintenanceReleaseVersion = 221;
    pub const kMRelease222: MaintenanceReleaseVersion = 222;
    pub const kMRelease223: MaintenanceReleaseVersion = 223;
    pub const kMRelease224: MaintenanceReleaseVersion = 224;
    pub const kMRelease225: MaintenanceReleaseVersion = 225;
    pub const kMRelease226: MaintenanceReleaseVersion = 226;
    pub const kMRelease227: MaintenanceReleaseVersion = 227;
    pub const kMRelease228: MaintenanceReleaseVersion = 228;
    pub const kMRelease229: MaintenanceReleaseVersion = 229;
    pub const kMRelease230: MaintenanceReleaseVersion = 230;
    pub const kMRelease231: MaintenanceReleaseVersion = 231;
    pub const kMRelease232: MaintenanceReleaseVersion = 232;
    pub const kMRelease233: MaintenanceReleaseVersion = 233;
    pub const kMRelease234: MaintenanceReleaseVersion = 234;
    pub const kMRelease235: MaintenanceReleaseVersion = 235;
    pub const kMRelease236: MaintenanceReleaseVersion = 236;
    pub const kMRelease237: MaintenanceReleaseVersion = 237;
    pub const kMRelease238: MaintenanceReleaseVersion = 238;
    pub const kMRelease239: MaintenanceReleaseVersion = 239;
    pub const kMRelease240: MaintenanceReleaseVersion = 240;
    pub const kMRelease241: MaintenanceReleaseVersion = 241;
    pub const kMRelease242: MaintenanceReleaseVersion = 242;
    pub const kMRelease243: MaintenanceReleaseVersion = 243;
    pub const kMRelease244: MaintenanceReleaseVersion = 244;
    pub const kMRelease245: MaintenanceReleaseVersion = 245;
    pub const kMRelease246: MaintenanceReleaseVersion = 246;
    pub const kMRelease247: MaintenanceReleaseVersion = 247;
    pub const kMRelease248: MaintenanceReleaseVersion = 248;
    pub const kMRelease249: MaintenanceReleaseVersion = 249;
    pub const kMRelease250: MaintenanceReleaseVersion = 250;
    pub const kMRelease251: MaintenanceReleaseVersion = 251;
    pub const kMRelease252: MaintenanceReleaseVersion = 252;
    pub const kMRelease253: MaintenanceReleaseVersion = 253;
    pub const kMRelease254: MaintenanceReleaseVersion = 254;
    pub const kMRelease255: MaintenanceReleaseVersion = 255;
    pub const kMReleaseNewest: MaintenanceReleaseVersion = 4;
    pub const kMRelease2010Newest: MaintenanceReleaseVersion = 226;
    pub const kMReleaseCurrent: MaintenanceReleaseVersion = 4;
    pub const kMReleaseUnknown: MaintenanceReleaseVersion = 2147483646;
    pub const kMReleaseMax: MaintenanceReleaseVersion = 2147483647;
    pub const kMRelease2010Max: MaintenanceReleaseVersion = 255;
    pub const kMReleaseExtendedNewest: MaintenanceReleaseVersion = 51;
    pub const kMReleaseExtendedCurrent: MaintenanceReleaseVersion = 51;
    pub const kMReleaseCheckExtended: MaintenanceReleaseVersion = 125;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ViewTableRecord {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Viewport {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ObjectContextManager {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Stub {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImpDatabase {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImpObject {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug)]
pub struct Entity {
    pub _base: Object,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IdMapping {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug)]
pub struct AuditInfo {
    pub mpImpAudit: *mut ImpAuditInfo,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImpAuditInfo {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug)]
pub struct DwgFiler {
    pub _base: acrx::Object,
    pub mController: *mut FilerController,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FilerController {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug)]
pub struct AcDbDxfFiler {
    pub _base: acrx::Object,
    pub mController: *mut FilerController,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Field {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ViewportDraw {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AcGiDrawStream {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AcGiDrawable {
    pub _base: acrx::object::Object,
    pub m_pAccessory: *mut acgi::DrawableAccessory,
}

#[repr(C)]
#[derive(Debug)]
pub struct ObjectReactor {
    pub _base: acrx::Object,
}

pub type ObjectIdArray = Array<ObjectId>;
pub type VoidPtrArray = Array<*mut c_void>;


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dictionary {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SystemInternals {
    _unused: [u8; 0],
}

extern "C" {
    #[link_name = "\u{1}?acdbFail@@YAXPEB_W@Z"]
    pub fn fail(str_: *const ACHAR);

    #[link_name = "\u{1}?acdbHandEnt@@YAHPEB_WQEA_J@Z"]
    pub fn hand_ent(handle: *const ACHAR, entres: *mut i64) -> c_int;

    #[link_name = "\u{1}?acdbXdRoom@@YAHQEB_JPEAH@Z"]
    pub fn xd_room(ent: *mut i64, result: *mut i32) -> c_int;

    #[link_name = "\u{1}?acdbXdSize@@YAHPEBUresbuf@@PEAH@Z"]
    pub fn xd_size(rb: *const _resbuf, result: *mut i32) -> c_int;

    #[link_name = "\u{1}?acdbXStrSave@@YAPEA_WPEA_WPEAPEA_W@Z"]
    pub fn xstr_save(pSource: *mut ACHAR, pDest: *mut *mut ACHAR) -> *mut ACHAR;

    #[link_name = "\u{1}?acdbXStrCase@@YAHPEA_W_K@Z"]
    pub fn xstr_case(str_: *mut ACHAR, strLen: size_t) -> c_int;

    #[link_name = "\u{1}?acdbEntDel@@YAHQEB_J@Z"]
    pub fn ent_del(ent: *mut i64) -> c_int;

    #[link_name = "\u{1}?acdbEntGetX@@YAPEAUresbuf@@QEB_JPEBU1@@Z"]
    pub fn ent_getx(ent: *mut i64, args: *const _resbuf) -> *mut _resbuf;

    #[link_name = "\u{1}?acdbEntGet@@YAPEAUresbuf@@QEB_J@Z"]
    pub fn ent_get(ent: *mut i64) -> *mut _resbuf;

    #[link_name = "\u{1}?acdbEntLast@@YAHQEA_J@Z"]
    pub fn ent_last(result: *mut i64) -> c_int;

    #[link_name = "\u{1}?acdbEntNext@@YAHQEB_JQEA_J@Z"]
    pub fn ent_next(ent: *mut i64, result: *mut i64) -> c_int;

    #[link_name = "\u{1}?acdbEntUpd@@YAHQEB_J@Z"]
    pub fn ent_upd(ent: *mut i64) -> c_int;

    #[link_name = "\u{1}?acdbEntMod@@YAHPEBUresbuf@@@Z"]
    pub fn ent_mod(ent: *const _resbuf) -> c_int;

    #[link_name = "\u{1}?acdbEntMake@@YAHPEBUresbuf@@@Z"]
    pub fn ent_make(ent: *const _resbuf) -> c_int;

    #[link_name = "\u{1}?acdbEntMakeX@@YAHPEBUresbuf@@QEA_J@Z"]
    pub fn ent_makex(entm: *const _resbuf, result: *mut i64) -> c_int;

    #[link_name = "\u{1}?acdbRegApp@@YAHPEB_W@Z"]
    pub fn reg_app(appname: *const ACHAR) -> c_int;

    #[link_name = "\u{1}?acdbTblNext@@YAPEAUresbuf@@PEB_WH@Z"]
    pub fn tbl_next(tblname: *const ACHAR, rewind: c_int) -> *mut _resbuf;

    #[link_name = "\u{1}?acdbTblSearch@@YAPEAUresbuf@@PEB_W0H@Z"]
    pub fn tbl_search(
        tblname: *const ACHAR,
        sym: *const ACHAR,
        setnext: c_int,
    ) -> *mut _resbuf;

    #[link_name = "\u{1}?acdbNamedObjDict@@YAHQEA_J@Z"]
    pub fn named_obj_dict(result: *mut i64) -> c_int;

    #[link_name = "\u{1}?acdbDictSearch@@YAPEAUresbuf@@QEB_JPEB_WH@Z"]
    pub fn dict_search(
        dict: *mut i64,
        sym: *const ACHAR,
        setnext: c_int,
    ) -> *mut _resbuf;

    #[link_name = "\u{1}?acdbDictNext@@YAPEAUresbuf@@QEB_JH@Z"]
    pub fn dict_next(dict: *mut i64, rewind: c_int) -> *mut _resbuf;

    #[link_name = "\u{1}?acdbDictRename@@YAHQEB_JPEB_W1@Z"]
    pub fn dict_rename(
        dictname: *mut i64,
        oldsym: *const ACHAR,
        newsym: *const ACHAR,
    ) -> c_int;

    #[link_name = "\u{1}?acdbDictRemove@@YAHQEB_JPEB_W@Z"]
    pub fn dict_remove(dictname: *mut i64, symname: *const ACHAR) -> c_int;

    #[link_name = "\u{1}?acdbDictAdd@@YAHQEB_JPEB_W0@Z"]
    pub fn dict_add(
        dictname: *mut i64,
        symname: *const ACHAR,
        newobj: *mut i64,
    ) -> c_int;

    #[link_name = "\u{1}?acdbTblObjName@@YAHPEB_W0QEA_J@Z"]
    pub fn tbl_obj_name(
        tblname: *const ACHAR,
        sym: *const ACHAR,
        result: *mut i64,
    ) -> c_int;

    #[link_name = "\u{1}?acdbAngToS@@YAHNHHPEA_W_K@Z"]
    pub fn ang_to_s(
        v: ads_real,
        unit: c_int,
        prec: c_int,
        str_: *mut ACHAR,
        nBufLen: size_t,
    ) -> c_int;

    #[link_name = "\u{1}?acdbRawAngToS@@YAHNHHPEA_W_K@Z"]
    pub fn raw_ang_to_s(
        v: ads_real,
        unit: c_int,
        prec: c_int,
        str_: *mut ACHAR,
        nBufLen: size_t,
    ) -> c_int;

    #[link_name = "\u{1}?acdbRToS@@YAHNHHPEA_W_K@Z"]
    pub fn r_to_s(
        val: ads_real,
        unit: c_int,
        prec: c_int,
        str_: *mut ACHAR,
        nBufLen: size_t,
    ) -> c_int;

    #[link_name = "\u{1}?acdbAngToF@@YAHPEB_WHPEAN@Z"]
    pub fn ang_to_f(
        str_: *const ACHAR,
        unit: c_int,
        v: *mut ads_real,
    ) -> c_int;

    #[link_name = "\u{1}?acdbRawAngToF@@YAHPEB_WHPEAN@Z"]
    pub fn raw_ang_to_f(
        str_: *const ACHAR,
        unit: c_int,
        v: *mut ads_real,
    ) -> c_int;

    #[link_name = "\u{1}?acdbDisToF@@YAHPEB_WHPEAN@Z"]
    pub fn dis_to_f(
        str_: *const ACHAR,
        unit: c_int,
        v: *mut ads_real,
    ) -> c_int;

    #[link_name = "\u{1}?acdbInters@@YAHQEBN000HQEAN@Z"]
    pub fn inters(
        from1: *mut ads_real,
        to1: *mut ads_real,
        from2: *mut ads_real,
        to2: *mut ads_real,
        teston: c_int,
        result: *mut ads_real,
    ) -> c_int;

    #[link_name = "\u{1}?acdbSNValid@@YAHPEB_WH@Z"]
    pub fn sn_valid(
        tbstr: *const ACHAR,
        pipetest: c_int,
    ) -> c_int;

    #[link_name = "\u{1}?acdbGroupCodeToType@@YA?AW4DwgDataType@AcDb@@F@Z"]
    pub fn group_code_to_type(pCode: DxfCode) -> DwgDataType;

    #[link_name = "\u{1}?acdbFreeResBufContents@@YAXPEAUresbuf@@@Z"]
    pub fn FreeResBufContents(pField: *mut _resbuf);

    #[link_name = "\u{1}?acdbPersistentReactorObjectId@@YA?AVAcDbObjectId@@PEBX@Z"]
    pub fn PersistentReactorObjectId(pSomething: *const c_void)
                                         -> ObjectId;

    #[link_name = "\u{1}?acdbDisplayPreviewFromDwg@@YA_NPEB_WPEAXPEBI@Z"]
    pub fn display_preview_from_dwg(
        pszDwgfilename: *const ACHAR,
        pPreviewWnd: *mut c_void,
        pBgColor: *const u32,
    ) -> bool;

    #[link_name = "\u{1}?acdbIsCustomObject@@YA_NAEBVAcDbObjectId@@@Z"]
    pub fn is_custom_object(id: *const ObjectId) -> bool;

    #[link_name = "\u{1}?acdbOpenObject@@YA?AW4ErrorStatus@Acad@@AEAPEAVAcDbObject@@VAcDbObjectId@@W4OpenMode@AcDb@@_NPEBVAcRxClass@@@Z"]
    pub fn open_object(
        pObj: *mut *mut Object,
        id: ObjectId,
        mode: OpenMode,
        openErased: bool,
        pClass: *const acrx::Class,
    ) -> ErrorStatus;

    #[link_name = "\u{1}?acdbOpenObject@@YA?AW4ErrorStatus@Acad@@AEAPEAVAcDbObject@@VAcDbObjectId@@P6APEAVAcRxClass@@XZW4OpenMode@AcDb@@_N@Z"]
    pub fn open_object1(
        pObj: *mut *mut Object,
        id: ObjectId,
        pfDesc: ::std::option::Option<unsafe extern "C" fn() -> *mut acrx::Class>,
        mode: OpenMode,
        openErased: bool,
    ) -> ErrorStatus;

    #[link_name = "\u{1}?acdbOpenObject@@YA?AW4ErrorStatus@Acad@@AEAPEAVAcDbEntity@@VAcDbObjectId@@W4OpenMode@AcDb@@_N@Z"]
    pub fn open_entity(
        pEnt: *mut *mut Entity,
        id: ObjectId,
        mode: OpenMode,
        openErased: bool,
    ) -> ErrorStatus;

    #[link_name = "\u{1}?acdbResurrectMeNow@@YA?AW4ErrorStatus@Acad@@AEBVAcDbObjectId@@@Z"]
    pub fn resurrect_me_now(id: *const ObjectId) -> ErrorStatus;

    #[link_name = "\u{1}?acdbGetAdsName@@YA?AW4ErrorStatus@Acad@@AEAY01_JVAcDbObjectId@@@Z"]
    pub fn get_ads_name(objName: *mut ads_name, objId: ObjectId) -> ErrorStatus;

    #[link_name = "\u{1}?acdbGetObjectId@@YA?AW4ErrorStatus@Acad@@AEAVAcDbObjectId@@QEB_J@Z"]
    pub fn get_object_id(objId: *mut ObjectId, objName: *mut i64) -> ErrorStatus;

    #[link_name = "\u{1}?acdbValKey@@YA?AW4ErrorStatus@Acad@@QEB_J@Z"]
    pub fn val_key(lname: *mut i64) -> ErrorStatus;

    #[link_name = "\u{1}?acdbSetReferenced@@YA?AW4ErrorStatus@Acad@@VAcDbObjectId@@@Z"]
    pub fn set_referenced(objId: ObjectId) -> ErrorStatus;

    #[link_name = "\u{1}?acdbActiveDatabaseArray@@YAAEBV?$AcArray@PEAVAcDbDatabase@@V?$AcArrayMemCopyReallocator@PEAVAcDbDatabase@@@@@@XZ"]
    pub fn active_database_array() -> *const Array<*mut Database>;

    #[link_name = "\u{1}?acdbLoadMlineStyleFile@@YA?AW4ErrorStatus@Acad@@PEB_W0@Z"]
    pub fn load_mline_style_file(sname: *const ACHAR, fname: *const ACHAR) -> ErrorStatus;

    #[link_name = "\u{1}?acdbLoadLineTypeFile@@YA?AW4ErrorStatus@Acad@@PEB_W0PEAVAcDbDatabase@@@Z"]
    pub fn load_linetype_file(
        ltname: *const ACHAR,
        fname: *const ACHAR,
        pDb: *mut Database,
    ) -> ErrorStatus;

    #[link_name = "\u{1}?acdbAlloc@@YAPEAX_K@Z"]
    pub fn alloc(arg1: size_t) -> *mut c_void;

    #[link_name = "\u{1}?acdbRealloc@@YAPEAXPEAX_K@Z"]
    pub fn realloc(
        arg1: *mut c_void,
        arg2: size_t,
    ) -> *mut c_void;

    #[link_name = "\u{1}?acdbFree@@YAXPEAX@Z"]
    pub fn free(arg1: *mut c_void);

    #[link_name = "\u{1}?acdbInitialize@@YAXXZ"]
    pub fn initialize();

    #[link_name = "\u{1}?acdbTerminate@@YAXXZ"]
    pub fn terminate();

    #[link_name = "\u{1}?acdbValidateSetup@@YA?AW4ErrorStatus@Acad@@AEBVAcLocale@@@Z"]
    pub fn validate_setup(loc: *const AcLocale) -> ErrorStatus;

    #[link_name = "\u{1}?acdbCleanUp@@YA?AW4ErrorStatus@Acad@@XZ"]
    pub fn cleanUp() -> ErrorStatus;

    #[link_name = "\u{1}?acdbOriginalXrefFullPathFor@@YAPEB_WPEBVAcDbDatabase@@@Z"]
    pub fn original_xref_full_path_for(arg1: *const Database) -> *const ACHAR;

    #[link_name = "\u{1}?acdbSetDefaultAcGiContext@@YAPEAVAcGiContext@@PEAV1@@Z"]
    pub fn set_default_acgi_context(arg1: *mut acgi::Context) -> *mut acgi::Context;

    #[link_name = "\u{1}?acdbGetThumbnailBitmapFromDxfFile@@YA?AW4ErrorStatus@Acad@@PEB_WAEAPEAX@Z"]
    pub fn get_thumbnail_bitmap_from_dxf_file(
        filename: *const ACHAR,
        pBitmap: *mut *mut c_void,
    ) -> ErrorStatus;
    

}
