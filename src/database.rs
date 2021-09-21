use crate::arx::acdb;
use crate::arx::string::ToWideChars;

pub fn reg_appname(appname: &str) {
    unsafe {
        acdb::reg_app(appname.to_wide());
    }
}

pub fn reg_appnames(appnames: &[&str]) {
    unsafe {
        for &app in appnames {
            acdb::reg_app(app.to_wide());
        }

    }
}