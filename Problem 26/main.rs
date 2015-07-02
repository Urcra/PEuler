fn main() {
    let mut res = (0, 0);
    for i in 2..1000 {
        if cyclelen(i) > res.1 {
            res = (i, cyclelen(i));
        }
    }
    println!("{:?}", res.0);
}

fn cyclelen(number: usize) -> usize {
    let mut remainders: Vec<usize> = vec![0; number];
    let mut x = 1;
    let mut pos = 0;

    while remainders[x] == 0 && x != 0 {
        remainders[x] = pos;
        x = x * 10;
        x = x % number;
        pos += 1;
    }
    pos - remainders[x]
}