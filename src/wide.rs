use libc::wchar_t;

macro_rules! wide {
    ($($ch:ident)*) => {
        [$(stringify!($ch).as_bytes()[0] as ::libc::wchar_t,)* 0]
    }
}

pub fn copy(dest: &mut [wchar_t], src: &str) {
    if dest.is_empty() { return }
    let mut index = 0;
    for ch in src.encode_utf16() {
        if index == dest.len() - 1 { break }
        dest[index] = ch;
        index += 1;
    }
    dest[index] = 0;
}

pub fn read(src: &[wchar_t]) -> String {
    let zero = src.iter().position(|&c| c == 0).unwrap_or(src.len());
    String::from_utf16_lossy(&src[..zero])
}

#[test]
fn test_macro() {
    let wide = wide!(M u m b l e L i n k);
    for (i, b) in "MumbleLink".bytes().enumerate() {
        assert_eq!(b as wchar_t, wide[i]);
    }
}

#[test]
fn test_copy() {
    let mut wide = [1; 32];
    copy(&mut wide, "FooBar");
    assert_eq!(&wide[..7], wide!(F o o B a r));

    let mut wide = [1; 3];
    copy(&mut wide, "ABC");
    assert_eq!(&wide[..], wide!(A B));
}
