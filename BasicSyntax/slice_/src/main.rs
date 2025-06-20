fn main() {
    let arr = [10, 20, 30, 40, 50];

    let a = &arr[..];      // 全部切片（等于 &arr）
    let b = &arr[1..3];    // 从下标 1 到 2（不包括 3）
    let c = &arr[..2];     // 从下标 0 到 1
    let d = &arr[2..];     // 从下标 2 到结束

}
