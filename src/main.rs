#[derive(Debug)]
enum Error{
    Overflow,
    Zero,
}

fn main() {
    //Result型もOption同様unwrapを用いる
    println!("x is {}", double(10).unwrap());

    if let Err(e) = func(){
        println!("func failed with {:?}", e);
    }
}

fn func() -> Result<u32, Error>{
    //エラーが発生すると上位関数にエラーをそのまま伝搬する
    let doubled = double(u32::MAX)?;
    //エラーが発生しなかった場合の処理
    Ok(doubled)
}

//エラーか値のどちらかが格納されているときはResult型
fn double(x:u32) -> Result<u32, Error>{
    if x == 0{
        //エラーの時はErr(e)
        Err(Error::Zero)
    }
    else if x > (u32::MAX / 2){
        Err(Error::Overflow)
    }
    else{
        //エラーじゃない時はOk(T)
        Ok(x * 2)
    }

    //ResultもOption同様matchやif letを用いたパターンマッチが可能
}
