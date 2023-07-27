// fn main() {
//   let a = 10;
//   let b: i32 = 20;
//   let c = 30i32;
//   let d = 30_i32;
//   let e = add(add(a, b), add(c, d));

//   println!("(a + b) + (c + d) = {}", e);
// }

// fn add(i: i32, j: i32) -> i32 {
//   i + j
// }

// 整型溢出
fn main() {
  let a: u8 = 255;
  let b = a.wrapping_add(20);
  println!("{}", b);
}