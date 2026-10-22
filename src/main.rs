// 引用的库
use std::cmp::Ordering;
use rand::Rng;
use std::io;

fn main() {
    
    let secret_number:u32 = rand::thread_rng().gen_range(1..101); 
    // 创建一个随机数生成器

    println!("Guess The Number!");

    loop {
        let mut guess = String::new();
        // 创建一个待填写的字符串 

        print!("请输入你的密码：");

        io::stdin()
            .read_line(&mut guess) // 读取之后写进guess
            .expect("读取失败"); // 读取失败之后返回括号内容

        println!("你的输入：{}", guess);

        println!("本次的密码是：{}", if guess.trim() == "printguess" {secret_number.to_string()} else {"保密！".to_string()});

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // 将用户输入转化为整数
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了！"),
            Ordering::Greater => println!("太大了！"),
            Ordering::Equal => {
                println!("你赢了！");
                break;
            }
        }
    }
}