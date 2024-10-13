use std::fs::read_to_string;

fn main() -> Result<(), MyError> {
    /*
    可以将多种不同的错误转化为一种 例如自定义的错误类型
    使用 dyn 特征对象 或者 此种方式
    此代码示例中的方式较为灵活
     */
    let html = render()?;
    println!("{}", html);
    Ok(())
}

fn render() -> Result<String, MyError> {
    let file = std::env::var("MARKDOWN")?;
    let source = read_to_string(file)?;
    Ok(source)
}

#[derive(Debug)]
enum MyError {
    EnvironmentVariableNotFound,
    IOError(std::io::Error),
}

impl From<std::env::VarError> for MyError {
    fn from(_: std::env::VarError) -> Self {
        Self::EnvironmentVariableNotFound
    }
}

impl From<std::io::Error> for MyError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

// impl std::error::Error for MyError {}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::EnvironmentVariableNotFound => write!(f, "Environment variable not found"),
            MyError::IOError(err) => write!(f, "IO Error: {}", err.to_string()),
        }
    }
}
