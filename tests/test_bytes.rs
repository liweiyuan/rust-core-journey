///
/// 学习使用bytes库来读写文件
///
///
use anyhow::{Ok, Result};
use bytes::BytesMut;
use std::{
    fs::File,
    io::{BufReader, Read, Write},
};

//文件打开操作
fn open_file(path: &str, write_mode: bool) -> Result<File> {
    if write_mode {
        Ok(File::create(path)?)
    } else {
        Ok(File::open(path)?)
    }
}

//读取文件
fn read_file(path: &str) -> Result<Vec<u8>> {
    let file = open_file(path, false)?;
    let mut reader = BufReader::new(file);
    let mut buf = BytesMut::with_capacity(1024);
    let mut processed_data = Vec::new();
    loop {
        buf.resize(1024, 0); // 确保缓冲区大小
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }
        buf.resize(n, 0); // 调整缓冲区大小为实际读取的大小
                          //转为大写,放入到processed_data中
        processed_data.extend(buf.iter().map(|b| b.to_ascii_uppercase()));
    }
    Ok(processed_data)
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
