####Rust는 메모리에 굉장히 민감하기 때문에 참조주소를 막 사용하지 못함.
- Rust 참조 예시

```rust
// 참조주소로 문자열 변경함수를 사용하는 예시

fn plus_n(n: &mut String){ // Plus String Function

n.push_str("_DONG_WOOk");

}

fn plus_s1(s1: &mut String){ // Plus String Function

s1.push_str(", World!");

}

fn main(){

let mut name = String::from("KIM"); // Mutable String

let name_ref = &mut name; // Mutable String's Reference Address

plus_n(name_ref); // Name Reference Address

println!("{}", name);

//------------------------------------------------------------------------//

let mut s1 = String::from("Hello"); // Mutable String

plus_s1(&mut s1); // Mutable String Reference Address

println!("{}", s1);

}
```