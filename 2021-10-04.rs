//コード6-24

#[derive(Debug)]
enum LANG {
  JAPANESE,
  ENGLISH,
  CHINESE,
  FRENCH,
}

fn main () {
  let lang = LANG::JAPANESE;
  //すべての候補を列挙する
  let m = match lang {
    LANG::JAPANESE => "日本語",
    LANG::ENGLISH => "英語",
    LANG::CHINESE => "中国語",
    LANG::FRENCH => "フランス語",
  };

  println!("lang is {}", m);
}
