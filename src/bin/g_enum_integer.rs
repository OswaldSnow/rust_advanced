fn main(){
    /*
    枚举和整数
     */

    let e1 = My_Enum::B;
    let i1 = e1 as i32;
    println!("i1 is {i1}");

}

enum My_Enum{
    A = 1,
    B,
    C
}