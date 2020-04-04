fn main()
{
    let t1 = (88, true);

    // フィールド0の要素(左から数えて最初の要素)を取り出す
    assert_eq!(t1.0, 88);

    // フィールド1の要素(2番目の要素)を取り出す
    assert_eq!(t1.1, true);

    // フィールド名にはコンパイル時の定数のみ使える。変数は不可能
    //let i = 0;
    //let t1a = t1.i;
    // 上記はコンパイルエラー
    // no field `i` on type `({integer}, bool)`

    // 要素を書き換えるので、変数t1にmutをつけて可変にする
    let mut t1 = (88, true);

    // フィールド0の要素を書き換える
    t1.0 += 100; // 現在の値に100を足す
    assert_eq!(t1, (188, true));



    // パターンマッチで分解する
    let (n1, b1) = (88, true);
    assert_eq!(n1, 88);
    assert_eq!(b1, true);

    let ((x1, y1), (x2, y2)) = ((0, 5), (10, -1));
    assert_eq!(x1, 0);
    assert_eq!(y1, 5);
    assert_eq!(x2, 10);
    assert_eq!(y2, -1);

    // 不要な値はアンダースコアを使うと無視できる
    let ((x1, y1), _) = ((0, 5), (10, -1));

    let mut t1 = ((0,5), (10, -1));

    // 要素を指す可変の参照を得るためにref mutを追加する
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t1;

    // *を付加するとポインタが指すアドレスにあるデータにアクセスできる
    *x1_ptr += 3;
    *y1_ptr *= -1;

    assert_eq!(t1, ((3, -5), (10, -1)));
}