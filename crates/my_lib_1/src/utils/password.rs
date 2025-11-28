use rand::seq::IndexedRandom;
// 引入这个 trait 用于 .choose()
use rand::{rng, Rng};
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Debug, Zeroize, ZeroizeOnDrop)]
pub struct SecurePassword(String);

impl SecurePassword {
    pub fn new(length: usize) -> Self {
        let mut rng = rng();

        // 定义字符集 (放在循环外)
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                 abcdefghijklmnopqrstuvwxyz\
                                 0123456789!#$%^&*()-_=+[]{}";

        // 1. 预分配内存，避免 String 在扩容时产生未擦除的内存副本
        let mut password_content =
            String::with_capacity(length);

        // 2. 直接生成
        for _ in 0..length {
            // 使用 choose 直接从切片选值，比手动索引更语义化且安全
            let char_byte = CHARSET
                .choose(&mut rng)
                .expect("Charset is empty");
            password_content.push(*char_byte as char);
        }

        Self(password_content)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generation() {
        let pwd = SecurePassword::new(32);
        // 注意：在实际生产日志中不要打印明文密码，这里仅做测试演示
        println!("Generated:\n {}", pwd.as_str());
        assert_eq!(pwd.as_str().len(), 32);
    }
}
