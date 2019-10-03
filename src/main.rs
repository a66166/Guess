use std::cmp::Ordering; //使用枚举匹配检查
use std::io; //导入标准库 
use rand::Rng;//使用外部依赖
fn main() {//主函数入口
    println!("猜数字!");
    //变量 = 随机函数生成器指定范围1..100
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("电脑生成随机数字是: {}", secret_number); //测试用，可以删除
    loop {//循环
        println!("请输入数字：");
         //新建一个可变量以字符串的形式
        let mut guess = String::new();
         //调用io::stdin()函数 read_line方法读取输入（参数为引用变量）
        io::stdin().read_line(&mut guess) //可以写成一行
            .expect("Failed to read line");
         //同名变量隐藏前者，字符串转换为u32类型与随机数同类型可以比较。
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,//match 检查匹配同类型继续
            Err(_) => continue,//不同类型进入下一次循环，避免崩溃。
        };
         //打印格式化字符串
        println!("你猜的数字: {}", guess);
         //match 检查分支进行匹配
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小了"),
            Ordering::Greater => println!("大了"),
            Ordering::Equal => {//增加条件赢了就退出
                println!("赢了！");
                break;//程序退出
            }
        }
    }
}
