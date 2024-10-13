use std::fs::File;

fn main() -> Result<(), AppError> {
    // 自定义错误
    // match create_app_error() {
    //     Err(e) => println!("{}",e),
    //     _ => println!("have no error")
    // }

    // Err(AppError::default())

    // ? 将错误进行隐形的转换
    // 因为实现了 From trait，可以将 std::io::Error 转换为 AppError
    // 所以下面关联函数的返回将会被转换成 AppError
    let _file = File::open("nonexistent_file.txt")?;

    Ok(())
}

#[allow(unused)]
fn create_app_error() -> Result<(), AppError> {
    Err(AppError::default())
}

struct AppError {
    code: usize,
    message: String,
}

impl Default for AppError {
    fn default() -> Self {
        AppError {
            code: 404,
            message: String::from("no page"),
        }
    }
}

impl std::fmt::Debug for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "custom Debug AppError {{ code: {}, message: {} }}",
            self.code, self.message
        )
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_msg = match self.code {
            404 => "Sorry! Page not found!",
            _ => "Sorry! something wrong!",
        };
        write!(f, "{}", err_msg)
    }
}

// 使用 From<T> 特征，将 T 类型转化为 Self
impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError {
            code: 10,
            message: error.to_string(),
        }
    }
}
