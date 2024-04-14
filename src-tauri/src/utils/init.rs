use std::env;
use std::fs;

pub fn init() {
    let env_dir = env::current_dir().expect("Failed to get current directory");
    let dir = env_dir.join("trailnext");

    if !dir.exists() {
        // 创建子目录
        // match fs::create_dir(&dir) {
        //     Ok(_) => println!("子目录创建成功: {:?}", dir),
        //     Err(e) => println!("子目录创建失败: {}", e),
        // }
    } else {
        println!("子目录已存在: {:?}", dir);
    }
}
