use r_calc::*;
fn main() {
    let arr1:Vec<f32>=vec![1.0,3.0,7.0,3.0,4.0,0.0];
    let arr2:Vec<u32>=vec![ 11, 61,  2,  2,  2, 1];
    println!("Equation: -1*avg(3,7,3,4)");
    let res = parsers::parse_mathematical(
        arr1,
        arr2
    );
    println!("result: {:?}",res);
}
