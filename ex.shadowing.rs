// shadowing - 같은 변수 이름을 다시 쓰는 것
fn main() {
	let my_variable = 7;
	println!("{}", my_variable);
	{
			let my_variable = "Some String";
			println!("{}", my_variable); // Some String 출력
	} // 중괄호를 사용하였기 때문에 기존의 my_variable 변수를 건들이지 않고 String my_variable은 괄호가 끝남과 할께 사라짐.
	println!("{}", my_variable); // 7 출력
}