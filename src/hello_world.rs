use std::io;

fn main() {
    println!("안녕하세요! 이름을 입력해주세요.");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("입력을 읽는데 실패했습니다.");

    let name = name.trim();

    println!("Hello rust world, {}님!", name);
}
