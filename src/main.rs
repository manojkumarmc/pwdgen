use rand::Rng;

const WIDTH: i8= 8;
const PWD_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789~!@#$%^&*";


fn main() {
    let mut rng = rand::thread_rng();
    println!("{}", PWD_CHARS);
    let mut pwd: String = String::from("");
    for _x in 1..=WIDTH {
        let rc = rng.gen_range(1..=PWD_CHARS.len());
        let pchars = match PWD_CHARS.chars().nth(rc) {
            Some(pc) => pc,
            None => ' ',
        };
        pwd.insert(0, pchars);
    }
    println!("{}", pwd);
}
