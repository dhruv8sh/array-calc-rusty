
/*todo:
 *  Final Precedence Order by default:
 *  0. Numbers->0
 *  1. Function end->1
 *  2. Comma->2
 *  3. Arthemetic->10+
 *  4. Logical->40+
 *  5. Functions->60+
 *
 */
use crate::parsers::parse_mathematical;

mod op_solvers{
    pub fn type_map(op:u32) ->u8{
        match op{
            0..=3=>op as u8,
            10..=40=>3,
            41..=60=>4,
            _=>4
        }
    }
    pub fn binary_op_solve(a:f32,b:f32,op:&u32)->f32{
        match op{
            11=>a-b,         // -(Minus)
            12=>a+b,         // +(Add)
            14=>a/b,         // /(Divide)
            15=>a%b,         // %(Remainder)
            16=>a.powf(b),  // **(Power)
            // 7=>*a<<b as u32,        // <<(Shift left)
            // 8=>*a>>b as u32,        // >>(Shift right)
            // 9=>a&b,         // &(Bitwise AND)
            // 10=>a|b,        // |(Bitwise OR)
            // 11=>a^b,        // ^(Bitwise XOR)
            _=>a*b          // *(Multiply)      ---Also for precedence=13
        }
    }
    pub fn logical_op_solve(a:bool,b:bool,op:u32) ->bool{
        match op{
            21=> a<b,
            22=> a>b,
            23=> a<=b,
            24=> a>=b,
            25=> a==b,
            26=> a!=b,
            27=> a&&b,
            28=> a||b,
            _=> false
        }
    }
    pub fn function_op_solve(arr:&Vec<f32>, op:u32) ->f32{
        match op{
            61=>function_operators::max_num(arr),
            62=>function_operators::min_num(arr),
            63=>if function_operators::print_all(arr){
                1.0}
                else{0.0},
            _=>function_operators::average(arr)
        }
    }
    mod function_operators {
        pub fn average(arr: &Vec<f32>) -> f32 {
            let mut res: f32 = 0.0;
            for x in arr {
                res += x;
            }
            res / arr.len() as f32
        }

        pub fn max_num(arr: &Vec<f32>) -> f32 {
            let mut res = f32::MIN;
            for x in arr {
                if res < *x {
                    res = *x;
                }
            }
            println!("res: {}",res);
            res
        }

        pub fn min_num(arr: &Vec<f32>) -> f32 {
            let mut res = f32::MAX;
            for x in arr {
                if res > *x {
                    res = *x;
                }
            }
            res
        }

        pub fn print_all(arr: &Vec<f32>) ->bool{
            if arr.len() == 0{
                return false;
            }
            for x in arr {
                print!("{}\t",x);
            }
            println!();
            true
        }
    }
}

mod parsers{
    use crate::op_solvers;
    // pub fn parse_mathematical(nums: Vec<f32>, ops:Vec<u32>) ->Vec<f32>{
    //     let mut num_stack: Vec<f32> = Vec::new();
    //     let mut op_stack: Vec<u32> = Vec::new();
    //     let mut temp_vec: Vec<f32>;
    //     let mut temp1: f32;
    //     let mut temp2: f32;
    //     for (x,op) in ops.iter().enumerate(){
    //         println!("{}: {}",x,op);
    //         match op{
    //             0=>num_stack.push(nums[x]),
    //             1=> {
    //                 temp_vec=Vec::new();
    //                 while op_stack.pop()==Some(2){
    //                     temp_vec.push(num_stack.pop().unwrap());
    //                 }
    //                 num_stack.push(
    //                     op_solvers::function_op_solve(
    //                         &temp_vec,
    //                         *op
    //                     )
    //                 );
    //             }
    //             2=>{
    //                 while op_stack[op_stack.len()-1] != 2{
    //                     temp1=num_stack.pop().unwrap();
    //                     temp2=num_stack.pop().unwrap();
    //                     num_stack.push(
    //                         op_solvers::binary_op_solve(
    //                             temp1,
    //                             temp2,
    //                             &op_stack.pop().unwrap())
    //                     );
    //                 }
    //             }
    //             10..=40=> {
    //                 while op_stack[op_stack.len()-1] >= *op {
    //                     temp1=num_stack.pop().unwrap();
    //                     temp2=num_stack.pop().unwrap();
    //                     num_stack.push(
    //                         op_solvers::binary_op_solve(
    //                             temp1,
    //                             temp2,
    //                             &op)
    //                     );
    //                 };
    //                 op_stack.push(ops[x]);
    //             }
    //             60..=80=> op_stack.push(*op),
    //             _=>println!("Not yet implemented!")
    //         };
    //     }
    //     for x in nums{
    //         print!("{}\t",x);
    //     }
    //     println!();
    //     for x in ops{
    //         print!("{}\t",x);
    //     }
    //     while op_stack.len() > 0{
    //         temp1=num_stack.pop().unwrap();
    //         temp2=num_stack.pop().unwrap();
    //         num_stack.push(
    //             op_solvers::binary_op_solve(
    //                 temp1,
    //                 temp2,
    //                 &op_stack.pop().unwrap())
    //         );
    //     }
    //     num_stack
    // }
    pub fn parse_mathematical(mut nums: Vec<f32>, mut ops: Vec<u32>) ->Vec<f32>{
        if ops.len() == 0{
            return nums;
        }
        let mut counter:usize=1;
        while ops.len() > 0 {
            match ops[counter]{
                1=> {
                    nums.remove(counter);
                    let mut arr: Vec<f32>=Vec::new();
                    while ops[counter] <= 60{
                        ops.remove(counter);
                        counter-=1;
                        arr.push(nums.remove(counter));
                    }
                    ops.remove(counter);
                    println!("temp: {:?}",arr);
                    nums.insert(
                        counter,
                        op_solvers::function_op_solve(&arr,ops[counter-1]));
                    ops.insert(
                        counter,
                        13
                    );
                },
                2=> counter+=1,
                11..=40=> {
                    if counter == 0{
                        match ops.remove(counter){
                            11=>nums[0]=-nums[0],
                            _=>{}
                        };
                    }
                    else {
                        while counter > 0 && ops[counter - 1] < ops[counter] {
                            let temp1: f32 = nums.remove(counter);
                            let temp2: f32 = nums.remove(counter-1);
                            nums.insert(
                                counter-1,
                                op_solvers::binary_op_solve(
                                    temp1,
                                    temp2,
                                    &ops.remove(counter))
                            );
                            counter -= 1;
                        }
                    }
                }
                41..=60=> println!("Not in logical mode!"),
                61..=80=> counter+=1,
                _=>println!("Not implemented yet!")
            };
            println!("{:?}",nums);
            println!("{:?}  {}",ops, counter);
        }
        nums
    }
}

fn main() {
    let arr1:Vec<f32>=vec![1.0,1.0,2.0,3.0,4.0,0.0];
    let arr2:Vec<u32>=vec![ 12, 61,  2,  2,  2, 1];
    let res = parse_mathematical(arr1,arr2);
    println!("result: {:?}",res);
}
