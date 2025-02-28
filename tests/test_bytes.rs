///
/// 学习使用bytes库来读写文件
///
///
use anyhow::{Context, Result};
use bytes::{BufMut, BytesMut};
use std::{
    fs::File,
    io::{BufReader, Read, Write},
};

//文件打开操作
fn open_file(path: &str, write_mode: bool) -> Result<File> {
    if write_mode {
        File::create(path).with_context(|| format!("Failed to create file: {}", path))
    } else {
        File::open(path).with_context(|| format!("Failed to open file: {}", path))
    }
}

//读取文件
fn read_file(path: &str) -> Result<BytesMut> {
    let file = open_file(path, false)?;
    let mut reader = BufReader::new(file);
    let mut buf = BytesMut::with_capacity(4096);

    let mut temp = [0u8; 512]; // 临时缓冲区

    loop {
        let n = reader.read(&mut temp)?;
        if n == 0 {
            break;
        }
        buf.reserve(n);
        buf.put(&temp[..n]);
    }

    buf.iter_mut().for_each(|b| *b = b.to_ascii_uppercase());
    Ok(buf)
}

//写入文件
fn write_file(path: &str, data: &[u8]) -> Result<()> {
    let mut file = open_file(path, true)?;
    file.write_all(data)?;
    file.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_file_use_bytes() -> Result<()> {
        //打开文件
        let input_path = "tests/inputs/input_bytes.txt";
        let output_path = "tests/outputs/output_bytes.txt";
        //读取文件,使用上面的方法
        let data = read_file(input_path)?;
        //写入文件
        write_file(output_path, &data)
    }

    //增加一个文件不存在的校验
    #[test]
    fn test_file_not_exist() -> Result<()> {
        //打开文件
        let input_path = "tests/inputs/input_bytes_not_exist.txt";
        //读取文件,使用上面的方法
        let result = read_file(input_path);
        assert!(result.is_err());

        if let Err(e) = result {
            assert_eq!(
                e.to_string(),
                format!("Failed to open file: {}", input_path)
            );
        }
        Ok(())
    }

    #[test]
    fn test_read_non_existent_file() -> Result<()> {
        let non_existent_path = "tests/inputs/non_existent_file.txt";
        let result = read_file(non_existent_path);
        assert!(result.is_err());

        // 可选：进一步验证错误类型或消息
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Failed to open file"));

        Ok(())
    }

    #[test]
    fn test_read_and_write_file() -> Result<()> {
        // 创建临时输入文件
        let input_temp = NamedTempFile::new()?;
        let input_path = input_temp.path();
        let test_data = b"hello, world!";
        write_file(input_path.to_str().unwrap(), test_data)?;

        // 读取文件内容
        let data = read_file(input_path.to_str().unwrap())?;
        assert_eq!(data, BytesMut::from(&b"HELLO, WORLD!"[..]));

        // 创建临时输出文件
        let output_temp = NamedTempFile::new()?;
        let output_path = output_temp.path();

        // 写入转换后的数据
        write_file(output_path.to_str().unwrap(), &data)?;

        // 验证输出文件内容
        let output_data = read_file(output_path.to_str().unwrap())?;
        assert_eq!(output_data, BytesMut::from(&b"HELLO, WORLD!"[..]));

        // 可选：清理临时文件（NamedTempFile 会在析构时自动删除）
        Ok(())
    }
}
