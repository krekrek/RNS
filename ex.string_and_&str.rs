use std::any::type_name;

fn type_of<T>(_: T) -> &'static str { // 인자의 타입을 반환해주는 함
    type_name::<T>()
}
fn main() {
    // Rust는 String과 &str 두가지 문자열 타입이 있는데 둘의 차이는 간단하게
    // String은 문자열 수정이 가능하지만,
    // &str은 문자열 수정이 불가능하다.
    let name = "KIM DONG WOOK"; // 일반적인 선언 -> &str타입 (수정 불가능)
    let my_name = "DW_KIM".to_string(); // 문자열을 to_string으로 선언 -> String타입
    let other_name = String::from("krekrek"); // 문자열은 String::from으로 선언 -> String타입

    let mut my_other_name = "kr3v".to_string(); // String타입이기 때문에 문자열 수정이 가능
    println!("수정 전 문자열은 {} 이다.", my_other_name);
    my_other_name.push('-');
    my_other_name.push_str("kr3v");
    println!("수정 후 문자열은 {} 이다.\n", my_other_name);
    
    println!("name의 타입은 {} ,",type_of(name));
    println!("my_name은 {} ,", type_of(my_name));
    println!("other_name은 {} ,", type_of(other_name));
    println!("my_other_name은 {} 이다.", type_of(my_other_name));
}