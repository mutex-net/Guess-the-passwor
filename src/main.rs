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
        
        println!("请输入你的密码：");

        println!("当前密码格式：\n⬡⬡⬡⬡");
        io::stdin()
            .read_line(&mut guess) // 读取之后写进guess
            .expect("读取失败"); // 读取失败之后返回括号内容

        println!("你的输入：{}", guess);
        // 将用户输入的内容转换为数字 

        let jubge_guess = guess.trim();
        // 将判断条件整合

        if jubge_guess == "printguess" {
            println!("悄悄告诉你，密码是{}", secret_number);
            continue;
        } 
        else if jubge_guess == "exit" {
            println!("游戏结束！");
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("错误，请输入关键字或者数字");
                continue;},
        };

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
