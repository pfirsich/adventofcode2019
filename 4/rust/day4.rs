fn num_less(a: &[i32], b: &[i32]) -> bool {
    assert!(a.len() == b.len());
    for i in 0..a.len() {
        if a[i] < b[i] {
            return true;
        } else if a[i] > b[i] {
            return false;
        }
        // If equal (else-case), go to next digit
    }
    return false;
}

fn is_mono(a: &[i32]) -> bool {
    for i in 1..a.len()-1 {
        if a[i] > a[i+1] {
            return false;
        }
    }
    return true;
}

fn has_repeat(a: &[i32]) -> bool {
    for i in 0..a.len()-1 {
        if a[i] == a[i+1] {
            return true;
        }
    }
    return false;
}

fn repeat_length(a: &[i32], start: usize) -> usize {
    let mut i = start + 1;
    while i < a.len() && a[i] == a[start] {
        i += 1;
    }
    return i - start;
}

fn has_special_repeat(a: &[i32]) -> bool {
    let mut i = 0;
    while i < a.len()-1 {
        let rep_len = repeat_length(&a, i);
        println!("Num = {:?}, index = {}, digit = {}, repeat = {}", a, i, a[i], rep_len);
        if rep_len == 2 {
            return true;
        }
        i += rep_len;
    }
    return false;
}

fn is_valid(a: &[i32]) -> bool {
    return is_mono(&a) && has_special_repeat(&a);
    //return is_mono(&a) && has_repeat(&a);
}

fn get_next_mono(a: &[i32; 6]) -> [i32; 6] {
    let mut mono = a.clone();
    for i in 1..a.len()-1 {
        if a[i] > a[i+1] {
            for j in i+1..a.len() {
                mono[j] = a[i];
            }
            break;
        }
    }
    return mono;
}

fn increase_digit(num: &mut[i32], digit: usize) -> i32 {
    if digit == 0 && num[digit] == 9 {
        return 9; // maxed out, don't do anything
    }
    num[digit] += 1;
    if num[digit] > 9 {
        let mut num_ref: &mut[i32] = num; // I need this variable to silence the borrow checker
        num[digit] = increase_digit(&mut num_ref, digit - 1);
    }
    return num[digit];
}

fn main() {
    let mut cur_number = get_next_mono(&[1, 3, 0, 2, 5, 4]);
    let digit_num = cur_number.len();
    println!("First mono: {:?}", cur_number);
    let max_number = [6, 7, 8, 2, 7, 5];
    let mut counter = 0;
    while num_less(&cur_number, &max_number) {
        println!("{:?} - {}", cur_number, is_valid(&cur_number));
        if is_valid(&cur_number) {
            counter += 1;
        }
        increase_digit(&mut cur_number, digit_num - 1);
        assert!(is_mono(&cur_number));
    }
    println!("Count: {}", counter);
}
