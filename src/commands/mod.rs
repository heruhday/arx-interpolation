use crate::arx::editor::get_string;
use crate::error;
use crate::error::ArxError;
use crate::qeprintln;
use crate::arx::string::ToWideChars;


pub static mut scale: f64 = 1.0f64;
pub static mut scale2: f64 = 1.0f64;
pub static mut unit: f64 = 1.0f64;
pub static mut m: f64 = 1.0f64;
pub static mut digit: i32 = 3;
pub static mut global_scale: bool = true;

pub fn set_scale(){
    fn run() -> error::Result<(f64,f64,f64,f64)>{
        unsafe{
            let str = get_string(true, Some(&format!("\nScale (unit : m)")))?;
            let v = str.split(":").collect::<Vec<&str>>();
            if v.len() != 2 {
                return Err(Box::new(ArxError::CommandErr("failed to parse scale")));
            }
            let unit_ = v[0].parse::<f64>()?;
            let m_ = v[1].parse::<f64>()?;
            let scale_ = m / unit;
            let scale2_ = scale * scale;
            Ok((unit_, m_, scale_, scale2_))
        }
    }
   match run() {
       Ok((unit_, m_, scale_, scale2_)) => {
           unsafe {
               unit = unit_;
               m = m_;
               scale = scale_;
               scale2 = scale2_;
           }
       }
       Err(e) => {
               unsafe{qeprintln!("{}", e.to_string())}
           }
   }

}