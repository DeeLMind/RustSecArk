
fn fn_match(){
    let result : Result<i32,&str> = Ok(1);

    match result {
        Ok(val) => {
            println!("{:?}",result);
        }
        Err(err) => {}
    }

    if let Ok(val) = result {
        println!("成功值是: {}", val);
    }

    if let Err(err) = result {
        println!("错误信息是: {}", err);
    }

    let val = result.unwrap();           // Err 会 panic
    let val = result.expect("错误提示"); // Err 会 panic 并显示提示信息

    let val = result.unwrap_or(0);                   // 如果是 Err，使用 0
    let val = result.unwrap_or_else(|e| {
        println!("出错了: {}", e);
        0
    });

    if result.is_ok() {
        println!("是成功的");
    }

    if result.is_err() {
        println!("是失败的");
    }

    let opt = result.ok();  // Ok(val) => Some(val), Err(_) => None
    let opt = result.err(); // Err(err) => Some(err), Ok(_) => None

    let new_result = result.map(|val| val * 2);            // Ok(val) -> Ok(val * 2)
    let new_result = result.map_err(|e| format!("错误: {}", e)); // Err(e) -> Err(新e)

    // Ok 才继续执行下一个 Result
    let new_result = result.and_then(|val| {
        if val > 0 {
            Ok(val * 10)
        } else {
            Err("值太小")
        }
    });
    
    
    
}

fn try_parse(s: &str) -> Result<i32, std::num::ParseIntError> {
    let num = s.parse::<i32>()?; // 如果 parse 出错会自动 return Err
    Ok(num * 2)
}

// Result<T, E>
// ├── match / if let          // 模式匹配
// ├── unwrap / expect         // 直接提取（危险）
// ├── is_ok / is_err          // 判断状态
// ├── ok() / err()            // 转 Option
// ├── unwrap_or(_else)        // 提供默认值
// ├── map / map_err           // 转换值
// ├── and_then / or_else      // 链式操作
// ├── ?                       // 快速 return Err
// └── inspect(_err)           // 调试用


fn main() {
    fn_match()
}
