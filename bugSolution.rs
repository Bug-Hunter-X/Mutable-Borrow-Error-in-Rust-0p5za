fn main() {
    let mut x = 5;
    { //This is a fix with block scope 
        let y = &mut x;
        *y += 1;
    }
    { //Another fix with block scope
        let z = &mut x;
        *z += 1;
    }
    println!("x = {}", x);
}