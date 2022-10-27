/*
Actual Equation in the form of vectors.
Thinking about creating an Operator struct to use in Equation,
would give an Equation to bear expressions like log(log(log(2))) without taking extra space.
But that would also make it harder for the implementer to manage.
 */
mod mathematic_functions;

pub struct Equation<T>{
    pub  nums: Vec<T>,
    pub ops: Vec<u32>,
    pub multiply_operator: u32,
    pub bracket_end_operator: u32,
    pub comma: u32
}

impl<T> Equation<T>
where T:PrecedenceControl
{
    fn parse_mathematical(&self) ->Vec<T>{
        if self.ops.len() == 0{
            return self.nums;
        }
        let mut arr: Vec<T> = Vec::new();
        let mut counter:usize = 0;
        let mut loop_count:u32 = 0;
        while self.ops.len() > 0 || loop_count == 100{
            if self.ops[counter] == self.bracket_end_operator {
                arr = Vec::new();
                while PrecedenceControl::is_function(&self.ops[counter]) {
                    self.ops.remove(counter);
                    counter -= 1;
                    arr.insert(
                        0,
                        self.nums.remove(counter));
                }
                arr = PrecedenceControl::op_solve( &arr,self.ops.remove(counter));


                /*
                Insert multiply operator on left side if arr.len() == 1
                Else, just Begin and End Brackets.
                */
                if arr.len() > 1 {
                    self.ops.insert(
                        counter,
                        3
                    );
                } else {
                    self.ops.insert(
                        counter,
                        self.comma
                    );
                }

                for x in arr {
                    self.nums.insert(counter, x);
                    counter += 1;
                    self.ops.insert(counter, self.comma);
                }

                self.ops.remove(counter);
                if arr.len() > 1 {
                    self.ops.insert(
                        counter,
                        1
                    );
                }
            }
            else if self.ops[counter] == 2 {
                counter+=1;
            }
            else if PrecedenceControl::is_function(&self.ops[counter]) {
                arr=Vec::new();
                arr.insert(0,self.nums.remove(counter));
                arr.insert(0,self.nums.remove(counter-1));
                while counter > 0 && self.ops[counter - 1] < self.ops[counter] {
                    self.nums.insert(
                        counter-1,
                        PrecedenceControl::op_solve(
                            &arr,
                            self.ops.remove(counter))
                    );
                    counter -= 1;
                }
            }
            else if PrecedenceControl::is_function(&self.ops[counter]){
                counter+=1;
            }
            else {
                println!("Not implemented yet!");
            }
            loop_count+=1;
        }
        if loop_count == 100 {
            println!("Reached max iteration value!");
        }
        self.nums
    }
}

/*
Implement the trait PrecedenceControl for struct Equation<T> to pass to the parser.
PrecedenceControl contains two functions all of which are necessary to implement for now.
*/
pub trait PrecedenceControl{
    /*
    op_solve is the actual function for your control over precedence.
    takes input using a type vector and an operator from the parser.

    Map your current operator to the operation, preferably by a switch statement and return
    an answer bearing vector.

    IMPORTANT: Higher operator value = Higher precedence
     */
    fn op_solve<T>(nums:&Vec<T>, op:u32) -> Vec<T>
    {
        let ans:Vec<T> = Vec::new();
        ans.append(
            match op{
                // 11=>nums[0]-nums[1],         // -(Minus)
                // 12=>nums[0]+nums[1],         // +(Add)
                // 14=>nums[0]/nums[1],         // /(Divide)
                // 13=>nums[0]*nums[1],         // *(Multiply)
                // 15=>nums[0]%nums[1],         // %(Remainder)
                // 16=>nums[0].powf(nums[1]),   // **(Power)
                61=>mathematic_functions::max_num(nums),
                62=>mathematic_functions::min_num(nums),
                63=>if mathematic_functions::print_all(nums){
                    1.0}
                else{0.0},
                _=>mathematic_functions::average(nums)
            }
        );
        ans
    }


    /*
    is_function returns true, if the current operator is a 'Function Type'
    else returns false

    Function Type Operator:
    This type of operator has to fulfill the following conditions:
        1. Numerical values should be enclosed within this operator and an `BRACKET_END` operator.
        2. Cannot have zero numerical values... For now!
        3.
     */
    fn is_function(op:&u32)->bool{
        op <= 60
    }
    // fn set_const_precedences(multiply_precedence: u32, bracket_end_precedence: u32);

}