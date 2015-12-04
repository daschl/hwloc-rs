use libc::{c_char};

use ffi;
use std::fmt;
use std::ptr;
use std::ffi::CStr;

pub enum IntHwlocBitmap {}

pub struct HwlocBitmap {
	bitmap: *mut IntHwlocBitmap,
	owns_ptr: bool,
}

pub type CpuSet = HwlocBitmap;
pub type NodeSet = HwlocBitmap;

impl HwlocBitmap {

	pub fn new() -> HwlocBitmap {
		let int_bitmap = unsafe { ffi::hwloc_bitmap_alloc() };
		HwlocBitmap { bitmap: int_bitmap, owns_ptr: true }
	}

	pub fn from_raw(bitmap: *mut IntHwlocBitmap) -> HwlocBitmap {
		HwlocBitmap { bitmap: bitmap, owns_ptr: false }
	}

	/// Add index id in bitmap bitmap.
	pub fn set(&mut self, id: u32) {
		unsafe { ffi::hwloc_bitmap_set(self.bitmap, id) }
	}

	/// Add indexes from begin to end in this bitmap.
	///
	/// If end is -1, the range is infinite.
	pub fn set_range(&mut self, begin: u32, end: i32) {
		unsafe { ffi::hwloc_bitmap_set_range(self.bitmap, begin, end) }
	}

	/// Remove index id from bitmap bitmap.
	pub fn unset(&mut self, id:u32) {
		unsafe { ffi::hwloc_bitmap_clr(self.bitmap, id) }
	}

	/// Remove indexes from begin to end in this bitmap.
	///
	/// If end is -1, the range is infinite.
	pub fn unset_range(&mut self, begin: u32, end: i32) {
		unsafe { ffi::hwloc_bitmap_clr_range(self.bitmap, begin, end) }
	}

	/// The number of indexes that are in the bitmap.
	pub fn weight(&self) -> i32 {
		unsafe { ffi::hwloc_bitmap_weight(self.bitmap) }
	}

	/// Clears the complete bitmap.
	pub fn clear(&mut self) {
		unsafe { ffi::hwloc_bitmap_zero(self.bitmap) }
	}

	/// Checks if this bitmap has set fields.
	pub fn is_empty(&self) -> bool {
		let result = unsafe { ffi::hwloc_bitmap_iszero(self.bitmap) };
		if result == 0 { false } else { true }
	}

	/// Check if the field with the given id is set.
	pub fn is_set(&self, id: u32) -> bool {
		let result = unsafe { ffi::hwloc_bitmap_isset(self.bitmap, id) };
		if result == 0 { false } else { true }
	}

}

impl Drop for HwlocBitmap {

	fn drop(&mut self) {
		if self.owns_ptr {
			unsafe { ffi::hwloc_bitmap_free(self.bitmap) }
		}
	}

}

impl fmt::Display for HwlocBitmap {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut result: *mut c_char = ptr::null_mut();
		unsafe {
			ffi::hwloc_bitmap_list_asprintf(&mut result, self.bitmap);
			write!(f, "{}", CStr::from_ptr(result).to_str().unwrap())
		}
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn should_check_if_bitmap_is_empty() {
		let mut bitmap = HwlocBitmap::new();

		assert!(bitmap.is_empty());
		bitmap.set(1);
		assert!(!bitmap.is_empty());
		bitmap.unset(1);
		assert!(bitmap.is_empty());
	}

	#[test]
	fn should_set_and_unset_bitmap_index() {
		let mut bitmap = HwlocBitmap::new();
		assert_eq!("", format!("{}", bitmap));

		assert!(bitmap.is_empty());

		bitmap.set(1);
		bitmap.set(3);
		bitmap.set(8);
		assert_eq!("1,3,8", format!("{}", bitmap));
		assert!(!bitmap.is_empty());

		bitmap.unset(3);
		assert_eq!("1,8", format!("{}", bitmap));
		assert!(!bitmap.is_empty());
	}

	#[test]
	fn should_check_if_is_set() {
		let mut bitmap = HwlocBitmap::new();

		assert!(!bitmap.is_set(3));
		bitmap.set(3);
		assert!(bitmap.is_set(3));
		bitmap.unset(3);
		assert!(!bitmap.is_set(3));
	}

	#[test]
	fn should_set_and_unset_range() {
		let mut bitmap = HwlocBitmap::new();
		assert_eq!("", format!("{}", bitmap));

		bitmap.set_range(2, 5);
		assert_eq!("2-5", format!("{}", bitmap));

		bitmap.set_range(4, 7);
		assert_eq!("2-7", format!("{}", bitmap));

		bitmap.set(9);
		assert_eq!("2-7,9", format!("{}", bitmap));

		bitmap.unset_range(6, 10);
		assert_eq!("2-5", format!("{}", bitmap));
	}

	#[test]
	fn should_clear_the_bitmap() {
		let mut bitmap = HwlocBitmap::new();

		assert!(bitmap.is_empty());
		bitmap.set_range(4, 7);
		assert!(!bitmap.is_empty());
		assert!(bitmap.is_set(5));

		bitmap.clear();
		assert!(!bitmap.is_set(5));
		assert!(bitmap.is_empty());
	}

	#[test]
	fn should_get_weight() {
		let mut bitmap = HwlocBitmap::new();

		assert_eq!(0, bitmap.weight());

		bitmap.set(9);
		assert_eq!(1, bitmap.weight());

		bitmap.set_range(2, 5);
		assert_eq!(5, bitmap.weight());

		bitmap.unset(4);
		assert_eq!(4, bitmap.weight());

		bitmap.clear();
		assert_eq!(0, bitmap.weight());
	}

}