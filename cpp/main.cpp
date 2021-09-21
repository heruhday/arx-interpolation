#include <accmd.h>
#include <dbobjptr.h> 
#include <Windows.h>
#include <rxobject.h>
#include <rxregsvc.h>
#include <aced.h>
#include <dbsymtb.h>
#include <dbapserv.h>
#include <adslib.h>
#include <actrans.h>
#include "tchar.h"

extern "C" void _add_command(const ACHAR * cmdGroupName, const ACHAR * cmdGlobalName, const ACHAR * cmdLocalName,  Adesk::Int32 commandFlags, AcRxFunctionPtr FunctionAddr){
	acedRegCmds->addCommand(cmdGroupName, cmdGlobalName, cmdLocalName, ACRX_CMD_TRANSPARENT, FunctionAddr);
}

extern "C" void _remove_group(const ACHAR * groupName){
     acedRegCmds->removeGroup(groupName);
}


extern "C" AcTransaction*  start_transaction(){
	return actrTransactionManager->startTransaction();
}
extern "C" Acad::ErrorStatus  end_transaction(){
	return actrTransactionManager->endTransaction();
}
extern "C" Acad::ErrorStatus  abort_transaction(){
	return actrTransactionManager->abortTransaction();
}
extern "C" Acad::ErrorStatus  transaction_get_object(AcTransaction *pTrans, AcDbObject*& obj, AcDbObjectId id, AcDb::OpenMode mode, bool openErasedObj){
	return pTrans->getObject(*&obj, id, mode, openErasedObj);

}

extern "C" Acad::ErrorStatus _get_length(const ads_name ent, double& length) {
	AcDbObjectId id;
	Acad::ErrorStatus rs;
	rs = acdbGetObjectId(id, ent);
	if (rs == Acad::eOk) {
		AcDbObjectPointer<AcDbCurve> pCurve(id, AcDb::kForRead);
		rs = pCurve.openStatus();
		if (rs == Acad::eOk) {
			double startParam, endParam, startDist, endDist;
			rs = pCurve->getStartParam(startParam);
			rs = pCurve->getEndParam(endParam);
			rs = pCurve->getDistAtParam(startParam, startDist);
			rs = pCurve->getDistAtParam(endParam, endDist);
			length = endDist - startDist;
		}
		else {

		}
		pCurve->close();
	}
	return rs;
}

extern "C" bool acdb_curve_is_closed(AcDbCurve* pCurve){
	return pCurve->isClosed();
}

extern "C" Acad::ErrorStatus acdb_curve_get_start_param(AcDbCurve* pCurve, double& param) {
	return pCurve->getStartParam(param);
}

extern "C" Acad::ErrorStatus acdb_curve_get_end_param(AcDbCurve* pCurve, double& param) {
	return pCurve->getEndParam(param);
}

extern "C" Acad::ErrorStatus acdb_curve_get_start_point(AcDbCurve* pCurve, AcGePoint3d& point) {
	return pCurve->getStartPoint(point);
}

extern "C" Acad::ErrorStatus acdb_curve_get_end_point(AcDbCurve* pCurve, AcGePoint3d& point) {
	return pCurve->getEndPoint(point);
}

extern "C" Acad::ErrorStatus acdb_curve_get_point_at_param(AcDbCurve* pCurve, double param, AcGePoint3d& point){
	return pCurve->getPointAtParam(param, point);
}

extern "C" Acad::ErrorStatus acdb_curve_get_param_at_point(AcDbCurve* pCurve, const AcGePoint3d& point, double& param){
	return pCurve->getParamAtPoint(point, param);
}

extern "C" Acad::ErrorStatus acdb_curve_get_dist_at_param(AcDbCurve* pCurve, double param, double& dist){
	return pCurve->getDistAtParam(param, dist);
}

extern "C" Acad::ErrorStatus acdb_curve_get_param_at_dist(AcDbCurve* pCurve, double dist, double& param){
	return pCurve->getParamAtDist(dist, param);
}

extern "C" Acad::ErrorStatus acdb_curve_get_point_at_dist(AcDbCurve* pCurve, double dist, AcGePoint3d& point){
	return pCurve->getPointAtParam(dist, point);
}

extern "C" Acad::ErrorStatus acdb_curve_get_dist_at_point(AcDbCurve* pCurve, const AcGePoint3d& point, double& dist){
	return pCurve->getParamAtPoint(point, dist);
}

extern "C" Acad::ErrorStatus acdb_curve_get_closest_point_to(AcDbCurve* pCurve, const AcGePoint3d& givenPoint, AcGePoint3d& pointOnCurve, Adesk::Boolean extend){
	return pCurve->getClosestPointTo(givenPoint, pointOnCurve, extend);
}

extern "C" Acad::ErrorStatus acdb_curve_get_area(AcDbCurve* pCurve, double& area){
	return pCurve->getArea(area);
}

extern "C" Acad::ErrorStatus acdb_curve_get_area_length(AcDbCurve* pCurve, double& area , double& length){
	Acad::ErrorStatus rs;
	rs = pCurve->getArea(area);
	if (rs != Acad::eOk) {
		return rs;
	}
	double startParam, endParam, startDist, endDist;
	rs = pCurve->getStartParam(startParam);
	rs = pCurve->getEndParam(endParam);
	rs = pCurve->getDistAtParam(startParam, startDist);
	rs = pCurve->getDistAtParam(endParam, endDist);
	length = endDist - startDist;
	return rs;
}

extern "C" Acad::ErrorStatus acdb_curve_get_length(AcDbCurve* pCurve, double& length){
	Acad::ErrorStatus rs;
	double startParam, endParam, startDist, endDist;
	rs = pCurve->getStartParam(startParam);
	rs = pCurve->getEndParam(endParam);
	rs = pCurve->getDistAtParam(startParam, startDist);
	rs = pCurve->getDistAtParam(endParam, endDist);
	length = endDist - startDist;
	return rs;
}


extern "C" bool acdb_object_cast_to_acdb_curve(AcDbObject *obj, AcDbCurve** pCurve){
	if( (*pCurve = AcDbCurve::cast(obj)) != NULL )
	{
		return true;
	}else {
		return false;
	}
}

extern "C" Acad::ErrorStatus _get_area(const ads_name ent, double& area) {
	AcDbObjectId id;
	Acad::ErrorStatus rs;
	rs = acdbGetObjectId(id, ent);
	if (rs == Acad::eOk) {
		
		AcDbObjectPointer<AcDbCurve> pCurve(id, AcDb::kForRead);
		rs = pCurve.openStatus();
		if (rs == Acad::eOk) {
			rs = pCurve->getArea(area);
		}
		else {

		}
		pCurve->close();
	}
	return rs;
}


extern "C" Acad::ErrorStatus _get_area_length(const ads_name ent, double& area, double& length) {
	AcDbObjectId id;
	Acad::ErrorStatus rs;
	rs = acdbGetObjectId(id, ent);
	if (rs == Acad::eOk) {
		AcDbObjectPointer<AcDbCurve> pCurve(id, AcDb::kForRead);
		rs = pCurve.openStatus();
		if (rs == Acad::eOk) {
			rs = pCurve->getArea(area);
			double startParam, endParam, startDist, endDist;
			rs = pCurve->getStartParam(startParam);
			rs = pCurve->getEndParam(endParam);
			rs = pCurve->getDistAtParam(startParam, startDist);
			rs = pCurve->getDistAtParam(endParam, endDist);
			length = endDist - startDist;
		}

		pCurve->close();
	}
	return rs;
}