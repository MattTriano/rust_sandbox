fn main() {
    println!("Hello, world!");
    let x: u32 = 25;
    let y = 44;
    println!("int explicit type: {}", x);
    println!("int implicit type: {}", y);
    
    let x_f: f64 = -87.619283;
    let y_f = 41.877846;
    println!("float explicit type: {}", x_f);
    println!("float implicit type: {}", y_f);
    
    let x_bt: bool = true;
    let y_bf = false;
    println!("bool explicit type: {}", x_bt);
    println!("bool implicit type: {}", y_bf);
    
    let x_c = 'c';
    let y_c: char = 'Y';
    // let y_cc: char = 'YY';
    let heart_eyed_cat = '😻';
    println!("char explicit type: {}", x_c);
    println!("char implicit type: {}", y_c);
    // println!("{}", y_cc);
    println!("char non-[a-zA-Z0-9] unicode: {}", heart_eyed_cat);
    
    let x_tuple: (i8, char, bool) = (30, 'U', true);
    println!("tuple explicit types: {} {} {}", x_tuple.0, x_tuple.1, x_tuple.2);

    let arr_i = [0, 1, 2, 3];
    let arr_e: [u16; 4] = [10, 11, 12, 13];
    println!("array implicit types: {}, {}, {}, {}", arr_i[0], arr_i[1], arr_i[2], arr_i[3]);
    println!("array explicit types: {}, {}, {}, {}", arr_e[0], arr_e[1], arr_e[2], arr_e[3]);
}
