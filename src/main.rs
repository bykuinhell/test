#[derive(Debug)]
struct Bio {
    name: String,
    age: u8,
    gender: String,
}

fn main() {
    let bio = Bio {
        name: String::from("张三"),
        age: 18,
        gender: String::from("男"),
    };

    println!(
        "姓名：{}，年龄：{}，性别：{}",
        bio.name, bio.age, bio.gender
    );
}
