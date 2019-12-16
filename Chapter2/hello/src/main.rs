fn main()
{
    //exp関数をRPN形式の文字列に変換する
    let exp = "6.1 5.2 4.3 * + 3.4 2.5 / 1.6 * -";

    //rpn関数で上記を計算する
    let ans = rpn(exp);

    //デバッグビルド時のみチェックする
    //小数点以下4桁までを文字列に変換
    debug_assert_eq!("26.2840", format!("{:.4}", ans)); // format!("", )が文字列変換？

    println!("{} = {:.4}", exp, ans);
}

fn rpn(exp: &str) -> f64
{
    //Vecでスタック「stack」を定義
    let mut stack = Vec::new();

    //expをスペースで分割、tokenを順に束縛
    for token in exp.split_whitespace()
    {
        //tokenがf64ならスタックに積む
        if let Ok(num) = token.parse::<f64>()
        {
            stack.push(num);
        }
        else
        {
            //演算子かどうか
            match token
            {
                //演算子ならapply2関数で計算
                "+" => apply2(&mut stack, |x, y| x + y),
                "-" => apply2(&mut stack, |x, y| x - y),
                "*" => apply2(&mut stack, |x, y| x * y),
                "/" => apply2(&mut stack, |x, y| x / y),

                //tokenが非演算子ならエラーを起こして強制終了
                _ => panic!("unknown operator: {}", token),
            }
        }
    }
    //スタックから1つ取り出す 失敗したらエラーを起こして終了する
    stack.pop().expect("Stack underflow")
}

//スタックから数値を2つ取り出し、F型のクロージャfunで計算、スタックに積む
fn apply2<F>(stack: &mut Vec<f64>, fun: F)

//F型のトレイト境界。
where
    F: Fn(f64, f64) -> f64
{
    //yとxをスタックの最後の2要素に束縛する
    if let(Some(y), Some(x)) = (stack.pop(), stack.pop())
    {
        //funクロージャで計算、zに束縛する
        let z = fun(x, y);
        
        stack.push(z);
    }
    else
    {
        //スタック要素を取れなかったらエラー
        panic!("Stack underflow");
    }
}