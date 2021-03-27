use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let cpu_path     = "/sys/class/thermal/thermal_zone0/temp";
    let mut file     = File::open(cpu_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents  = contents.trim_end().to_string();
    let cpu_temp: f32 = match contents.parse::<f32>() {
        Ok(r)  => r / 1000_f32,
        Err(e) => {
            println!("{:?}", e);
            42.42_f32
        }
    };    
    println!("{}", cpu_temp);
    Ok(())
}
