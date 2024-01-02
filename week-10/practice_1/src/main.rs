fn main() {

    let v = vec![101, 250, 330, 400];
    //vector v owns the object in heap

    //only a single variable owns the heap memory at any given time
    let _v2 = v.clone();
    //here two variable owns heap value,
    //two pointer to the same content is not allowed in rust

    //Rust is very smart in terms of memory access ,so it decet a race condition
    //as two variables point to same heap

    println!("{:?}",v);
}