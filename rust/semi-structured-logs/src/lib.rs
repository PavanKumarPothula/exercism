// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    if speed > 10 {panic!("Invalid speed")} 
    let standard_production_rate = (221);
    let success_rate = {if speed==0 {0.0} else{ if speed<=4 {1.0} else { if speed<=8 {0.9} else {0.77}}}};
    (speed as f64 * standard_production_rate as f64 * success_rate as f64) 
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
