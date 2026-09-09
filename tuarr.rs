//Thismodule contains tupples and arrays in rust (Composite types)

use std::result;

pub fn array(){
    let mut a :[i8 ; 5]=[3,4,5,2,1];

    a[4]=0;
    //TRhis operation wil pavnic at runtime thus rust prevents it from 
    //running unless youre wwithin the limits of lenght 

    println!("a:{a:?}")
}

pub fn get_index()->usize{
    6
}

pub fn tuple(){
    let t: (i8, bool) = (7, true);

    dbg!(t.0);
    dbg!(t.1);
}

//Array iteration
pub fn prime(){
    let primes= [2,5,7,11,13,17,19];

    for prime in primes {
        for i in 2 .. prime{
            assert_ne!(prime % i ,0);
        }
    }
}

//Destructructuring and patterns matching

pub fn check_order(tuple:(i32,i32,i32))->bool{
    let (left,middle,right) = tuple;

    left < middle && middle < right

    
}

//Exercise:Nested arrays 
pub fn transpose(matrix:[[i32;3];3])->[[i32;3];3]{
    //OPen pg 53 for more on this ....
    let mut result = [[0;3];3];

    for i in 0..3 {
      for j in 0..3{
        result[j][i] = matrix[i][j];
      }
    } ;
    result
}