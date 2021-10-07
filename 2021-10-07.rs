//コード7-7

fn main() {
  //0で初期化する
  let mut ary: [u8; 16] = [0; 16];
  println!("ary[0] is {}", ary[0]);
  println!("ary[15] is {}", ary[15]);
  //値を変更する
  ary[0] = 10;
  println!("ary[0] is {}", ary[0]);
  println!("ary[15] is {}", ary[15]);
}
