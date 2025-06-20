fn main() {
    'outer: for i in 0..10 {
        'inner: for j in 0..10 {
            if j == 5 { break 'outer; }
        }
    }
}
