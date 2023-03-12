use wasm_bindgen::prelude::*;
use std::mem::size_of;

use super::sqlite::*;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_name = getRandomBytes, js_namespace = crypto)]
	fn get_random_bytes(buf: &mut [u8]);

	#[wasm_bindgen(js_name = now, js_namespace = Date)]
	fn now() -> f64;
}

#[repr(C)]
pub struct File {
	inner: sqlite3_file
}

extern "C" fn open(_vfs: *mut sqlite3_vfs, _filename: *const i8, _file_out: *mut sqlite3_file, _flags_in: i32, _flags_out: *mut i32) -> i32 {
	todo!()
}
extern "C" fn delete(_vfs: *mut sqlite3_vfs, _filename: *const i8, _sync: i32) -> i32 {
	todo!()
}
extern "C" fn access(_vfs: *mut sqlite3_vfs, _filename: *const i8, _flags_in: i32, _res_out: *mut i32) -> i32 {
	todo!()
}
extern "C" fn full_pathname(_vfs: *mut sqlite3_vfs, _filename: *const i8, _buf_len: i32, _buf: *mut i8) -> i32 {
	todo!()
}
extern "C" fn randomness(_vfs: *mut sqlite3_vfs, buf_len: i32, buf: *mut i8) -> i32 {
	let buf = unsafe {
		std::slice::from_raw_parts_mut(buf as *mut u8, buf_len as usize)
	};
	get_random_bytes(buf);
	buf_len
}
extern "C" fn sleep(_vfs: *mut sqlite3_vfs, microseconds: i32) -> i32 {
	todo!()
}
extern "C" fn current_time(_vfs: *mut sqlite3_vfs, out: *mut i64) -> i32 {
	unsafe { *out = now() as i64 + 210866760000000; }
	0
}

#[no_mangle]
extern "C" fn sqlite3_os_init() -> i32 {
	let vfs = Box::into_raw(Box::new(sqlite3_vfs {
		iVersion: 2,
		szOsFile: size_of::<File>() as i32,
		mxPathname: 1024,
		pNext: std::ptr::null_mut(),
		zName: "default-vfs".as_ptr() as *const i8,
		pAppData: std::ptr::null_mut(),
		xOpen: Some(open),
		xDelete: Some(delete),
		xAccess: Some(access),
		xFullPathname: Some(full_pathname),
		xDlOpen: None,
		xDlError: None,
		xDlSym: None,
		xDlClose: None,
		xRandomness: Some(randomness),
		xSleep: Some(sleep),
		xCurrentTime: None,
		xGetLastError: None,
		xCurrentTimeInt64: Some(current_time),
		xSetSystemCall: None,
		xGetSystemCall: None,
		xNextSystemCall: None
	}));
	let res = unsafe { sqlite3_vfs_register(vfs, 1) };
	res
}
#[no_mangle]
extern "C" fn sqlite3_os_end() -> i32 { 0 }
