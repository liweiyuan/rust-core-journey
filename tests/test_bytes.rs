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
    let mut buf = BytesMut::with_capacity(1024);

    let mut temp = [0u8; 512]; // 临时缓冲区

    while let Ok(n) = reader.read(&mut temp) {
        if n == 0 {
            break;
        }
        buf.put(&temp[..n]); // 将数据放入 BytesMut
    }
    buf.iter_mut().for_each(|b| *b = b.to_ascii_uppercase());
    Ok(buf)
}

//写入文件
fn write_file(path: &str, data: &[u8]) -> Result<()> {
    let mut file = open_file(path, true)?;
    file.write_all(data)?;
    Ok(())
}

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
