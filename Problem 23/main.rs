fn main() {
    // Find abundant numbers

    const LIMIT: u32 = 28123;

    let mut abnums: Vec<u32> = Vec::new();

    for i in 0..LIMIT {
        if sumofdiv(i) > i {
            abnums.push(i);
        }
    }

    // Find all number that can be written as the sum of two abundent numbers
    let mut numbers = [false; (LIMIT as usize) +1];
    for i in 0..abnums.len() {
        for j in i..abnums.len() {
            if abnums[i as usize] + abnums[j as usize] <= LIMIT {
                numbers[(abnums[i as usize] + abnums[j as usize]) as usize] = true;
            } else {
                break;
            }
        }
    }

    // sum the numbers that can't be written as two abundent numbers
    let mut sum = 0;
    for i in 1..LIMIT+1 {
        if !numbers[i as usize] {
            sum += i;
        }
    }

    println!("{:?}", sum);

}

fn sumofdiv(num: u32) -> u32 {
    let mut sum = 0;
    for i in 1..(num/2 + 1) {
        if num % i == 0 {
            sum += i;
        }
    }
    sum
}