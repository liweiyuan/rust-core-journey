use anyhow::Result;
use tokio::{
    fs,
    io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
};

async fn read_file(path: &str) -> Result<String> {
    let file = fs::read(path).await?;
    Ok(String::from_utf8_lossy(&file).to_string())
}

async fn write_file(path: &str, content: &str) -> Result<()> {
    let mut file = fs::File::create(path).await?;
    file.write_all(content.as_bytes()).await?;
    file.flush().await?;
    Ok(())
}

/// 流式读文件
async fn read_file_stream(path: &str) -> Result<String> {
    let file = fs::File::open(path).await?;
    let mut reader = BufReader::new(file);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).await?;
    Ok(String::from_utf8_lossy(&buf).to_string())
}

/// 流式写文件
async fn write_file_stream(path: &str, content: &str) -> Result<()> {
    let mut file = fs::File::create(path).await?;
    let mut writer = BufWriter::new(&mut file);
    writer.write_all(content.as_bytes()).await?;
    writer.flush().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_write_read_file() -> Result<()> {
        let content = "test";
        let path = "test.txt";
        write_file(path, content).await?;
        let read_content = read_file(path).await?;
        assert_eq!(content, read_content);

        // 删除测试文件
        fs::remove_file(path).await?;
        Ok(())
    }

    //使用临时文件来进行测试
    #[tokio::test]
    async fn test_write_read_temp_file() -> Result<()> {
        let content = "test";
        let binding = tempfile::NamedTempFile::new()?;
        let path = binding.path().to_str().unwrap();
        write_file(path, content).await?;
        let read_content = read_file(path).await?;
        assert_eq!(content, read_content);
        Ok(())
    }

    #[tokio::test]
    async fn test_write_read_stream() -> Result<()> {
        let content = "test";
        let path = "test_stream.txt";
        write_file_stream(path, content).await?;
        let read_content = read_file_stream(path).await?;
        assert_eq!(content, read_content);
        fs::remove_file(path).await?;
        Ok(())
    }

    //使用临时文件来进行测试
    #[tokio::test]
    async fn test_write_read_temp_file_stream() -> Result<()> {
        let content = "test";
        let binding = tempfile::NamedTempFile::new()?;
        let path = binding.path().to_str().unwrap();
        write_file_stream(path, content).await?;
        let read_content = read_file_stream(path).await?;
        assert_eq!(content, read_content);
        Ok(())
    }
}
