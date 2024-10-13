use std::fmt::Error;

fn main() {
    // or else or_else and_then
    let o1_none: Option<&str> = None;
    let o2_some: Option<&str> = Some("123");

    assert_eq!(o2_some, o2_some.or(o1_none));

    assert_eq!(o1_none, o2_some.and(o1_none));

    assert_eq!(Some("Yes"), o1_none.or_else(|| { Some("Yes") }));

    assert_eq!(None, o1_none.and_then(|_| { Some(123) }));

    let r1_ok: Result<&str, Error> = Ok("ok");
    let r2_error: Result<&str, Error> = Err(Error);

    assert_eq!(r1_ok, r1_ok.or(r2_error));

    assert_eq!(r2_error, r1_ok.and(r2_error));

    assert_eq!(Err(Error), r2_error.or_else(|Error| { Err(Error) }));

    assert_eq!(r1_ok, r1_ok.and_then(|_| { Ok("ok") }));

    // 针对于 Option 的 filter
    println!("filter result is {:?}", o2_some.filter(|_| { false }));

    // map map_err 将来 Some 或 Ok 中的值映射为另一个
    let o2_some: Option<&str> = Some("123");
    assert_eq!(Some(123), o2_some.map(|_| { 123 }));

    let r1_ok: Result<&str, Error> = Ok("ok");
    assert_eq!(Ok(123), r1_ok.map(|_| { 123 }));
    // 但是对于 Result，map 方法只能修改 Ok 中的值类型，不能修改 Err 的值类型
    // 此时就需要使用 map_err
    let r2_error: Result<&str, i32> = Err(6);
    assert_eq!(
        Err("this is new error type"),
        r2_error.map_err(|_| { "this is new error type" })
    );

    // map_or map_or_else
    // 当为 None 或 Err 时提供默认值，map_or 提供一个默认值 map_or_else 使用闭包提供默认值

    // ok_or ok_or_else 将 Option 转换为 Result
    // Some(T) -> Ok(T)
    // None -> Err(default)
    // ok_or_else 使用闭包
}
