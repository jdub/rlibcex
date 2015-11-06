pub use super::libc::*;

// http://pubs.opengroup.org/onlinepubs/9699919799/basedefs/ctype.h.html
// http://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap07.html#tag_07_03_01

#[no_mangle]
pub unsafe extern fn isalnum(c: c_int) -> c_int {
    (isalpha(c) == 1 || isdigit(c) == 1) as c_int
}

#[no_mangle]
pub unsafe extern fn isalpha(c: c_int) -> c_int {
    (isupper(c) == 1 || islower(c) == 1) as c_int
}

#[no_mangle]
pub unsafe extern fn isascii(c: c_int) -> c_int {
    (c >= 0 && c <= 127) as c_int
}

#[no_mangle]
pub unsafe extern fn isblank(c: c_int) -> c_int {
    (c == 9 || c == 32) as c_int
}

#[no_mangle]
pub unsafe extern fn iscntrl(c: c_int) -> c_int {
    ((c >= 0 && c <= 31) || c == 127) as c_int
}

#[no_mangle]
pub unsafe extern fn isdigit(c: c_int) -> c_int {
    (c >= 48 && c <= 57) as c_int
}

#[no_mangle]
pub unsafe extern fn isgraph(c: c_int) -> c_int {
    (isalpha(c) == 1 || isdigit(c) == 1 || ispunct(c) == 1) as c_int
}

#[no_mangle]
pub unsafe extern fn islower(c: c_int) -> c_int {
    (c >= 97 && c <= 122) as c_int
}

#[no_mangle]
pub unsafe extern fn isprint(c: c_int) -> c_int {
    ((isgraph(c) == 1 && iscntrl(c) != 1) || c == 32) as c_int
}

#[no_mangle]
pub unsafe extern fn ispunct(c: c_int) -> c_int {
    ((c >= 33 && c <= 47) || (c >= 58 && c <= 64) || (c >= 91 && c <= 96) || (c >= 123 && c <= 126)) as c_int
}

#[no_mangle]
pub unsafe extern fn isspace(c: c_int) -> c_int {
    ((c >= 9 && c <= 13) || c == 32) as c_int
}

#[no_mangle]
pub unsafe extern fn isupper(c: c_int) -> c_int {
    (c >= 65 && c <= 90) as c_int
}

#[no_mangle]
pub unsafe extern fn isxdigit(c: c_int) -> c_int {
    (isdigit(c) == 1 || (c >= 65 && c <= 70) || (c >= 97 && c <= 102)) as c_int
}

#[no_mangle]
pub unsafe extern fn toascii(c: c_char) -> c_char {
    c & 0x7f
}

#[no_mangle]
pub unsafe extern fn tolower(c: c_char) -> c_char {
    if isupper(c as c_int) == 1 {
        c + 32
    } else {
        c
    }
}

#[no_mangle]
pub unsafe extern fn toupper(c: c_char) -> c_char {
    if islower(c as c_int) == 1 {
        c - 32
    } else {
        c
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use test::black_box;

    #[test]
    fn test_isalnum() {
        unsafe {
            assert!(isalnum(black_box('A') as c_int) > 0 as c_int, "A");
            assert!(isalnum('a' as c_int) > 0 as c_int, "a");
            assert!(isalnum(' ' as c_int) == 0 as c_int, "<space>");
            assert!(isalnum('0' as c_int) > 0 as c_int, "0");
            assert!(isalnum('\t' as c_int) == 0 as c_int, "<tab>");
            assert!(isalnum('-' as c_int) == 0 as c_int, "-");
        }
    }

    #[test]
    fn test_isalpha() {
        unsafe {
            assert!(isalpha('A' as c_int) > 0 as c_int, "A");
            assert!(isalpha('a' as c_int) > 0 as c_int, "a");
            assert!(isalpha(' ' as c_int) == 0 as c_int, "<space>");
            assert!(isalpha('0' as c_int) == 0 as c_int, "0");
            assert!(isalpha('\t' as c_int) == 0 as c_int, "<tab>");
            assert!(isalpha('-' as c_int) == 0 as c_int, "-");
        }
    }

    #[test]
    fn test_isascii() {
        unsafe {
            assert!(isascii('A' as c_int) > 0 as c_int, "A");
            assert!(isascii('a' as c_int) > 0 as c_int, "a");
            assert!(isascii(' ' as c_int) > 0 as c_int, "<space>");
            assert!(isascii('0' as c_int) > 0 as c_int, "0");
            assert!(isascii('\t' as c_int) > 0 as c_int, "<tab>");
            assert!(isascii('-' as c_int) > 0 as c_int, "-");
            assert!(isascii(153 as c_int) == 0 as c_int, "™");
        }
    }

    #[test]
    fn test_isblank() {
        unsafe {
            assert!(isblank('A' as c_int) == 0 as c_int, "A");
            assert!(isblank('a' as c_int) == 0 as c_int, "a");
            assert!(isblank(' ' as c_int) > 0 as c_int, "<space>");
            assert!(isblank('0' as c_int) == 0 as c_int, "0");
            assert!(isblank('\t' as c_int) > 0 as c_int, "<tab>");
            assert!(isblank('-' as c_int) == 0 as c_int, "-");
        }
    }

    #[test]
    fn test_iscntrl() {
        unsafe {
            assert!(iscntrl('A' as c_int) == 0 as c_int, "A");
            assert!(iscntrl('a' as c_int) == 0 as c_int, "a");
            assert!(iscntrl(' ' as c_int) == 0 as c_int, "<space>");
            assert!(iscntrl('0' as c_int) == 0 as c_int, "0");
            assert!(iscntrl('\t' as c_int) > 0 as c_int, "<tab>");
            assert!(iscntrl('-' as c_int) == 0 as c_int, "-");
        }
    }

    #[test]
    fn test_isdigit() {
        unsafe {
            assert!(isdigit('A' as c_int) == 0 as c_int, "A");
            assert!(isdigit('a' as c_int) == 0 as c_int, "a");
            assert!(isdigit(' ' as c_int) == 0 as c_int, "<space>");
            assert!(isdigit('0' as c_int) > 0 as c_int, "0");
            assert!(isdigit('\t' as c_int) == 0 as c_int, "<tab>");
            assert!(isdigit('-' as c_int) == 0 as c_int, "-");
        }
    }

    #[test]
    fn test_isgraph() {
        unsafe {
            assert!(isgraph('A' as c_int) > 0 as c_int, "A");
            assert!(isgraph('a' as c_int) > 0 as c_int, "a");
            assert!(isgraph(' ' as c_int) == 0 as c_int, "<space>");
            assert!(isgraph('0' as c_int) > 0 as c_int, "0");
            assert!(isgraph('\t' as c_int) == 0 as c_int, "<tab>");
            assert!(isgraph('-' as c_int) > 0 as c_int, "-");
        }
    }

    #[test]
    fn test_islower() {
        unsafe {
            assert!(islower('A' as c_int) == 0 as c_int, "A");
            assert!(islower('a' as c_int) > 0 as c_int, "a");
            assert!(islower(' ' as c_int) == 0 as c_int, "<space>");
            assert!(islower('0' as c_int) == 0 as c_int, "0");
            assert!(islower('\t' as c_int) == 0 as c_int, "<tab>");
            assert!(islower('-' as c_int) == 0 as c_int, "-");
        }
    }

    #[test]
    fn test_isprint() {
        unsafe {
            assert!(isprint('A' as c_int) > 0 as c_int, "A");
            assert!(isprint('a' as c_int) > 0 as c_int, "a");
            assert!(isprint(' ' as c_int) > 0 as c_int, "<space>");
            assert!(isprint('0' as c_int) > 0 as c_int, "0");
            assert!(isprint('\t' as c_int) == 0 as c_int, "<tab>");
            assert!(isprint('-' as c_int) > 0 as c_int, "-");
        }
    }

    #[test]
    fn test_ispunct() {
        unsafe {
            assert!(ispunct('A' as c_int) == 0 as c_int, "A");
            assert!(ispunct('a' as c_int) == 0 as c_int, "a");
            assert!(ispunct(' ' as c_int) == 0 as c_int, "<space>");
            assert!(ispunct('0' as c_int) == 0 as c_int, "0");
            assert!(ispunct('\t' as c_int) == 0 as c_int, "<tab>");
            assert!(ispunct('-' as c_int) > 0 as c_int, "-");
        }
    }

    #[test]
    fn test_isspace() {
        unsafe {
            assert!(isspace('A' as c_int) == 0 as c_int, "A");
            assert!(isspace('a' as c_int) == 0 as c_int, "a");
            assert!(isspace(' ' as c_int) > 0 as c_int, "<space>");
            assert!(isspace('0' as c_int) == 0 as c_int, "0");
            assert!(isspace('\t' as c_int) > 0 as c_int, "<tab>");
            assert!(isspace('-' as c_int) == 0 as c_int, "-");
        }
    }

    #[test]
    fn test_isupper() {
        unsafe {
            assert!(isupper('A' as c_int) > 0 as c_int, "A");
            assert!(isupper('a' as c_int) == 0 as c_int, "a");
            assert!(isupper(' ' as c_int) == 0 as c_int, "<space>");
            assert!(isupper('0' as c_int) == 0 as c_int, "0");
            assert!(isupper('\t' as c_int) == 0 as c_int, "<tab>");
            assert!(isupper('-' as c_int) == 0 as c_int, "-");
        }
    }

    #[test]
    fn test_isxdigit() {
        unsafe {
            assert!(isxdigit('A' as c_int) > 0 as c_int, "A");
            assert!(isxdigit('a' as c_int) > 0 as c_int, "a");
            assert!(isxdigit(' ' as c_int) == 0 as c_int, "<space>");
            assert!(isxdigit('0' as c_int) > 0 as c_int, "0");
            assert!(isxdigit('\t' as c_int) == 0 as c_int, "<tab>");
            assert!(isxdigit('-' as c_int) == 0 as c_int, "-");
        }
    }

    #[test]
    #[allow(overflowing_literals)]
    fn test_toascii() {
        unsafe {
            assert!(toascii('A' as c_char) == 'A' as c_char, "A to A");
            assert!(toascii('a' as c_char) == 'a' as c_char, "a to a");
            assert!(toascii(' ' as c_char) == ' ' as c_char, "<space> to <space>");
            assert!(toascii('0' as c_char) == '0' as c_char, "0 to 0");
            assert!(toascii('\t' as c_char) == '\t' as c_char, "<tab> to <tab>");
            assert!(toascii('-' as c_char) == '-' as c_char, "- to -");
            assert_eq!(toascii(153 as c_char), 25 as c_char);
        }
    }

    #[test]
    fn test_tolower() {
        unsafe {
            assert!(tolower('A' as c_char) == 'a' as c_char, "A to a");
            assert!(tolower('a' as c_char) == 'a' as c_char, "a to a");
            assert!(tolower(' ' as c_char) == ' ' as c_char, "<space> to <space>");
            assert!(tolower('0' as c_char) == '0' as c_char, "0 to 0");
            assert!(tolower('\t' as c_char) == '\t' as c_char, "<tab> to <tab>");
            assert!(tolower('-' as c_char) == '-' as c_char, "- to -");
        }
    }

    #[test]
    fn test_toupper() {
        unsafe {
            assert!(toupper('A' as c_char) == 'A' as c_char, "A to A");
            assert!(toupper('a' as c_char) == 'A' as c_char, "a to A");
            assert!(toupper(' ' as c_char) == ' ' as c_char, "<space> to <space>");
            assert!(toupper('0' as c_char) == '0' as c_char, "0 to 0");
            assert!(toupper('\t' as c_char) == '\t' as c_char, "<tab> to <tab>");
            assert!(toupper('-' as c_char) == '-' as c_char, "- to -");
        }
    }
}
