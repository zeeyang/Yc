extern crate yrs;

use std::ptr;
use std::os::raw::c_char;
use std::ops::{Deref, DerefMut};

unsafe fn from_buf_raw<T>(ptr: *const T, elts: usize) -> Vec<T> {
    let mut dst = Vec::with_capacity(elts);
    dst.set_len(elts);
    ptr::copy(ptr, dst.as_mut_ptr(), elts);
    dst
}

#[derive(Clone)]
pub struct Doc {
    doc: yrs::Doc,
    text: Option<String>,
    binary: Option<Vec<u8>>,
}

impl Doc {
    fn init(doc: yrs::Doc) -> Doc {
        Doc {
            doc,
            text: None,
            binary: None,
        }
    }
}

impl Deref for Doc {
    type Target = yrs::Doc;
    fn deref(&self) -> &Self::Target {
        &self.doc
    }
}

impl DerefMut for Doc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.doc
    }
}

impl From<Doc> for *mut Doc {
    fn from(d: Doc) -> Self {
        Box::into_raw(Box::new(d))
    }
}

#[no_mangle]
pub extern "C" fn yrs_init() -> *mut Doc {
    Doc::init(yrs::Doc::new()).into()
}

/// # Safety
/// This must be called with a valid doc pointer
#[no_mangle]
pub unsafe extern "C" fn yrs_free(doc: *mut Doc) {
    let doc: Doc = *Box::from_raw(doc);
    drop(doc)
}

/// # Safety
/// This must be called with a valid doc pointer
/// and buffer must be a valid pointer of at least the number of bytes returned by the previous
/// call that generated a text result
#[no_mangle]
pub unsafe extern "C" fn yrs_read_text_buffer(doc: *mut Doc, buffer: *mut c_char) {
    if let Some(text) = &(*doc).text {
        let len = text.len();
        buffer.copy_from(text.as_ptr().cast(), len);
        (*buffer.add(len)) = 0; // null terminate
        (*doc).text = None;
    }
}

/// # Safety
///
/// This must be called with a valid doc pointer
/// the buffer must be a valid pointer pointing to at least as much space as was
/// required by the previous binary result call
#[no_mangle]
pub unsafe extern "C" fn yrs_read_binary_buffer(doc: *mut Doc, buffer: *mut u8) {
    if let Some(bin) = &(*doc).binary {
        let len = bin.len();
        buffer.copy_from(bin.as_ptr(), len);
        (*doc).binary = None;
    }
}

/// # Safety
/// This must be called with a valid doc pointer
/// change must point to a valid memory location with at least len bytes
#[no_mangle]
pub unsafe extern "C" fn yrs_apply_update(doc: *mut Doc, len: usize, update: *const u8,) {
    let bytes = from_buf_raw(update, len);
    (*doc).apply_update(&bytes);
}

/// # Safety
/// This must be called with a valid doc pointer
#[no_mangle]
pub unsafe extern "C" fn yrs_encode_state_as_update(doc: *mut Doc) -> isize {
    let change = (*doc).encode_state_as_update();
    let len = change.len();
    (*doc).binary = Some(change);
    len as isize
}

/// # Safety
/// This must be called with a valid doc pointer
#[no_mangle]
pub unsafe extern "C" fn yrs_to_string(doc: *mut Doc) -> isize {
    let tp = (*doc).get_type("");
    let string = tp.to_string();
    let len = string.len();
    (*doc).text = Some(string);
    (len + 1) as isize
}

/// # Safety
/// This must be called with a valid doc pointer
#[no_mangle]
pub unsafe extern "C" fn yrs_insert(doc: *mut Doc, index: u32, change: c_char) {
    let tp = (*doc).get_type("");
    tp.insert(&(*doc).transact(), index, change as u8 as char);
}
