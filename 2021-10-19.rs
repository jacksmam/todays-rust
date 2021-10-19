//コード7-13

fn main() {
  // 型を指定して初期化
  let mut v: Vec<i32> = Vec::new();
  println!("v.len is {}", v.len());
  // 要素を末尾に追加
  v.push(10);
  v.push(20);
  v.push(30);
  println!("v.len is {}", v.len());
  // 要素を末尾から取得
  println!("pop is {:?}", v.pop());
  println!("pop is {:?}", v.pop());
  println!("pop is {:?}", v.pop());
  println!("v.len is {}", v.len());
}
