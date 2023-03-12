use std::future::Future;
use std::any::Any;
use std::cell::Cell;
use std::pin::Pin;
use std::mem::MaybeUninit;
use std::process::Output;
use std::ptr::{addr_of_mut};

#[link(wasm_import_module = "asyncify")]
extern "C" {
	fn start_unwind(stack: *mut AsyncifyStack) -> !;
	fn stop_unwind();
	fn start_rewind(stack: *mut AsyncifyStack);
	fn stop_rewind();
}

#[repr(C)]
pub struct AsyncifyStack {
	current: *mut u8,
	end: *mut u8
}
#[repr(C)]
pub struct AsyncifyStackSized<const N: usize = 1000> {
	stack: AsyncifyStack,
	stack_data: [u8; N]
}

enum CoState {
	Unset,
	Normal(*mut AsyncifyStack),
	Unwinding(Box<dyn Future<Output = Box<dyn Any>>>),
	Rewinding(*mut AsyncifyStack, Box<dyn Any>)
}
impl Default for CoState {
	fn default() -> Self {
		Self::Unset
	}
}
thread_local! {
	static CO_STATE: Cell<CoState> = Cell::new(CoState::Unset);
}

impl<const N: usize> AsyncifyStackSized<N> {
	pub fn new_boxed() -> Pin<Box<Self>> {
		let mut ret = Box::new(Self {
			stack: AsyncifyStack {
				current: std::ptr::null_mut(),
				end: std::ptr::null_mut()
			},
			stack_data: [0; N]
		});
		ret.stack.current = addr_of_mut!(ret.stack_data[0]);
		ret.stack.end = unsafe { ret.stack.current.offset(N as isize) };

		Box::into_pin(ret)
	}
	pub async fn re_enter<T, F: FnMut() -> T>(mut self: Pin<&mut Self>, mut re_enter: F) -> T {
		let old_state = CO_STATE.replace(CoState::Normal(&mut self.stack));

		if let CoState::Unset = old_state {} else {
			panic!("state must be unset in order to start a coroutine.")
		}
		
		let mut res;
		loop {
			res = re_enter();
			let state = CO_STATE.take();
			match state {
				CoState::Normal(_) | CoState::Unset => break,
				CoState::Unwinding(f) => {
					let pinned = Box::into_pin(f);
					unsafe { stop_unwind(); }

					let res = pinned.await;
					
					unsafe { start_rewind(&mut self.stack); }
					CO_STATE.set(CoState::Rewinding(&mut self.stack, res));
				},
				_ => panic!("re_enter should never see a rewinding state: it sets that state.")
			}
		}
		
		res
	}
}

pub fn co_await_dyn(f: Box<dyn Future<Output = Box<dyn Any>>>) -> Box<dyn Any> {
	let state = CO_STATE.take();
	match state {
		CoState::Unset => panic!("No coroutine running"),
		CoState::Normal(stack) => {
			CO_STATE.set(CoState::Unwinding(f));
			unsafe { start_unwind(stack); }
		},
		CoState::Rewinding(stack, val) => {
			unsafe { stop_rewind() }
			CO_STATE.set(CoState::Normal(stack));

			val
		},
		_ => panic!("Can't call co_await while unwinding!")
	}
}
