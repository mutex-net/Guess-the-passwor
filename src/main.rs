// 引用的库
use std::io;
use rand::Rng;

fn main() {
    let mut guess = String::new(); // 创建一个待填写的字符串
    let secret_number = rand::thread_rng().gen_range(1..101); // 创建一个随机数生成器

    println!("Guess The Number!");
    println!("这次的正确密码是：{}", secret_number);
    println!("请输入你猜的密码：");
    // 用于读取用户输入
    io::stdin()
        .read_line(&mut guess) // 读取之后写进guess
        .expect("读取失败"); // 读取失败之后返回括号内容

    println!("你的输入：{}", guess);
}
