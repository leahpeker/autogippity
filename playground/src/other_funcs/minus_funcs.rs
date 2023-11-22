pub fn sub_ten(num: u32) -> u32 {
    num -10
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sub_ten_test(){
        let x: u32 = 100;
        let y: u32 = sub_ten(x);

        println!("x and y are from test: {} {}", x,  y);
        assert_eq!(y, 90);
    }
}