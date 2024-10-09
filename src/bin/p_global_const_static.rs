use lazy_static::lazy_static;
use std::sync::{atomic::AtomicUsize, Mutex, OnceLock};

// 常量
const USER_NAME: &str = "OswaldSnow 🤣";

// 可变静态变量 必须在 unsafe 中修改和使用值
// 多线程情况下可能导致数据竞争和不安全的状态
static mut REQUEST_RECV: usize = 0;

// 为了多线程状态的数据安全可以使用 原子化数据
static ATOMIC_COUNTER: AtomicUsize = AtomicUsize::new(0);

// lazy_static库  懒加载的方式定义全局静态数据 运行时加载
// 变量都是不可变引用
lazy_static! {
    static ref NAMES: Mutex<String> = Mutex::new(String::from("Sunface, Jack, Allen"));
}

// OnceLock 可以在多线程中使用
static CONFIG_SYNC: OnceLock<Config> = OnceLock::new();

// OnceCell 不可在多线程中使用
// 提示错误
// `OnceCell<String>` cannot be shared between threads safely the trait `Sync` is not implemented for `OnceCell<String>`
// OnceCell 无法作为静态变量在多线程中安全传输
// static LOGGET: OnceCell<String> = OnceCell::new();

fn main() {
    println!("USER_NAME is {}", USER_NAME);

    unsafe {
        REQUEST_RECV += 1;
    }
    println!("request receive count is {}", unsafe { REQUEST_RECV });

    for _ in 0..100 {
        ATOMIC_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    let atomic_cur_val = ATOMIC_COUNTER.load(std::sync::atomic::Ordering::SeqCst);
    println!("atomic current val is {}", atomic_cur_val);

    println!("NAMES is {}", NAMES.lock().unwrap());

    // 使用 Box::leak 创建一个全局变量
    // Bok 可以将一个变量 leak 泄漏出去，变成一个拥有静态生命周期的数据
    // 例如 在运行时创建一个静态变量
    let app_config = init().unwrap();
    println!(
        "current app_name is {}, app_version is {}",
        app_config.app_name, app_config.app_version
    );

    app_config.app_name = "freedom china future life style".to_string();
    println!(
        "updated app_name is {}, app_version is {}",
        app_config.app_name, app_config.app_version
    );

    // 多线程情况下还可以使用 OnceLock
    let config_sync = init_by_oncelock();
    println!(
        "app_name is {}, app_version is {}",
        config_sync.app_name, config_sync.app_version
    );

    println! {"CONFIG_SYNC is {:?}", CONFIG_SYNC};

    let config_sync = init_by_oncelock();
    println!(
        "app_name is {}, app_version is {}",
        config_sync.app_name, config_sync.app_version
    );
}

#[derive(Debug)]
struct Config {
    app_name: String,
    app_version: String,
}

impl Config {
    fn new(app_name: String, app_version: String) -> Self {
        Config {
            app_name,
            app_version,
        }
    }
}

fn init() -> Option<&'static mut Config> {
    Some(Box::leak(Box::new(Config::new(
        "future life style".to_string(),
        "1.1.0".to_string(),
    ))))
}

fn init_by_oncelock() -> &'static Config {
    CONFIG_SYNC.get_or_init(|| Config::new("OswaldSnow app".to_string(), "1.8.1".to_string()))
}
