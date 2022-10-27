mod lib;

use crate::lib::*;
/*
Implement the trait PrecedenceControl for struct Equation<T> to pass to the parser.
Precedence control contains
 */
impl dyn PrecedenceControl {
    fn op_solve(nums: Vec<T>, op: u32) -> Vec<T>{

    }
}

impl dyn Parser{

}


fn main() {
    eqn.nums = vec![1.0,3.0,7.0,3.0,4.0,0.0];
    eqn.ops  = vec![ 11, 61,  2,  2,  2, 1];
    eqn.bracket_end_operator = 1;
    eqn.multiply_operator = 13;
    eqn.comma = 2;
    println!("Equation: -1*avg(3,7,3,4)");
    let res = parsers::parse_mathematical(
        arr1,
        arr2
    );
    println!("result: {:?}",res);
}
