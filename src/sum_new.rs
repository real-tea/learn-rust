fn sum(values : &[i32])-> i32{
    let mut res = 0;
    for i in 0..values.len(){
        res+=values[i];
    }
    res
}

fn plus_one(x : &i32)-> i32{
    x+1 //don't use a semicolon
}

fn main(){
    let arr = [1,2,3,4,5];
    let res = sum(&arr);
    println!("sum is : {}",res);
    println!("after plus one function");
    let res2 = plus_one(&res);
    println!("{}",res2);
}