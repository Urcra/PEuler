fn main() {
    let perm = [0,1,2,3,4,5,6,7,8,9];
    let mut remain = 1000000-1;

    let mut numbers: Vec<usize> = Vec::new();

    for i in 0..perm.len() {
        numbers.push(i);
    }

    for i in 1..perm.len() {
        let j = remain / factorial(perm.len() - i);
        remain = remain % factorial(perm.len()- i);

        print!("{:?}", numbers[j]);

        numbers.remove(j);

        if remain == 0 {
            break;
        }
    }

    for i in 0..numbers.len() {
        print!("{:?}", numbers[i]);
    }




}

fn factorial(num: usize) -> usize {
    (1..num+1).fold(1, |p, n| p*n)
}