use anyhow::Result;
use tokio::{fs, io::AsyncWriteExt};

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
        Ok(())
    }
}
