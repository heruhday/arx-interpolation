use crate::arx::acdb::open_mode::kForWrite;
use crate::arx::aced::ret_list;
use crate::arx::object::{Object, TryOpenFromRaw};
use crate::arx::resbuf::{resbuf, Resbuf};
use crate::arx::rlname::EntityKind;
use crate::arx::ssname::SSName;
use crate::database::reg_appname;
use crate::{arx, entry::*, qprintln};
use crate::error;
use crate::transaction::Transaction;
use crate::arx::string::ToWideChars;
use makima_spline::Spline;
use crate::arx::acut;


use crate::arx::acut::*;
pub fn get_msg(_rb: resbuf) -> error::Result<()> {
    unsafe {
        if let Some(_) = *MSG.lock().unwrap() {
            let msg = format!("{}\n", "msg has initilize!").to_wide();
            printf(msg);
        } else {
            let msg = format!("{}\n", "msg has not initilize yet!").to_wide();
            printf(msg);
        }
    }

    Ok(())
}

pub fn set_int(_rb: resbuf) -> error::Result<()> {
    let rb = Resbuf::from_raw(_rb)?;
    let trans = Transaction::start();
    let _rs = _set_int(&trans, rb.into_inner())?;
    Ok(())
}

pub fn _set_int(trans: &Transaction, rb: resbuf) -> error::Result<()> {
    let mut rb = Resbuf::from_raw(rb)?;
    let e = rb.get_name()?;
    rb.next()?;
    let app = rb.get_string()?;
    let _rs = reg_appname(&app);
    rb.next()?;
    let b = rb.get_int()?;
    match e {
        EntityKind::Ename(e) => {
            let mut o = Object::trans_try_open_from_raw(trans, e, kForWrite)?;
            o.set_byte_xdata(&app, &b.to_be_bytes())?;
        }
        EntityKind::SSname(ss) => {
            let ss = SSName::from_raw(ss)?;
            for i in 0..ss.len()? {
                let mut o = Object::trans_try_open_from_raw(trans, ss.name(i)?, kForWrite)?;
                o.set_byte_xdata(&app, &b.to_be_bytes())?;
            }
        }
    };
    Ok(())
}



pub fn print_code(_rb: resbuf) -> error::Result<()> {
    let mut rb = Resbuf::from_raw(_rb)?;
    
    rb.print_code()?;
    
    Ok(())
}

pub fn get_codes(_rb: resbuf) -> error::Result<()> {
    let mut rb = Resbuf::from_raw(_rb)?;
    
    let rs = rb.get_codes()?;
    let rs= unsafe { ret_list(rs)};
    unsafe{qprintln!("rs: {}", rs);}
    Ok(())
}


pub fn get_points2(_rb: resbuf) -> error::Result<()> {
    unsafe{
        let mut rb = Resbuf::from_raw(_rb)?;
        let v= rb.get_points()?;
        // qprintln!("v: {:?}", v);
    
        let delta = rb.get_double()?;rb.next()?;
        // qprintln!("delta: {}", delta);
        
        let len = rb.get_double()?;rb.next()?;
        // qprintln!("tol: {}", tol);
        
        rb.set_back(_rb)?;
        let rs = get_in_points2(v, delta, len);
        let rbres = acut::new_rb_(arx::RTLB);
        let mut rb_cur = rbres;

        for x in rs {
            let mut pt = [0f64; 3usize];
            pt[0] = x.0;
            pt[1] = x.1;
            (*rb_cur).rbnext = acut::new_rb_(arx::RTPOINT);
            rb_cur = (*rb_cur).rbnext;
            (*rb_cur).resval.rpoint = pt;
        }
        (*rb_cur).rbnext = acut::new_rb_(arx::RTLE);
        let rs =arx::aced::ret_list(rbres);
        qprintln!("return list: {}", rs);
    
        Ok(())
        
    }
}


fn get_in_points2(v: Vec<(f64, f64)>,  delta: f64,  len: f64) -> Vec<(f64, f64)> {
    let mut x = vec![];
    let mut y = vec![];
    for xx in v {
        x.push(xx.0);
        y.push(xx.1);
    }
    let points = makima_spline::vec_to_points(&x, &y);
    let spline = Spline::from_vec(points);
    let mut res = Vec::new();
    
    // let itermax = 1000;
    // let delta_x_min = 0.01;
    // let delta = 0.1f64;
    // let tol = 1.0e-4;
    let mut pt = 0f64;
    while pt < len {
        let mut hi = spline.sample(pt);  
        res.push(((pt*1000f64).round()/1000f64, (hi*1000f64).round()/1000f64));
      pt += delta;
    }
        
   res
}


pub fn get_points(_rb: resbuf) -> error::Result<()> {
unsafe{
    let mut rb = Resbuf::from_raw(_rb)?;
    let v= rb.get_points()?;
    // qprintln!("v: {:?}", v);

    let itermax = rb.get_integer()?;rb.next()?;
    // qprintln!("itermax: {}", itermax);
    
    let delta = rb.get_double()?;rb.next()?;
    // qprintln!("delta: {}", delta);
    
    let tol = rb.get_double()?;rb.next()?;
    // qprintln!("tol: {}", tol);
    
    let delta_x_min = rb.get_double()?;rb.next()?;
    // qprintln!("xmin: {}", delta_x_min);
    
    rb.set_back(_rb)?;
    
    // qprintln!("itermax: {}, delta: {}, tol: {}, deltax_min: {}", itermax, delta, tol, delta_x_min );
    let rs = get_in_points(v, itermax, delta, tol,delta_x_min);
    
    let rbres = acut::new_rb_(arx::RTLB);
    let mut rb_cur = rbres;

    for x in rs {
        let mut pt = [0f64; 3usize];
        pt[0] = x.0;
        pt[1] = x.1;
        (*rb_cur).rbnext = acut::new_rb_(arx::RTPOINT);
        rb_cur = (*rb_cur).rbnext;
        (*rb_cur).resval.rpoint = pt;
    }
    (*rb_cur).rbnext = acut::new_rb_(arx::RTLE);
    let rs =arx::aced::ret_list(rbres);
    qprintln!("return list: {}", rs);
    Ok(())
}}

fn get_in_points(v: Vec<(f64, f64)>, itermax: i32, delta: f64, tol: f64, delta_x_min: f64) -> Vec<(f64, f64)> {
    let mut x = vec![];
    let mut y = vec![];
    for xx in v {
        x.push(xx.0);
        y.push(xx.1);
    }
    let points = makima_spline::vec_to_points(&x, &y);
    let spline = Spline::from_vec(points);
    let mut res = Vec::new();
    
    // let itermax = 1000;
    // let delta_x_min = 0.01;
    // let delta = 0.1f64;
    // let tol = 1.0e-4;
    for i in 0..x.len()-1{
        let j = i + 1;    
        let x0 = x[i];
        let x1 = x[j];
        let base = y[i];
        // let dlt = (y[j] - y[i]) / nseg as f64;
        let dlt = if y[j] < y[i] {-delta}else {delta};        
        
        if  (y[j] - y[i]).abs() > 0.001 {
            // println!("i : {}, delta: {}, yj: {}, yi: {}", i, dlt,y[j] , y[i]);
            let nseg = ((y[j] - y[i]) / dlt).round() as u32;
            for ix in 1 .. nseg {
                let h=  base + (ix as f64)*dlt;
                let mut xi = x0;
                let mut xj = x1;
                let mut _hi = spline.sample(xi);
                let mut _hj = spline.sample(xj);
                let mut _xx = 0.0f64;
                let mut hmid = 0f64;
                let mut xmid = 0f64;
                let mut iter = 0;
                if _hj > _hi {
                    while iter < itermax {
                        iter += 1;
                
                        xmid = (xj+xi)/ 2.0f64;
                        hmid = spline.sample(xmid);
                        if (h-hmid).abs() < tol {
                            _xx = xmid;
                            break;
                        }
                        if hmid > h {
                            xj = xmid;
                            _hj = spline.sample(xj);
                        }else {
                            xi = xmid;
                            _hi = spline.sample(xi);
                        }
                    } 
                }else if _hj < _hi  {
                    while iter < itermax {
                        iter += 1;
                        
                        xmid = (xj+xi)/ 2.0f64;
                        hmid = spline.sample(xmid);
                        if (h-hmid).abs() < tol {
                            _xx = xmid;
                            break;
                        }
                        if hmid < h {
                            xj = xmid;
                            _hj = spline.sample(xj);
                        }else {
                            xi = xmid;
                            _hi = spline.sample(xi);
                        }
                    } 
                }
                res.push(((xmid*1000f64).round()/1000f64, (hmid*1000f64).round()/1000f64));
                // println!("x: {:.3}, h: {:.3}", xmid, hmid);
        
            }
        }else {
            let mut dx = x[i];
            let x0 = x[i];
            let h0 = spline.sample(x0);
            let x1 = x[j];
            let xmid = (x0 + x1) / 2 as f64;
            let hmid = spline.sample(xmid);
            let mut hpeak = h0;
            if hmid < h0 {
                loop {
                    let hdx = spline.sample(dx);
                    if hpeak > hdx {
                        hpeak = hdx
                    }

                    dx += delta_x_min;
                    if dx > x1 {break;}
                }
                if (hpeak - h0).abs() < delta{
                    // println!("peak in range");    
                }else {
                    // let nx =  ((hpeak - h0) / delta) as u32;
                    // println!("peak in multi range delta");    
                }
                // println!("hpeak (-) = {}, h0: {}", hpeak, h0);
            }else if hmid > h0  {
                loop {
                    let hdx = spline.sample(dx);
                    if hpeak < hdx {
                        hpeak = hdx
                    }

                    dx += delta_x_min;
                    if dx > x1 {break;}
                }
                if (hpeak - h0).abs() < delta{
                    // println!("peak in range");    
                }else {
                    // println!("peak in multi range delta");    
                }
                // println!("hpeak (+) = {}, h0: {}", hpeak, h0);
            }
            
            
        }
        
    }
        
   res
}
