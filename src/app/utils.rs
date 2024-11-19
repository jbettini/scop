use std::collections::HashSet;


pub fn has_duplicate(f: &Vec<u32>) -> bool {
    let mut tmp:  HashSet<u32> = HashSet::new();
    for x in f {
        if tmp.contains(&x) {
            return true;
        }
        tmp.insert(x.clone());
    }
    return false;
}

pub fn print_help() {
    println!("\x1b[1;31m## Available Commands ##\x1b[0m");
    println!("\x1b[32mV\x1b[0m : Change polygon draw mode (fill, line, point)");
    println!("\x1b[32mB\x1b[0m : Toggle backface culling");
    println!("\x1b[32mM\x1b[0m : Switch between Gouraud or Blinn-Phong shading");
    println!("\x1b[32mL\x1b[0m : Switch between moving fov or light.");
    println!("\x1b[32mZ\x1b[0m : Move forward in Z");
    println!("\x1b[32mS\x1b[0m : Move backward in Z");
    println!("\x1b[32mQ\x1b[0m : Move left");
    println!("\x1b[32mD\x1b[0m : Move right");
    println!("\x1b[32mX\x1b[0m : Reverse rotation direction");
    println!("\x1b[32mH\x1b[0m : Display this help");
    println!("\x1b[32mUp Arrow\x1b[0m    : Move up");
    println!("\x1b[32mDown Arrow\x1b[0m  : Move down");
    println!("\x1b[32mLeft Arrow\x1b[0m  : Increase rotation speed to the left or decrease to the right");
    println!("\x1b[32mRight Arrow\x1b[0m : Increase rotation speed to the right or decrease to the left");
    println!("\x1b[1;31m------------------------\x1b[0m");
}