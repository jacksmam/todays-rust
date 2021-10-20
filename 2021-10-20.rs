//コード7-23

fn main() {
  //ベクターを定義
  let v = vec![1,2,3,4,5];
  //for文で表示
  print!("for is ");
  for i in &v {
    print!("{} ", i);
  }
  println!("");
  //for文とイテレーターの利用
  print!("for and iter is ");
  for i in v.iter() {
    print!("{} ", i);
  }
  println!("");
}
