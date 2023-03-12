#![feature(local_key_cell_methods, box_into_inner)]
use wasm_bindgen::prelude::*;

#[allow(unused)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
mod sqlite;
pub use sqlite::*;

mod vfs;
mod asyncify;



#[wasm_bindgen]
pub fn test_open(pathname: String) -> *mut sqlite3 {
	let mut ret = std::ptr::null_mut();
	let res = unsafe { sqlite3_open(pathname.as_ptr() as *const i8, &mut ret) };

	if res != 0 {
		if ret != std::ptr::null_mut() {
			unsafe { sqlite3_close(ret); }
		}
		panic!("")
	}

	ret
}
