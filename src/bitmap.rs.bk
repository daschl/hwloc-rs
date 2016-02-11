use ffi;
use libc::c_char;
use std::{fmt, ptr};
use std::ffi::CStr;
use std::ops::Not;
use std::clone::Clone;
use std::iter::FromIterator;

pub enum IntHwlocBitmap {}

/// A generic bitmap, understood by hwloc.
///
/// The `Bitmap` represents a set of objects, typically OS processors – which may actually be
/// hardware threads (represented by `CpuSet`, which is a type alias for `Bitmap` – or memory
/// nodes (represented by `NodeSet`, which is also a typedef for `Bitmap`).
///
/// Both `CpuSet` and `NodeSet` are always indexed by OS physical number.
///
/// A `Bitmap` may be of infinite size.
pub struct Bitmap {
    bitmap: *mut IntHwlocBitmap,
    manage: bool,
}

/// A `CpuSet` is a `Bitmap` whose bits are set according to CPU physical OS indexes.
pub type CpuSet = Bitmap;
/// A `NodeSet` is a `Bitmap` whose bits are set according to NUMA memory node physical OS indexes.
pub type NodeSet = Bitmap;

impl Bitmap {

    /// Creates an empty `Bitmap`.
    ///
    /// Examples:
    ///
    /// ```
    /// use hwloc::Bitmap;
    ///
    /// let bitmap = Bitmap::new();
    /// assert_eq!("", format!("{}", bitmap));
    /// assert_eq!(true, bitmap.is_empty());
    // ```
    pub fn new() -> Bitmap {
        let bitmap = unsafe { ffi::hwloc_bitmap_alloc() };
        Bitmap { bitmap: bitmap, manage: true }
    }

    /// Creates a full `Bitmap`.
    ///
    /// Examples:
    ///
    /// ```
    /// use hwloc::Bitmap;
    ///
    /// let bitmap = Bitmap::full();
    /// assert_eq!("0-", format!("{}", bitmap));
    /// assert_eq!(false, bitmap.is_empty());
    // ```
    pub fn full() -> Bitmap {
        let bitmap = unsafe { ffi::hwloc_bitmap_alloc_full() };
        Bitmap { bitmap: bitmap, manage: true }
    }

    /// Creates a new HwlocBitmap (either CpuSet or NodeSet) and sets one index right away.
    ///
    /// Examples:
    ///
    /// ```
    /// use hwloc::Bitmap;
    ///
    /// let bitmap = Bitmap::from(1);
    /// assert_eq!("1", format!("{}", bitmap));
    /// assert_eq!(false, bitmap.is_empty());
    // ```
    pub fn from(id: u32) -> Bitmap {
        let mut bitmap = Bitmap::new();
        bitmap.set(id);
        bitmap
    }

    /// Creates a new `Bitmap` with the given range.
    ///
    /// Examples:
    ///
    /// ```
    /// use hwloc::Bitmap;
    ///
    /// let bitmap = Bitmap::from_range(0, 5);
    /// assert_eq!("0-5", format!("{}", bitmap));
    // ```
    pub fn from_range(begin: u32, end: i32) -> Bitmap {
        let mut bitmap = Bitmap::new();
        bitmap.set_range(begin, end);
        bitmap
    }

    /// Wraps the given hwloc bitmap pointer into its `Bitmap` representation.
    ///
    /// This function is not meant to be used directly, it rather serves as the
    /// conversion factory when dealing with hwloc-internal structures.
    pub fn from_raw(bitmap: *mut IntHwlocBitmap, manage: bool) -> Bitmap {
        Bitmap { bitmap: bitmap, manage: manage }
    }

    /// Returns the containted hwloc bitmap pointer for interaction with hwloc.
    pub fn as_ptr(&self) -> *const IntHwlocBitmap {
        self.bitmap as *const IntHwlocBitmap
    }

    /// Set index `id` in this `Bitmap`.
    ///
    /// Examples:
    ///
    /// ```
    /// use hwloc::Bitmap;
    ///
    /// let mut bitmap = Bitmap::new();
    /// bitmap.set(4);
    /// assert_eq!("4", format!("{}", bitmap));
    // ```
    pub fn set(&mut self, id: u32) {
        unsafe { ffi::hwloc_bitmap_set(self.bitmap, id) }
    }

    /// Add indexes from `begin` to `end` in this `Bitmap`.
    ///
    /// If end is -1, the range is infinite.
    ///
    /// Examples:
    ///
    /// ```
    /// use hwloc::Bitmap;
    ///
    /// let mut bitmap = Bitmap::new();
    /// bitmap.set_range(3, 5);
    /// assert_eq!("3-5", format!("{}", bitmap));
    ///
    /// bitmap.set_range(2, -1);
    /// assert_eq!("2-", format!("{}", bitmap));
    // ```
    pub fn set_range(&mut self, begin: u32, end: i32) {
        unsafe { ffi::hwloc_bitmap_set_range(self.bitmap, begin, end) }
    }

    /// Remove index `id` from the `Bitmap`.
    ///
    /// Examples:
    ///
    /// ```
    /// use hwloc::Bitmap;
    ///
    /// let mut bitmap = Bitmap::from_range(1,3);
    /// bitmap.unset(1);
    /// assert_eq!("2-3", format!("{}", bitmap));
    // ```
    pub fn unset(&mut self, id: u32) {
        unsafe { ffi::hwloc_bitmap_clr(self.bitmap, id) }
    }

    /// Remove indexes from `begin` to `end` in this `Bitmap`.
    ///
    /// If end is -1, the range is infinite.
    ///
    /// Examples:
    ///
    /// ```
    /// use hwloc::Bitmap;
    ///
    /// let mut bitmap = Bitmap::from_range(1,5);
    /// bitmap.unset_range(4,6);
    /// assert_eq!("1-3", format!("{}", bitmap));
    ///
    /// bitmap.unset_range(2,-1);
    /// assert_eq!("1", format!("{}", bitmap));
    // ```
    pub fn unset_range(&mut self, begin: u32, end: i32) {
        unsafe { ffi::hwloc_bitmap_clr_range(self.bitmap, begin, end) }
    }

    /// The number of indexes that are in the bitmap.
    ///
    /// Examples:
    ///
    /// ```
    /// use hwloc::Bitmap;
    ///
    /// let mut bitmap = Bitmap::from_range(1,5);
    /// assert_eq!(5, bitmap.weight());
    /// bitmap.unset(3);
    /// assert_eq!(4, bitmap.weight());
    /// ```
    pub fn weight(&self) -> i32 {
        unsafe { ffi::hwloc_bitmap_weight(self.bitmap) }
    }

    /// Clears the `Bitmap`.
    ///
    /// Examples:
    ///
    /// ```
    /// use hwloc::Bitmap;
    ///
    /// let mut bitmap = Bitmap::from_range(1,5);
    /// assert_eq!(5, bitmap.weight());
    /// assert_eq!(false, bitmap.is_empty());
    ///
    /// bitmap.clear();
    /// assert_eq!(0, bitmap.weight());
    /// assert_eq!(true, bitmap.is_empty());
    /// ```
    pub fn clear(&mut self) {
        unsafe { ffi::hwloc_bitmap_zero(self.bitmap) }
    }

    /// Checks if this `Bitmap` has indexes set.
    ///
    /// Examples:
    ///
    /// ```
    /// use hwloc::Bitmap;
    ///
    /// let mut bitmap = Bitmap::new();
    /// assert_eq!(true, bitmap.is_empty());
    ///
    /// bitmap.set(3);
    /// assert_eq!(false, bitmap.is_empty());
    ///
    /// bitmap.clear();
    /// assert_eq!(true, bitmap.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        let result = unsafe { ffi::hwloc_bitmap_iszero(self.bitmap) };
        if result == 0 {
            false
        } else {
            true
        }
    }

    /// Check if the field with the given id is set.
    ///
    /// Examples:
    ///
    /// ```
    /// use hwloc::Bitmap;
    ///
    /// let mut bitmap = Bitmap::new();
    /// assert_eq!(false, bitmap.is_set(2));
    ///
    /// bitmap.set(2);
    /// assert_eq!(true, bitmap.is_set(2));
    /// ```
    pub fn is_set(&self, id: u32) -> bool {
        let result = unsafe { ffi::hwloc_bitmap_isset(self.bitmap, id) };
        if result == 0 {
            false
        } else {
            true
        }
    }

    /// Keep a single index among those set in the bitmap.
    ///
    /// May be useful before binding so that the process does not have a
    /// chance of migrating between multiple logical CPUs in the original mask.
    ///
    /// Examples:
    ///
    /// ```
    /// use hwloc::Bitmap;
    ///
    /// let mut bitmap = Bitmap::new();
    /// bitmap.set_range(0, 127);
    /// assert_eq!(128, bitmap.weight());
    ///
    /// bitmap.invert();
    /// assert_eq!(-1, bitmap.weight());
    ///
    /// bitmap.singlify();
    /// assert_eq!(1, bitmap.weight());
    ///
    /// assert_eq!(128, bitmap.first());
    /// assert_eq!(128, bitmap.last());
    /// ```
    pub fn singlify(&mut self) {
        unsafe { ffi::hwloc_bitmap_singlify(self.bitmap) }
    }

    /// Inverts the current `Bitmap`.
    ///
    /// Examples:
    ///
    /// ```
    /// use hwloc::Bitmap;
    ///
    /// let mut bitmap = Bitmap::new();
    /// bitmap.set(3);
    ///
    /// assert_eq!("3", format!("{}", bitmap));
    /// assert_eq!("0-2,4-", format!("{}", !bitmap));
    /// ```
    pub fn invert(&mut self) {
        unsafe { ffi::hwloc_bitmap_not(self.bitmap, self.bitmap) }
    }

    /// Compute the first index (least significant bit) in this `Bitmap`.
    ///
    /// Returns -1 if no index is set.
    ///
    /// Examples:
    ///
    /// ```
    /// use hwloc::Bitmap;
    ///
    /// let bitmap = Bitmap::from_range(4,10);
    /// assert_eq!(4, bitmap.first());
    /// ```
    pub fn first(&self) -> i32 {
        unsafe { ffi::hwloc_bitmap_first(self.bitmap) }
    }

    /// Compute the last index (most significant bit) in this `Bitmap`.
    ///
    /// Returns -1 if no index is bitmap, or if the index bitmap is infinite.
    ///
    /// Examples:
    ///
    /// ```
    /// use hwloc::Bitmap;
    ///
    /// let bitmap = Bitmap::from_range(4,10);
    /// assert_eq!(10, bitmap.last());
    /// ```
    pub fn last(&self) -> i32 {
        unsafe { ffi::hwloc_bitmap_last(self.bitmap) }
    }

    /// Test whether this `Bitmap` is completely full.
    ///
    /// Examples:
    ///
    /// ```
    /// use hwloc::Bitmap;
    ///
    /// let empty_bitmap = Bitmap::new();
    /// assert_eq!(false, empty_bitmap.is_full());
    ///
    /// let full_bitmap = Bitmap::full();
    /// assert_eq!(true, full_bitmap.is_full());
    /// ```
    pub fn is_full(&self) -> bool {
        let result = unsafe { ffi::hwloc_bitmap_isfull(self.bitmap) };
        result == 1
    }
}

impl Not for Bitmap {
    type Output = Bitmap;

    /// Returns a new bitmap which contains the negated values of the current
    /// one.
    fn not(self) -> Bitmap {
        unsafe {
            let result = ffi::hwloc_bitmap_alloc();
            ffi::hwloc_bitmap_not(result, self.bitmap);
            Bitmap::from_raw(result, true)
        }
    }
}

impl Drop for Bitmap {
    fn drop(&mut self) {
        if self.manage {
            unsafe { ffi::hwloc_bitmap_free(self.bitmap) }
        }
    }
}

impl fmt::Display for Bitmap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result: *mut c_char = ptr::null_mut();
        unsafe {
            ffi::hwloc_bitmap_list_asprintf(&mut result, self.bitmap);
            write!(f, "{}", CStr::from_ptr(result).to_str().unwrap())
        }
    }
}

impl fmt::Debug for Bitmap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result: *mut c_char = ptr::null_mut();
        unsafe {
            ffi::hwloc_bitmap_list_asprintf(&mut result, self.bitmap);
            write!(f, "{}", CStr::from_ptr(result).to_str().unwrap())
        }
    }
}

impl Clone for Bitmap {
    fn clone(&self) -> Bitmap {
        let dup = unsafe { ffi::hwloc_bitmap_dup(self.bitmap) };
        Bitmap::from_raw(dup, true)
    }
}

impl PartialEq for Bitmap {
    fn eq(&self, other: &Self) -> bool {
        let result = unsafe { ffi::hwloc_bitmap_isequal(self.bitmap, other.as_ptr()) };
        result == 1
    }
}

impl IntoIterator for Bitmap {
    type Item = u32;
    type IntoIter = BitmapIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        BitmapIntoIterator { bitmap: self, index: -1 }
    }
}

pub struct BitmapIntoIterator {
    bitmap: Bitmap,
    index: i32,
}

impl Iterator for BitmapIntoIterator {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let result = unsafe { ffi::hwloc_bitmap_next(self.bitmap.as_ptr(), self.index) };
        self.index = result;
        if result < 0 {
            None
        } else {
            Some(result as u32)
        }
    }
}

impl FromIterator<u32> for Bitmap {
    fn from_iter<I: IntoIterator<Item = u32>>(iter: I) -> Bitmap {
        let mut bitmap = Bitmap::new();
        for i in iter {
            bitmap.set(i);
        }
        bitmap
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_check_if_bitmap_is_empty() {
        let mut bitmap = Bitmap::new();

        assert!(bitmap.is_empty());
        bitmap.set(1);
        assert!(!bitmap.is_empty());
        bitmap.unset(1);
        assert!(bitmap.is_empty());
    }

    #[test]
    fn should_create_by_range() {
        let bitmap = Bitmap::from_range(0, 5);
        assert_eq!("0-5", format!("{}", bitmap));
    }

    #[test]
    fn should_set_and_unset_bitmap_index() {
        let mut bitmap = Bitmap::new();
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
        let mut bitmap = Bitmap::new();

        assert!(!bitmap.is_set(3));
        bitmap.set(3);
        assert!(bitmap.is_set(3));
        bitmap.unset(3);
        assert!(!bitmap.is_set(3));
    }

    #[test]
    fn should_set_and_unset_range() {
        let mut bitmap = Bitmap::new();
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
        let mut bitmap = Bitmap::new();

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
        let mut bitmap = Bitmap::new();

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

    #[test]
    fn should_invert() {
        let mut bitmap = Bitmap::new();
        bitmap.set(3);

        assert_eq!("3", format!("{}", bitmap));
        assert_eq!("0-2,4-", format!("{}", !bitmap));
    }

    #[test]
    fn should_singlify() {
        let mut bitmap = Bitmap::new();
        bitmap.set_range(0, 127);
        assert_eq!(128, bitmap.weight());

        bitmap.invert();
        assert_eq!(-1, bitmap.weight());

        bitmap.singlify();
        assert_eq!(1, bitmap.weight());

        assert_eq!(128, bitmap.first());
        assert_eq!(128, bitmap.last());
    }

    #[test]
    fn should_check_equality() {
        let mut bitmap1 = Bitmap::new();
        bitmap1.set_range(0, 3);

        let mut bitmap2 = Bitmap::new();
        bitmap2.set_range(0, 3);

        let mut bitmap3 = Bitmap::new();
        bitmap3.set_range(1, 5);

        assert_eq!(bitmap1, bitmap2);
        assert!(bitmap2 == bitmap1);
        assert!(bitmap1 != bitmap3);
        assert!(bitmap3 != bitmap2);
    }

    #[test]
    fn should_clone() {
        let mut src = Bitmap::new();
        src.set_range(0, 3);

        let dst = src.clone();
        assert_eq!(src, dst);
    }

    #[test]
    fn should_support_into_iter() {
        let mut bitmap = Bitmap::from_range(4, 8);
        bitmap.set(2);

        let collected = bitmap.into_iter().collect::<Vec<u32>>();
        assert_eq!(6, collected.len());
        assert_eq!(vec![2, 4, 5, 6, 7, 8], collected);
    }

    #[test]
    fn should_support_from_iter() {
        let bitmap = (1..10).collect::<Bitmap>();
        assert_eq!("1-9", format!("{}", bitmap));
    }

}
