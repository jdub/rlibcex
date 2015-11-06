pub use super::libc::*;

// http://pubs.opengroup.org/onlinepubs/9699919799/basedefs/string.h.html

#[no_mangle]
pub unsafe extern fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char {
    let n = strlen(src) + 1;
    let mut i = 0;
    while i < n {
        *dst.offset(i as isize) = *src.offset(i as isize);
        i += 1;
    }
    dst
}

/*
#[no_mangle]
pub unsafe extern fn strncpy(dst: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char { 0 }
*/

#[no_mangle]
pub unsafe extern fn strcat(dst: *mut c_char, src: *const c_char) -> *mut c_char {
    strcpy(dst.offset(strlen(dst) as isize), src)
}

/*
#[no_mangle]
pub unsafe extern fn strncat(dst: *mut c_char, src: *const c_char, mut n: size_t) -> *mut c_char { 0 }
*/

#[no_mangle]
pub unsafe extern fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int {
    let mut cs_ptr = cs;
    let mut ct_ptr = ct;

    while *cs_ptr != 0 && *cs_ptr == *ct_ptr {
        cs_ptr = cs_ptr.offset(1);
        ct_ptr = ct_ptr.offset(1);
    }
    (*cs_ptr - *ct_ptr) as c_int
}

#[no_mangle]
pub unsafe extern fn strncmp(cs: *const c_char, ct: *const c_char, mut n: size_t) -> c_int {
    let mut cs_ptr = cs;
    let mut ct_ptr = ct;

    // FIXME: checking every byte is slow, grab per-word optimisation from glibc
    while n > 0 {
        if *cs_ptr == 0 || *cs_ptr != *ct_ptr {
            break;
        }
        cs_ptr = cs_ptr.offset(1);
        ct_ptr = ct_ptr.offset(1);
        n -= 1;
    }

    (*cs_ptr - *ct_ptr) as c_int
}

/*
#[no_mangle]
pub unsafe extern fn strcoll(cs: *const c_char, ct: *const c_char) -> c_int { 0 }

#[no_mangle]
pub unsafe extern fn strchr(cs: *const c_char, c: c_int) -> *mut c_char { 0 }

#[no_mangle]
pub unsafe extern fn strrchr(cs: *const c_char, c: c_int) -> *mut c_char { 0 }

#[no_mangle]
pub unsafe extern fn strspn(cs: *const c_char, ct: *const c_char) -> size_t { 0 }

#[no_mangle]
pub unsafe extern fn strcspn(cs: *const c_char, ct: *const c_char) -> size_t { 0 }

#[no_mangle]
pub unsafe extern fn strpbrk(cs: *const c_char, ct: *const c_char) -> *mut c_char { 0 }

#[no_mangle]
pub unsafe extern fn strstr(cs: *const c_char, ct: *const c_char) -> *mut c_char { 0 }
*/

#[no_mangle]
pub unsafe extern fn strlen(cs: *const c_char) -> size_t {
    let mut ptr = cs;

    // FIXME: checking every byte is slow, grab per-word optimisation from glibc
    while *ptr != 0 {
        ptr = ptr.offset(1);
    }

    ptr as size_t - cs as size_t
}

/*
#[no_mangle]
pub unsafe extern fn strerror(n: c_int) -> *mut c_char { 0 }

#[no_mangle]
pub unsafe extern fn strtok(s: *mut c_char, t: *const c_char) -> *mut c_char { 0 }

#[no_mangle]
pub unsafe extern fn strxfrm(s: *mut c_char, ct: *const c_char, n: size_t) -> size_t { 0 }
*/


#[cfg(test)]
mod test {
    use super::*;
    use test::black_box;

    #[test]
    fn test_strcpy() {
        let orig: &[c_char; 7] = &black_box([48, 49, 50, 5, 10, 15, 0]);
        let src: &[c_char; 7] = &black_box([1, 2, 3, 4, 5, 6, 0]);
        let dst: &mut [c_char; 7] = &mut black_box([48, 49, 50, 5, 10, 15, 0]);

        unsafe {
            assert_eq!(dst, orig);
            strcpy(dst.as_mut_ptr(), src.as_ptr());
            assert_eq!(dst, src);
        }
    }

    #[test]
    fn test_strcat() {
        let dst: &mut [c_char; 13] = &mut black_box([48, 49, 50, 5, 10, 15, 0, 0, 0, 0, 0, 0, 0]);
        let one: &[c_char; 7] = &black_box([1, 2, 3, 4, 5, 6, 0]);
        let one_res: &[c_char; 13] = &black_box([48, 49, 50, 5, 10, 15, 1, 2, 3, 4, 5, 6, 0]);

        unsafe {
            strcat(dst.as_mut_ptr(), one.as_ptr());
            assert_eq!(dst, one_res);
        }
    }

    /*
    #[test]
    fn test_strncat() {
        let dst: &mut [c_char; 10] = &mut black_box([48, 49, 50, 5, 10, 15, 0, 0, 0, 0]);
        let one: &[c_char; 7] = &black_box([1, 2, 3, 4, 5, 6, 0]);
        let one_res: &[c_char; 10] = &black_box([48, 49, 50, 5, 10, 15, 1, 2, 3, 0]);

        unsafe {
            strncat(dst.as_mut_ptr(), one.as_ptr(), 3);
            assert_eq!(dst, one_res);
        }
    }
    */

    #[test]
    fn test_strcmp() {
        let one: &[c_char; 7] = &black_box([48, 49, 50, 5, 10, 15, 0]);
        let two: &[c_char; 7] = &black_box([48, 49, 50, 6, 10, 15, 0]);
        let three: &[c_char; 7] = &black_box([48, 49, 50, 0, 10, 15, 0]);

        unsafe {
            assert_eq!(strcmp(one.as_ptr(), one.as_ptr()), 0);

            assert_eq!(strcmp(one.as_ptr(), two.as_ptr()), -1);
            assert_eq!(strcmp(one.as_ptr(), three.as_ptr()), 5);

            assert_eq!(strcmp(two.as_ptr(), one.as_ptr()), 1);
            assert_eq!(strcmp(two.as_ptr(), three.as_ptr()), 6);

            assert_eq!(strcmp(three.as_ptr(), one.as_ptr()), -5);
            assert_eq!(strcmp(three.as_ptr(), two.as_ptr()), -6);
        }
    }

    #[test]
    fn test_strncmp() {
        let one: &[c_char; 7] = &black_box([48, 49, 50, 5, 10, 15, 0]);
        let two: &[c_char; 7] = &black_box([48, 49, 50, 6, 10, 15, 0]);
        let three: &[c_char; 7] = &black_box([48, 49, 50, 0, 10, 15, 0]);

        unsafe {
            assert_eq!(strncmp(one.as_ptr(), one.as_ptr(), 6), 0);
            assert_eq!(strncmp(one.as_ptr(), one.as_ptr(), 0), 0);
            assert_eq!(strncmp(one.as_ptr(), one.as_ptr(), 3), 0);
            assert_eq!(strncmp(one.as_ptr(), one.as_ptr(), 20), 0);

            assert_eq!(strncmp(one.as_ptr(), two.as_ptr(), 6), -1);
            assert_eq!(strncmp(one.as_ptr(), three.as_ptr(), 6), 5);

            assert_eq!(strncmp(two.as_ptr(), one.as_ptr(), 6), 1);
            assert_eq!(strncmp(two.as_ptr(), three.as_ptr(), 6), 6);

            assert_eq!(strncmp(three.as_ptr(), one.as_ptr(), 6), -5);
            assert_eq!(strncmp(three.as_ptr(), two.as_ptr(), 6), -6);
        }
    }

    #[test]
    fn test_strlen() {
        let cs: &[c_char; 4] = &black_box([48, 49, 50, 0]);
        unsafe {
            assert_eq!(strlen(cs.as_ptr()), 3);
        }

        let weird: &[c_char; 8] = &black_box([48, 49, 50, 0, 5, 10, 15, 0]);
        unsafe {
            assert_eq!(strlen(weird.as_ptr()), 3);
        }

        let null: &[c_char; 1] = &black_box([0]);
        unsafe {
            assert_eq!(strlen(null.as_ptr()), 0);
        }

        let long: &[c_char; 1024] = &black_box([32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 0]);
        unsafe {
            assert_eq!(strlen(long.as_ptr()), 1023);
        }
    }
}
