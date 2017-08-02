use rand::OsRng;
use rand::Rng;

pub fn roll_four_dice(rng : &mut OsRng) -> [u8; 4] {
    return [(rng.next_u32() % 6) as u8, (rng.next_u32() % 6) as u8, (rng.next_u32() % 6) as u8, (rng.next_u32() % 6) as u8];
}

pub fn from_array_to_index(arr: [u8;4]) -> usize {
    let a = arr[0] as usize;
    let b = arr[1] as usize;
    let c = arr[2] as usize;
    let d = arr[3] as usize;
    return a * pow(6, 3) + b * pow(6, 2) + c * pow(6, 1) + d;
}

pub fn pow(x: usize, y: usize) -> usize {
    let mut r = 1u32;
    for _ in 0..y {
        //println!("building r = {}", r);
        r = r * (x as u32);
    }
    //println!("x={} ^ y={} == r={}", x, y, r);
    return r as usize;
}


#[cfg(test)]
mod test {

    use dice;


    #[test]
    fn index_from_array_1_test() {
        let x = dice::from_array_to_index([1, 1, 1, 1]);
        assert_eq!(x, 259);
    }

    #[test]
    fn index_from_array_2_test() {
        let x = dice::from_array_to_index([1, 2, 3, 4]);
        assert_eq!(x, 310);
    }



    #[test]
    fn index_from_array_3_test() {
        let x = dice::from_array_to_index([5, 5, 5, 5]);
        assert_eq!(x, 1295);
    }


    #[test]
    fn pow_test1() {
        let x = dice::pow(2, 9);
        assert_eq!(x, 512);
    }

    #[test]
    fn pow_test2() {
        let x = dice::pow(6, 3);
        assert_eq!(x, 216);
    }

    #[test]
    fn pow_test3() {
        let x = dice::pow(6, 2);
        assert_eq!(x, 36);
    }

    #[test]
    fn pow_test4() {
        let x = dice::pow(6, 1);
        assert_eq!(x, 6);
    }

    #[test]
    fn pow_test5() {
        let x = dice::pow(6, 0);
        assert_eq!(x, 1);
    }

    /*#[test]
    #[should_panic]
    fn failing_test() {
        assert!(1i32 == 2i32);
    }*/
}