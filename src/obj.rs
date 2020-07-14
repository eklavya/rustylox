use crate::{gc::GC, value::Value};
use std::alloc::Layout;
use std::mem;

pub trait Object {
    fn get_type(&self) -> ObjType;
}

#[derive(Debug, PartialEq, Eq)]
pub enum ObjType {
    OString,
}

pub struct ObjString {
    pub chars: *mut u8,
    pub len: usize,
}

impl Object for ObjString {
    fn get_type(&self) -> ObjType {
        ObjType::OString
    }
}

pub fn is_obj_type(v: Value, obj_type: ObjType) -> bool {
    match v {
        Value::Bool(_) => false,
        Value::Number(_) => false,
        Value::Obj(o) => unsafe { (*o).get_type() == obj_type },
        Value::Empty => false,
    }
}

pub fn print_obj(obj: *const dyn Object) -> () {
    unsafe {
        match (*obj).get_type() {
            ObjType::OString => {
                let o = (obj as *const ObjString).read();
                let utf = std::slice::from_raw_parts_mut(o.chars, o.len);
                println!("{:?}", std::str::from_utf8(utf).unwrap());
            }
        }
    }
}

pub fn copy_string(s: &str) -> *mut ObjString {
    let chars = GC::alloc(Layout::array::<u8>(s.len()).unwrap());
    unsafe {
        std::ptr::copy::<u8>(s.as_bytes().as_ptr(), chars, s.len());
    }
    allocate_string(chars, s.len())
}

pub fn allocate_string(chars: *mut u8, len: usize) -> *mut ObjString {
    let os = GC::alloc(Layout::new::<ObjString>()) as *mut ObjString;
    unsafe {
        (*os).len = len;
        (*os).chars = chars;
    };
    os
}
