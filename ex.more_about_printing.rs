fn main() { 
    let my_variable = 27;
    println!("{:x} {:o} {:b}", my_variable,my_variable,my_variable);
    // 위와 같이 {:-}는 C언어에서의 포멧스트링(%) 처럼 사용되며 아래와 같은 옵션들이 있음.
    // - ``, which uses the `Display` trait
    // - `?`, which uses the `Debug` trait
    // - `e`, which uses the `LowerExp` trait
    // - `E`, which uses the `UpperExp` trait
    // - `o`, which uses the `Octal` trait
    // - `p`, which uses the `Pointer` trait
    // - `b`, which uses the `Binary` trait
    // - `x`, which uses the `LowerHex` trait
    // - `X`, which uses the `UpperHex` trait


    let title = "TODAY'S NEWS";
    println!("{:-^30}", title); // 30개의 '-' 사이에 해당 문자열을 출력
    let bar = "|";
    println!("{: <15}{: >15}", bar, bar); // 15개의 공백 왼쪽과 15개의 공백 오른쪽에 해당 문자를 출력
    let a = "SEOUL";
    let b = "TOKYO";
    println!("{:-<15}{:->15}", a, b); // 15개의 '-'왼쪽에 a문자열을, 오른쪽에 b문자열을 출력
}