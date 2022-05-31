fn main() {
    let title = "TODAY'S NEWS";
    println!("{:-^30}", title); // 30개의 '-' 사이에 해당 문자열을 출력
    let bar = "|";
    println!("{: <15}{: >15}", bar, bar); // 15개의 공백 왼쪽과 15개의 공백 오른쪽에 해당 문자를 출력
    let a = "SEOUL";
    let b = "TOKYO";
    println!("{:-<15}{:->15}", a, b); // 15개의 '-'왼쪽에 a문자열을, 오른쪽에 b문자열을 출력
}