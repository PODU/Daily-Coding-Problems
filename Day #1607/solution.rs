// Bijective char mapping s1->s2: lengths equal + consistent forward map + injective (no two s1 chars share an s2 char).
// Time O(n), Space O(1) (fixed 256-size maps).

fn bijective_map(s1: &str, s2: &str) -> bool {
    let a: Vec<u8> = s1.bytes().collect();
    let b: Vec<u8> = s2.bytes().collect();
    if a.len() != b.len() {
        return false;
    }
    let mut fwd = [-1i32; 256];
    let mut rev = [-1i32; 256];
    for i in 0..a.len() {
        let (x, y) = (a[i] as usize, b[i] as usize);
        if fwd[x] == -1 && rev[y] == -1 {
            fwd[x] = y as i32;
            rev[y] = x as i32;
        } else if fwd[x] != y as i32 || rev[y] != x as i32 {
            return false;
        }
    }
    true
}

fn main() {
    println!("{}", bijective_map("abc", "bcd"));
    println!("{}", bijective_map("foo", "bar"));
}
