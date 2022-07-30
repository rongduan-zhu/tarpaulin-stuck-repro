fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use tokio::process::Command;

    #[tokio::test]
    async fn command_test() {
        let status = Command::new("echo").arg("hello world").status().await;
        assert!(status.is_ok());
    }
}
