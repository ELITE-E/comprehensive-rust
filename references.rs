// Shared References

//A reference provides a way to access another value without taking ownership of the value,
//and is also called ”borrowing”. Shared references are read-only, and the referenced data
//cannot change.

pub fn shared(){
    let a = 'A';
    let b = 'B';

    let mut r : &char = &a;
    dbg!(r);

    r = &b;
    dbg!(r);
}

//Important notes exist on pg 55 .Check em out 

//9.2 Exclusive references

//also known as mutable references, allow changing the value they refer
//to. They have type &mut T.

pub fn exclusive(){
    let mut point = (1,2);
    let x_coord = &mut point.0;
    *x_coord = 20 ;
    println!("point:{point:?}");
}
//Notes at the end of page 55

//9.3 Slices
//• Slices borrow data from the sliced type.
pub fn slice(){
    let a:[i32;6]=[0,3,1,5,7,8];
    println!("a:{a:?}");

    let s:&[i32]= &a[2..4];
    println!("s:{s:?}");
}