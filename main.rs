mod tuarr;
mod references;
//use tuarr::{
    //array,
    //get_index,
    //tuple,
    //prime,
   //check_order,
    //transpose};

use references::{
    shared,
    exclusive,
    slice
};



fn main() {
    slice();
    exclusive();
    shared();
    //-----------------------------------------------------------------------------------
 //   let matrix = [
//      [101, 102, 103], // <-- the comment makes rustfmt add a newline
//      [201, 202, 203],
//      [301, 302, 303],
// ];

//    println!("Original");
//    for row in matrix {
//     println!("{row:?}");
//    }

//    let transposed = transpose(matrix);
//    println!("\nTrnasposed:");
//    for row in transposed{
//     println!("{row:?}")
//    }

   //let tuple = (1,5,3);

    // println!("{tuple:?}:{}",
    // if check_order(tuple){"ordered"}else{"unordered"});
    // prime();
    //tuple();
     //array();
     
    //  let mut array:[i8;5] =[3,4,6,7,9];
    //  array[get_index()] = 0;
    //  println!("array :{array:#?}")

//      The println! macro asks for the debug implementation with the ? format parameter:
// {} gives the default output, {:?} gives the debug output. Types such as integers and
// strings implement the default output, but arrays only implement the debug output. This
// means that we must use debug output here.
// • Adding #, eg {a:#?}, invokes a ”pretty printing” format, which can be easier to re

    //-------------------------------------------------------------------------------------------------------------
    //Exercise
    //println!("Collatx lenght : {}",calculate_collazz_length(5));

    //Macros
    // let n = 4;
    // println!("the factorial f(n) of {n} is : {}",factorial(n));
    // //functions
    // dbg!(gcd(120, 90));
    //lables
    // let s = [[1,2,3],[5,4,6],[5,6,7]];

    // let mut elements_searched = 0;
    // let target_value = 10 ;
    // 'outer:for i in 0..2 {
    //     for j in 0..2{
    //         elements_searched += 1;
            
    //         if s[i][j] == target_value {
    //             break 'outer;
    //         };

    //         dbg!(elements_searched);
    //     };
    // };

    //continue and break 
    // let mut i = 0;
    // loop {
    //    i += 1;

    //    if i > 5{
    //     break;
    //    }

    //    else if i%2==0 {
    //        continue
    //    }
    //    dbg!(i);
    // }
    //while loops
    // let mut x = 100;

    // while x >10 {
    //   x = x/2;

    // }
    // dbg!(x);
    // // for loops
    // for x in  1 ..5 {
    //     dbg!(x);
    // }

    // for elem in [56,4,5,46,75]{
    //     dbg!(elem);
    // }

    // //loop
    // // The loop statement works like a while true loop. Ideal  for things like servers that
    // //will serve connections forever.

    // let mut i = 10 ;

    // loop {
    //     i += 1;

    //     dbg!(i);

    //     if i >100 {
    //         break;
    //     }
    // }

//     //MATCH Statements
//     let val = 12;
//     match  val {
//         1=>println!("one"),
//         2=>println!("two"),
//         100=>println!("hundred"),
//         _=>println!("Something else")
        
//     }
//     let flag = true;
//     let value = match flag {
//         true =>1,
//         false=>0,
//     };
//     println!("The value of the {flag} is :{value}")
    // If expressions

    // let x = 10;

    // if x == 0 {
    //     println!("X is zero");
    // }
    // else if x >100 {
    //     println!("x is bigger.")
    // }else{
    //     println!("x is smaller.");
    // };

  //Scope and Blocks
// let z = 13;
// let x = {
//     let y= 10;
//     dbg!(y);

//     z-y;

// };
// dbg!(y);This one results in error since y is not in scope here



    //Exercise :Fibonnacci
    // let n = 20 ;
    // let k =8;
    
    // println!("fib({n}) = {}",fib(n));
    // println!("kib({k}) = {}",kib(k));
    // //Type inference
    // let x=10;
    // let y=20;

    // takes_u32(x);
    // takes_i8(y);

    //Arithimetics
    //println!("result: {}",interproduct(120,100, 248))

    //Variable assignment

    // let x:i32 =18;
    // print!("x:{x} ");

    // println!("Hello, world!");

}


// fn kib(k:i32)->i32{
//     if k < 3 {
//         return k;
//     }else{
//         return  kib(k-1) + kib(k-2);
//     }


//fn interproduct(a:i32,b:i32,c:i32)->i32{
//     return  a*b + b*c + c*a;
// }

// fn takes_u32(x:u32){
//     println!("u32:{x}");
// }
// fn takes_i8(y:i8){
//     println!("i8:{y}");
// }
// //Exercise :Fibonnacci
// fn fib(n:u32)->u32{

//     if n < 2 {
//        return n; 
//     }else{
//         return fib(n-1) + fib(n-2);
//     }
// }

// fn gcd(a:u32 ,b:u32)->u32{
//    if b > 0 {
//      gcd(b,a%b)
//    }else{
//     a
//    }
// }


// fn factorial(_n:u32)->u32{
//     let mut product = 1;

//     for i in 1.. {
//         product *= dbg!(i);
//     } 
//     product
// }

// fn fizzbuzz(n:u32)->u32{
//     todo!("In a short while.")
// }

// //exercise
// fn calculate_collazz_length(mut n:i32)->u32{
//     let mut len = 1;

//     while n > 1{
//         n = if n%2 == 0 {return (n/2).try_into().unwrap()}else{3*n+1};
        
//         len += 1;
        
//     };
//     len
// }
