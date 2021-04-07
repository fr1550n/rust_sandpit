use cpu_temp::read_temp;
mod cpu_temp;

fn main() {
    println!("{:?}", read_temp().unwrap());
}
