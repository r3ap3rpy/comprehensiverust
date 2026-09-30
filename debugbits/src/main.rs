use std::mem::transmute;

macro_rules! dbg_bits {
    ($e:expr, $bit_type: ty) => {
        println!("- {}: {:#x}",stringify!($e),transmute::<_,$bit_type>($e));
    };
}

fn main() {
    unsafe {
        dbg_bits!(false,u8);
        dbg_bits!(Some(true),u8);
        dbg_bits!(None::<Option<bool>>,u8);
        dbg_bits!(Some(&0i32),usize);
    }
}
