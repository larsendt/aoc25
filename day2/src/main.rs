use std::fs;

// fn test(mut s: &str, len: usize) -> bool {
//     if s.len() % len != 0 {
//         return false;
//     }

//     let check = &s[..len];
//     while s.len() > 0 {
//         s = &s[len..];
//         dbg!(check, s);
//         if &s[..len.min(s.len())] != check {
//             return false;
//         }
//     }

//     check == s
// }

// fn test_all(s: &str) -> bool {
//     if s.len() == 1 {
//         return true;
//     }

//     let mut ok = false;
//     for l in 1..(s.len() / 2) + 1 {
//         ok |= test(s, l);
//     }

//     ok
// }

fn test_part1(i: usize) -> bool {
    let div = 10usize.pow((i.ilog10() + 1) / 2) as usize;
    let l = i / div;
    let r = i % (10 * l);
    l == r && div != 1
}

fn main() {
    let f = fs::read_to_string("input.txt").unwrap().replace('\n', "");
    let ranges = f.split(',');
    let mut sum = 0;
    for range in ranges {
        let mut sp_range = range.split('-');
        let lo: usize = sp_range.next().unwrap().parse().unwrap();
        let hi: usize = sp_range.next().unwrap().parse().unwrap();
        for i in lo..hi + 1 {
            let invalid = test_part1(i);
            if invalid {
                sum += i;
            }
        }
    }

    dbg!(sum);
}
