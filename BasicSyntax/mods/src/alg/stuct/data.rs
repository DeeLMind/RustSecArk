use crate::data::default::get_default as data;

pub fn get_data() -> i32 {
    data() * 2
}