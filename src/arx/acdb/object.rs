#![allow(dead_code)]
use crate::arx::acgi::drawable::Drawable;
use crate::arx::acdb::{ImpObject, IdMapping, AuditInfo, DwgFiler, AcDbDxfFiler, DuplicateRecordCloning, DwgVersion, MaintenanceReleaseVersion, Field, ViewportDraw, OpenMode, ObjectReactor, VoidPtrArray, Dictionary, SystemInternals};
use std::os::raw::{c_void, c_char};
use crate::arx::acad_app::ErrorStatus;
use crate::arx::adesk::Boolean;
use crate::arx::acdb::object_id::ObjectId;
use crate::arx::{acrx, acdb, ACHAR, CLSID, AppNameChangeFuncPtr, wchar_t, ObjectIdArray, matrix3d};
use crate::arx::resbuf::_resbuf;
use crate::arx::acgi::{WorldDraw, DrawableTraits};
use crate::arx::acdb::handle::Handle;
use crate::arx::acdb::database::Database;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Object {
    pub _base: Drawable,
    pub mpImpObject: *mut ImpObject,
}
extern "C" {
    #[link_name = "\u{1}?gpDesc@AcDbObject@@2PEAVAcRxClass@@EA"]
    pub static mut AcDbObject_gpDesc: *mut acrx::Class;
    
    #[link_name = "\u{1}?desc@AcDbObject@@SAPEAVAcRxClass@@XZ"]
    pub fn desc() -> *mut acrx::Class;
    
    #[link_name = "\u{1}?rxInit@AcDbObject@@SAXXZ"]
    pub fn rx_init();
    
    #[link_name = "\u{1}?rxInit@AcDbObject@@SAXP6AXPEBVAcRxClass@@AEAPEA_WH@Z@Z"]
    pub fn rx_init1(arg1: AppNameChangeFuncPtr);
    
    #[link_name = "\u{1}?objectId@AcDbObject@@QEBA?AVObjectId@@XZ"]
    pub fn object_id(this: *const Object) -> ObjectId;
    
    #[link_name = "\u{1}?ownerId@AcDbObject@@QEBA?AVObjectId@@XZ"]
    pub fn owner_id(this: *const Object) -> ObjectId;
    
    #[link_name = "\u{1}?getAcDbHandle@AcDbObject@@QEBAXAEAVAcDbHandle@@@Z"]
    pub fn get_handle(this: *const Object, handle: *mut Handle);
    
    #[link_name = "\u{1}?database@AcDbObject@@QEBAPEAVAcDbDatabase@@XZ"]
    pub fn database(this: *const Object) -> *mut Database;
    
    #[link_name = "\u{1}?createExtensionDictionary@AcDbObject@@QEAA?AW4ErrorStatus@Acad@@XZ"]
    pub fn create_extension_dictionary(this: *mut Object) -> ErrorStatus;
    
    #[link_name = "\u{1}?extensionDictionary@AcDbObject@@QEBA?AVObjectId@@XZ"]
    pub fn extension_dictionary(this: *const Object) -> ObjectId;
    
    #[link_name = "\u{1}?releaseExtensionDictionary@AcDbObject@@QEAA?AW4ErrorStatus@Acad@@XZ"]
    pub fn release_extension_dictionary(this: *mut Object) -> ErrorStatus;
    
    #[link_name = "\u{1}?upgradeOpen@AcDbObject@@QEAA?AW4ErrorStatus@Acad@@XZ"]
    pub fn upgrade_open(this: *mut Object) -> ErrorStatus;
    
    #[link_name = "\u{1}?upgradeFromNotify@AcDbObject@@QEAA?AW4ErrorStatus@Acad@@AEA_N@Z"]
    pub fn upgrade_from_notify( this: *mut Object, wasWritable: *mut bool) -> ErrorStatus;
    
    #[link_name = "\u{1}?downgradeOpen@AcDbObject@@QEAA?AW4ErrorStatus@Acad@@XZ"]
    pub fn downgrade_open(this: *mut Object) -> ErrorStatus;
    
    #[link_name = "\u{1}?downgradeToNotify@AcDbObject@@QEAA?AW4ErrorStatus@Acad@@_N@Z"]
    pub fn downgrade_to_notify( this: *mut Object, wasWritable: bool) -> ErrorStatus;
    
    #[link_name = "\u{1}?cancel@AcDbObject@@QEAA?AW4ErrorStatus@Acad@@XZ"]
    pub fn cancel(this: *mut Object) -> ErrorStatus;
    
    #[link_name = "\u{1}?close@AcDbObject@@QEAA?AW4ErrorStatus@Acad@@XZ"]
    pub fn close(this: *mut Object) -> ErrorStatus;
    
    #[link_name = "\u{1}?closeAndPage@AcDbObject@@QEAA?AW4ErrorStatus@Acad@@_N@Z"]
    pub fn close_and_page(this: *mut Object, onlyWhenClean: bool,
    ) -> ErrorStatus;
    
    #[link_name = "\u{1}?erase@AcDbObject@@QEAA?AW4ErrorStatus@Acad@@_N@Z"]
    pub fn erase(this: *mut Object, erasing: bool) -> ErrorStatus;
    
    #[link_name = "\u{1}?handOverTo@AcDbObject@@QEAA?AW4ErrorStatus@Acad@@PEAV1@_N1@Z"]
    pub fn hand_over_to( this: *mut Object, newObject: *mut Object, keepXData: bool, keepExtDict: bool) -> ErrorStatus;
    
    #[link_name = "\u{1}?swapIdWith@AcDbObject@@QEAA?AW4ErrorStatus@Acad@@VObjectId@@_N1@Z"]
    pub fn swap_id_with( this: *mut Object, otherId: ObjectId, swapXdata: bool, swapExtDict: bool) -> ErrorStatus;
    
    #[link_name = "\u{1}?dwgIn@AcDbObject@@QEAA?AW4ErrorStatus@Acad@@PEAVAcDbDwgFiler@@@Z"]
    pub fn dwg_in(this: *mut Object, pFiler: *mut DwgFiler) -> ErrorStatus;
    
    #[link_name = "\u{1}?dwgOut@AcDbObject@@QEBA?AW4ErrorStatus@Acad@@PEAVAcDbDwgFiler@@@Z"]
    pub fn dwg_out( this: *const Object, pFiler: *mut DwgFiler,
    ) -> ErrorStatus;
    
    #[link_name = "\u{1}?dxfIn@AcDbObject@@QEAA?AW4ErrorStatus@Acad@@PEAVAcDbDxfFiler@@@Z"]
    pub fn dxf_in(this: *mut Object, pFiler: *mut AcDbDxfFiler) -> ErrorStatus;
    
    #[link_name = "\u{1}?dxfOut@AcDbObject@@QEBA?AW4ErrorStatus@Acad@@PEAVAcDbDxfFiler@@_NPEBV?$AcArray@VObjectId@@V?$AcArrayMemCopyReallocator@VObjectId@@@@@@@Z"]
    pub fn dxf_out( this: *const Object, pFiler: *mut AcDbDxfFiler, bAllXdata: bool, pRegAppIds: *const ObjectIdArray) -> ErrorStatus;
    
    #[link_name = "\u{1}?xDataTransformBy@AcDbObject@@QEAA?AW4ErrorStatus@Acad@@AEBVAcGeMatrix3d@@@Z"]
    pub fn xdata_transform_by( this: *mut Object, xform: *const matrix3d) -> ErrorStatus;
    
    #[link_name = "\u{1}?setBinaryData@AcDbObject@@QEAA?AW4ErrorStatus@Acad@@PEB_WHPEBD@Z"]
    pub fn set_binary_data( this: *mut Object, key: *const wchar_t, size: i32, data: *const c_char) -> ErrorStatus;
    
    #[link_name = "\u{1}?getBinaryData@AcDbObject@@QEBA?AW4ErrorStatus@Acad@@PEB_WAEAHAEAPEAD@Z"]
    pub fn get_binary_data( this: *const Object, key: *const wchar_t, size: *mut i32, data: *mut *mut c_char) -> ErrorStatus;
    
    #[link_name = "\u{1}?isEraseStatusToggled@AcDbObject@@QEBA_NXZ"]
    pub fn is_erase_status_toggled(this: *const Object) -> bool;
    
    #[link_name = "\u{1}?isErased@AcDbObject@@QEBA_NXZ"]
    pub fn is_erased(this: *const Object) -> bool;
    
    #[link_name = "\u{1}?isReadEnabled@AcDbObject@@QEBA_NXZ"]
    pub fn is_read_enabled(this: *const Object) -> bool;
    
    #[link_name = "\u{1}?isWriteEnabled@AcDbObject@@QEBA_NXZ"]
    pub fn is_write_enabled(this: *const Object) -> bool;
    
    #[link_name = "\u{1}?isNotifyEnabled@AcDbObject@@QEBA_NXZ"]
    pub fn is_notify_enabled(this: *const Object) -> bool;
    
    #[link_name = "\u{1}?isModified@AcDbObject@@QEBA_NXZ"]
    pub fn is_modified(this: *const Object) -> bool;
    
    #[link_name = "\u{1}?isModifiedXData@AcDbObject@@QEBA_NXZ"]
    pub fn is_modified_xData(this: *const Object) -> bool;
    
    #[link_name = "\u{1}?isModifiedGraphics@AcDbObject@@QEBA_NXZ"]
    pub fn is_modified_graphics(this: *const Object) -> bool;
    
    #[link_name = "\u{1}?isNewObject@AcDbObject@@QEBA_NXZ"]
    pub fn is_new_object(this: *const Object) -> bool;
    
    #[link_name = "\u{1}?isNotifying@AcDbObject@@QEBA_NXZ"]
    pub fn is_notifying(this: *const Object) -> bool;
    
    #[link_name = "\u{1}?isUndoing@AcDbObject@@QEBA_NXZ"]
    pub fn is_undoing(this: *const Object) -> bool;
    
    #[link_name = "\u{1}?isCancelling@AcDbObject@@QEBA_NXZ"]
    pub fn is_cancelling(this: *const Object) -> bool;
    
    #[link_name = "\u{1}?isReallyClosing@AcDbObject@@QEBA_NXZ"]
    pub fn is_really_closing(this: *const Object) -> bool;
    
    #[link_name = "\u{1}?isTransactionResident@AcDbObject@@QEBA_NXZ"]
    pub fn is_transaction_resident(this: *const Object) -> bool;
    
    #[link_name = "\u{1}?isAProxy@AcDbObject@@QEBA_NXZ"]
    pub fn is_aProxy(this: *const Object) -> bool;
    
    #[link_name = "\u{1}?assertReadEnabled@AcDbObject@@QEBAXXZ"]
    pub fn assert_read_enabled(this: *const Object);
    
    #[link_name = "\u{1}?assertWriteEnabled@AcDbObject@@QEAAX_N0@Z"]
    pub fn assert_write_enabled( this: *mut Object, autoUndo: bool, recordModified: bool);
    
    #[link_name = "\u{1}?assertNotifyEnabled@AcDbObject@@QEBAXXZ"]
    pub fn assert_notify_enabled(this: *const Object);
    
    #[link_name = "\u{1}?isUndoRecordingDisabled@AcDbObject@@QEBA_NXZ"]
    pub fn is_undo_recording_disabled(this: *const Object) -> bool;
    
    #[link_name = "\u{1}?disableUndoRecording@AcDbObject@@QEAAX_N@Z"]
    pub fn disable_undo_recording(this: *mut Object, disable: bool);
    
    #[link_name = "\u{1}?undoFiler@AcDbObject@@QEAAPEAVAcDbDwgFiler@@XZ"]
    pub fn undo_filer(this: *mut Object) -> *mut DwgFiler;
    
    #[link_name = "\u{1}?addReactor@AcDbObject@@QEBA?AW4ErrorStatus@Acad@@PEAVAcDbObjectReactor@@@Z"]
    pub fn add_reactor( this: *const Object, pReactor: *mut ObjectReactor) -> ErrorStatus;
    
    #[link_name = "\u{1}?removeReactor@AcDbObject@@QEBA?AW4ErrorStatus@Acad@@PEAVAcDbObjectReactor@@@Z"]
    pub fn remove_reactor( this: *const Object, pReactor: *mut ObjectReactor) -> ErrorStatus;
    
    #[link_name = "\u{1}?addPersistentReactor@AcDbObject@@QEAA?AW4ErrorStatus@Acad@@VObjectId@@@Z"]
    pub fn add_persistent_reactor( this: *mut Object, objId: ObjectId) -> ErrorStatus;
    
    #[link_name = "\u{1}?removePersistentReactor@AcDbObject@@QEAA?AW4ErrorStatus@Acad@@VObjectId@@@Z"]
    pub fn remove_persistent_reactor( this: *mut Object, objId: ObjectId,) -> ErrorStatus;
    
    #[link_name = "\u{1}?hasReactor@AcDbObject@@QEBA_NPEBVAcDbObjectReactor@@@Z"]
    pub fn has_reactor( this: *const Object, pReactor: *const ObjectReactor) -> bool;
    
    #[link_name = "\u{1}?hasPersistentReactor@AcDbObject@@QEBA_NVObjectId@@@Z"]
    pub fn has_persistent_reactor(this: *const Object, objId: ObjectId) -> bool;
    
    #[link_name = "\u{1}?reactors@AcDbObject@@QEBAPEBV?$AcArray@PEAXV?$AcArrayMemCopyReallocator@PEAX@@@@XZ"]
    pub fn reactors(this: *const Object) -> *const VoidPtrArray;
    
    #[link_name = "\u{1}?setObjectIdsInFlux@AcDbObject@@QEAAXXZ"]
    pub fn set_ObjectIds_in_flux(this: *mut Object);
    
    #[link_name = "\u{1}?isObjectIdsInFlux@AcDbObject@@QEBA_NXZ"]
    pub fn is_ObjectIds_in_flux(this: *const Object) -> bool;
    
    #[link_name = "\u{1}?hasSaveVersionOverride@AcDbObject@@QEAA_NXZ"]
    pub fn has_save_version_vverride(this: *mut Object) -> bool;
    
    #[link_name = "\u{1}?setHasSaveVersionOverride@AcDbObject@@QEAAX_N@Z"]
    pub fn set_has_save_version_vverride(this: *mut Object, bSetIt: bool);
    
    #[link_name = "\u{1}?getObjectBirthVersion@AcDbObject@@QEAA?AW4ErrorStatus@Acad@@AEAW4AcDbDwgVersion@AcDb@@AEAW4MaintenanceReleaseVersion@5@@Z"]
    pub fn get_object_birth_version( this: *mut Object, ver: *mut DwgVersion, maintVer: *mut MaintenanceReleaseVersion,) -> ErrorStatus;
    
    #[link_name = "\u{1}?hasFields@AcDbObject@@QEBA_NXZ"]
    pub fn has_fields(this: *const Object) -> bool;
    
    #[link_name = "\u{1}?getField@AcDbObject@@QEBA?AW4ErrorStatus@Acad@@PEB_WAEAVObjectId@@@Z"]
    pub fn get_field( this: *const Object, pszPropName: *const wchar_t, fieldId: *mut ObjectId) -> ErrorStatus;
    
    #[link_name = "\u{1}?getField@AcDbObject@@QEBA?AW4ErrorStatus@Acad@@PEB_WAEAPEAVAcDbField@@W4OpenMode@AcDb@@@Z"]
    pub fn get_field1( this: *const Object, pszPropName: *const wchar_t, pField: *mut *mut Field, mode: OpenMode,) -> ErrorStatus;
    
    #[link_name = "\u{1}?getFieldDictionary@AcDbObject@@QEBA?AVObjectId@@XZ"]
    pub fn get_field_dictionary(this: *const Object) -> ObjectId;
    
    #[link_name = "\u{1}?getFieldDictionary@AcDbObject@@QEBA?AW4ErrorStatus@Acad@@AEAPEAVAcDbDictionary@@W4OpenMode@AcDb@@@Z"]
    pub fn get_field_dictionary1( this: *const Object, pFieldDict: *mut *mut Dictionary, mode: OpenMode) -> ErrorStatus;
    
    #[link_name = "\u{1}??0AcDbObject@@IEAA@PEAVAcDbSystemInternals@@@Z"]
    pub fn Object(this: *mut Object, arg1: *mut SystemInternals);
    
    #[link_name = "\u{1}??0AcDbObject@@IEAA@XZ"]
    fn  Object1(this: *mut Object);

    #[link_name = "\u{1}?isA@AcDbObject@@UEBAPEAVAcRxClass@@XZ"]
    pub fn is_a(this: *mut c_void) -> *mut acrx::Class;

    #[link_name = "\u{1}??_DAcDbObject@@QEAAXXZ"]
    pub fn destructor(this: *mut Object);

    #[link_name = "\u{1}?setOwnerId@AcDbObject@@UEAA?AW4ErrorStatus@Acad@@VObjectId@@@Z"]
    pub fn set_owner_id(
        this: *mut c_void,
        objId: ObjectId,
    ) -> ErrorStatus;

    #[link_name = "\u{1}?subOpen@AcDbObject@@UEAA?AW4ErrorStatus@Acad@@W4OpenMode@AcDb@@@Z"]
    pub fn sub_open(
        this: *mut c_void,
        mode: OpenMode,
    ) -> ErrorStatus;

    #[link_name = "\u{1}?subCancel@AcDbObject@@UEAA?AW4ErrorStatus@Acad@@XZ"]
    pub fn sub_cancel(this: *mut c_void) -> ErrorStatus;

    #[link_name = "\u{1}?subClose@AcDbObject@@UEAA?AW4ErrorStatus@Acad@@XZ"]
    pub fn sub_close(this: *mut c_void) -> ErrorStatus;

    #[link_name = "\u{1}?subErase@AcDbObject@@UEAA?AW4ErrorStatus@Acad@@_N@Z"]
    pub fn sub_erase(
        this: *mut c_void,
        erasing: Boolean,
    ) -> ErrorStatus;

    #[link_name = "\u{1}?subHandOverTo@AcDbObject@@UEAA?AW4ErrorStatus@Acad@@PEAV1@@Z"]
    pub fn sub_handover_to(
        this: *mut c_void,
        newObject: *mut Object,
    ) -> ErrorStatus;

    #[link_name = "\u{1}?subSwapIdWith@AcDbObject@@UEAA?AW4ErrorStatus@Acad@@VObjectId@@_N1@Z"]
    pub fn sub_swap_id_with(
        this: *mut c_void,
        otherId: ObjectId,
        swapXdata: Boolean,
        swapExtDict: Boolean,
    ) -> ErrorStatus;

    #[link_name = "\u{1}?swapReferences@AcDbObject@@UEAA?AW4ErrorStatus@Acad@@AEBVAcDbIdMapping@@@Z"]
    pub fn swap_references(
        this: *mut c_void,
        idMap: *const IdMapping,
    ) -> ErrorStatus;

    #[link_name = "\u{1}?audit@AcDbObject@@UEAA?AW4ErrorStatus@Acad@@PEAVAcDbAuditInfo@@@Z"]
    pub fn audit(
        this: *mut c_void,
        pAuditInfo: *mut AuditInfo,
    ) -> ErrorStatus;

    #[link_name = "\u{1}?dwgInFields@AcDbObject@@UEAA?AW4ErrorStatus@Acad@@PEAVAcDbDwgFiler@@@Z"]
    pub fn dwg_in_fields(
        this: *mut c_void,
        pFiler: *mut DwgFiler,
    ) -> ErrorStatus;

    #[link_name = "\u{1}?dwgOutFields@AcDbObject@@UEBA?AW4ErrorStatus@Acad@@PEAVAcDbDwgFiler@@@Z"]
    pub fn dwg_out_fields(
        this: *mut c_void,
        pFiler: *mut DwgFiler,
    ) -> ErrorStatus;

    #[link_name = "\u{1}?dxfInFields@AcDbObject@@UEAA?AW4ErrorStatus@Acad@@PEAVAcDbDxfFiler@@@Z"]
    pub fn dxf_in_fields(
        this: *mut c_void,
        pFiler: *mut AcDbDxfFiler,
    ) -> ErrorStatus;

    #[link_name = "\u{1}?dxfOutFields@AcDbObject@@UEBA?AW4ErrorStatus@Acad@@PEAVAcDbDxfFiler@@@Z"]
    pub fn dxf_out_fields(
        this: *mut c_void,
        pFiler: *mut AcDbDxfFiler,
    ) -> ErrorStatus;

    #[link_name = "\u{1}?mergeStyle@AcDbObject@@UEBA?AW4DuplicateRecordCloning@AcDb@@XZ"]
    pub fn merge_style(this: *mut c_void) -> DuplicateRecordCloning;

    #[link_name = "\u{1}?xData@AcDbObject@@UEBAPEAUresbuf@@PEB_W@Z"]
    pub fn xdata(
        this: *mut c_void,
        regappName: *const ACHAR,
    ) -> *mut _resbuf;

    #[link_name = "\u{1}?setXData@AcDbObject@@UEAA?AW4ErrorStatus@Acad@@PEBUresbuf@@@Z"]
    pub fn set_xdata(
        this: *mut c_void,
        xdata: *const _resbuf,
    ) -> ErrorStatus;

    #[link_name = "\u{1}?applyPartialUndo@AcDbObject@@UEAA?AW4ErrorStatus@Acad@@PEAVAcDbDwgFiler@@PEAVAcRxClass@@@Z"]
    pub fn apply_partial_undo(
        this: *mut c_void,
        undoFiler: *mut DwgFiler,
        classObj: *mut acrx::Class,
    ) -> ErrorStatus;

    #[link_name = "\u{1}?recvPropagateModify@AcDbObject@@UEAAXPEBV1@@Z"]
    pub fn recv_propagate_modify(
        this: *mut c_void,
        subObj: *const Object,
    );

    #[link_name = "\u{1}?xmitPropagateModify@AcDbObject@@UEBAXXZ"]
    pub fn xmit_propagate_modify(this: *mut c_void);

    #[link_name = "\u{1}?deepClone@AcDbObject@@UEBA?AW4ErrorStatus@Acad@@PEAV1@AEAPEAV1@AEAVAcDbIdMapping@@_N@Z"]
    pub fn deep_clone(
        this: *mut c_void,
        pOwnerObject: *mut Object,
        pClonedObject: *mut *mut Object,
        idMap: *mut IdMapping,
        isPrimary: Boolean,
    ) -> ErrorStatus;

    #[link_name = "\u{1}?wblockClone@AcDbObject@@UEBA?AW4ErrorStatus@Acad@@PEAVAcRxObject@@AEAPEAV1@AEAVAcDbIdMapping@@_N@Z"]
    pub fn wblock_clone(
        this: *mut c_void,
        pOwnerObject: *mut acrx::Object,
        pClonedObject: *mut *mut Object,
        idMap: *mut IdMapping,
        isPrimary: Boolean,
    ) -> ErrorStatus;

    #[link_name = "\u{1}?cancelled@AcDbObject@@UEAAXPEBV1@@Z"]
    pub fn cancelled(this: *mut c_void, dbObj: *const Object);

    #[link_name = "\u{1}?copied@AcDbObject@@UEAAXPEBV1@0@Z"]
    pub fn copied(
        this: *mut c_void,
        dbObj: *const Object,
        newObj: *const Object,
    );

    #[link_name = "\u{1}?erased@AcDbObject@@UEAAXPEBV1@_N@Z"]
    pub fn erased(
        this: *mut c_void,
        dbObj: *const Object,
        bErasing: Boolean,
    );

    #[link_name = "\u{1}?goodbye@AcDbObject@@UEAAXPEBV1@@Z"]
    pub fn goodbye(this: *mut c_void, dbObj: *const Object);

    #[link_name = "\u{1}?openedForModify@AcDbObject@@UEAAXPEBV1@@Z"]
    pub fn opened_for_modify(this: *mut c_void, dbObj: *const Object);

    #[link_name = "\u{1}?modified@AcDbObject@@UEAAXPEBV1@@Z"]
    pub fn modified(this: *mut c_void, dbObj: *const Object);

    #[link_name = "\u{1}?subObjModified@AcDbObject@@UEAAXPEBV1@0@Z"]
    pub fn sub_obj_modified(
        this: *mut c_void,
        dbObj: *const Object,
        subObj: *const Object,
    );

    #[link_name = "\u{1}?modifyUndone@AcDbObject@@UEAAXPEBV1@@Z"]
    pub fn modify_undone(this: *mut c_void, dbObj: *const Object);

    #[link_name = "\u{1}?modifiedXData@AcDbObject@@UEAAXPEBV1@@Z"]
    pub fn modified_xData(this: *mut c_void, dbObj: *const Object);

    #[link_name = "\u{1}?unappended@AcDbObject@@UEAAXPEBV1@@Z"]
    pub fn unappended(this: *mut c_void, dbObj: *const Object);

    #[link_name = "\u{1}?reappended@AcDbObject@@UEAAXPEBV1@@Z"]
    pub fn reappended(this: *mut c_void, dbObj: *const Object);

    #[link_name = "\u{1}?objectClosed@AcDbObject@@UEAAXVObjectId@@@Z"]
    pub fn object_closed(this: *mut c_void, objId: ObjectId);

    #[link_name = "\u{1}?modifiedGraphics@AcDbObject@@UEAAXPEBVAcDbEntity@@@Z"]
    pub fn modified_graphics(this: *mut c_void, dbEnt: *const acdb::Entity);

    #[link_name = "\u{1}?clone@AcDbObject@@UEBAPEAVAcRxObject@@XZ"]
    pub fn clone(this: *mut c_void) -> *mut acrx::Object;

    #[link_name = "\u{1}?copyFrom@AcDbObject@@UEAA?AW4ErrorStatus@Acad@@PEBVAcRxObject@@@Z"]
    pub fn copy_from(
        this: *mut c_void,
        pSrc: *const acrx::Object,
    ) -> ErrorStatus;

    #[link_name = "\u{1}?getObjectSaveVersion@AcDbObject@@UEAA?AW4ErrorStatus@Acad@@PEBVAcDbDwgFiler@@AEAW4AcDbDwgVersion@AcDb@@AEAW4MaintenanceReleaseVersion@6@@Z"]
    pub fn get_object_save_version(
        this: *mut c_void,
        pFiler: *const DwgFiler,
        ver: *mut DwgVersion,
        maintVer: *mut MaintenanceReleaseVersion,
    ) -> ErrorStatus;

    #[link_name = "\u{1}?getObjectSaveVersion@AcDbObject@@UEAA?AW4ErrorStatus@Acad@@PEBVAcDbDxfFiler@@AEAW4AcDbDwgVersion@AcDb@@AEAW4MaintenanceReleaseVersion@6@@Z"]
    pub fn get_object_save_version1(
        this: *mut c_void,
        pFiler: *const AcDbDxfFiler,
        ver: *mut DwgVersion,
        maintVer: *mut MaintenanceReleaseVersion,
    ) -> ErrorStatus;

    #[link_name = "\u{1}?decomposeForSave@AcDbObject@@UEAA?AW4ErrorStatus@Acad@@W4AcDbDwgVersion@AcDb@@AEAPEAV1@AEAVObjectId@@AEA_N@Z"]
    pub fn decompose_for_save(
        this: *mut c_void,
        ver: DwgVersion,
        replaceObj: *mut *mut Object,
        replaceId: *mut ObjectId,
        exchangeXData: *mut Boolean,
    ) -> ErrorStatus;

    #[link_name = "\u{1}?drawable@AcDbObject@@UEAAPEAVAcGiDrawable@@XZ"]
    pub fn drawable(this: *mut c_void) -> *mut Drawable;

    #[link_name = "\u{1}?isPersistent@AcDbObject@@UEBA_NXZ"]
    pub fn is_persistent(this: *mut c_void) -> Boolean;

    #[link_name = "\u{1}?id@AcDbObject@@UEBA?AVObjectId@@XZ"]
    pub fn id(this: *mut c_void) -> ObjectId;

    #[link_name = "\u{1}?getClassID@AcDbObject@@UEBA?AW4ErrorStatus@Acad@@PEAU_GUID@@@Z"]
    pub fn get_class_id(
        this: *mut c_void,
        pClsid: *mut CLSID,
    ) -> ErrorStatus;

    #[link_name = "\u{1}?setField@AcDbObject@@UEAA?AW4ErrorStatus@Acad@@PEB_WPEAVAcDbField@@AEAVObjectId@@@Z"]
    pub fn set_field(
        this: *mut c_void,
        pszPropName: *const ACHAR,
        pField: *mut Field,
        fieldId: *mut ObjectId,
    ) -> ErrorStatus;

    #[link_name = "\u{1}?removeField@AcDbObject@@UEAA?AW4ErrorStatus@Acad@@VObjectId@@@Z"]
    pub fn remove_field(
        this: *mut c_void,
        fieldId: ObjectId,
    ) -> ErrorStatus;

    #[link_name = "\u{1}?removeField@AcDbObject@@UEAA?AW4ErrorStatus@Acad@@PEB_WAEAVObjectId@@@Z"]
    pub fn remove_field1(
        this: *mut c_void,
        pszPropName: *const ACHAR,
        returnId: *mut ObjectId,
    ) -> ErrorStatus;

    #[link_name = "\u{1}?removeField@AcDbObject@@UEAA?AW4ErrorStatus@Acad@@PEB_W@Z"]
    pub fn remove_field2(
        this: *mut c_void,
        pszPropName: *const ACHAR,
    ) -> ErrorStatus;

    #[link_name = "\u{1}?subSetAttributes@AcDbObject@@MEAAIPEAVAcGiDrawableTraits@@@Z"]
    pub fn sub_set_attributes(
        this: *mut c_void,
        pTraits: *mut DrawableTraits,
    ) -> u32;

    #[link_name = "\u{1}?subWorldDraw@AcDbObject@@MEAA_NPEAVAcGiWorldDraw@@@Z"]
    pub fn sub_world_draw(
        this: *mut c_void,
        pWd: *mut WorldDraw,
    ) -> Boolean;

    #[link_name = "\u{1}?subViewportDraw@AcDbObject@@MEAAXPEAVAcGiViewportDraw@@@Z"]
    pub fn sub_viewport_draw(
        this: *mut c_void,
        pVd: *mut ViewportDraw,
    );

    #[link_name = "\u{1}?subGetClassID@AcDbObject@@MEBA?AW4ErrorStatus@Acad@@PEAU_GUID@@@Z"]
    pub fn sub_get_class_id(
        this: *mut c_void,
        pClsid: *mut CLSID,
    ) -> ErrorStatus;

    #[link_name = "\u{1}?subDeepClone@AcDbObject@@MEBA?AW4ErrorStatus@Acad@@PEAV1@AEAPEAV1@AEAVAcDbIdMapping@@_N@Z"]
    pub fn sub_deep_clone(
        this: *mut c_void,
        pOwnerObject: *mut Object,
        pClonedObject: *mut *mut Object,
        idMap: *mut IdMapping,
        isPrimary: Boolean,
    ) -> ErrorStatus;

    #[link_name = "\u{1}?subWblockClone@AcDbObject@@MEBA?AW4ErrorStatus@Acad@@PEAVAcRxObject@@AEAPEAV1@AEAVAcDbIdMapping@@_N@Z"]
    pub fn sub_wblock_clone(
        this: *mut c_void,
        pOwnerObject: *mut acrx::Object,
        pClonedObject: *mut *mut Object,
        idMap: *mut IdMapping,
        isPrimary: Boolean,
    ) -> ErrorStatus;
}


