fn takes_tuple(my_tuple: (char,i32,bool)) {
    let a = my_tuple.0;
    let b = my_tuple.1;
    let c = my_tuple.2;

    let (a,b,c) = my_tuple;

    let (_,b,c) = my_tuple;
    let (..,c) = my_tuple;
}
fn main() {
    takes_tuple(('a',12,true));
}
