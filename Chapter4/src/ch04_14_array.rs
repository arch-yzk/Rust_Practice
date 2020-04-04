fn main()
{
    let a1 = [false, true, false] // [bool; 3]型
    let a2 = [0.0, -1.0, 1.0, 0.5]; // [f64; 4]型

    // `len()`で配列の長さを得る
    assert_eq!(a1.len(), 3);

    // 長さ100の配列を作り、全要素を0i32で初期化する
    // 要素の型はCopyトレイトを実装していなければならない
    let a3 = [0; 100];
    assert_eq!(a3.len(), 100);

    //配列は入れ子にできる。記述はしない

    // 配列の長さは実行時に指定することができない
    let size = 100;
    //let a1 = [0; size];
    // 上記はコンパイルエラー

    // ベクタなら実行時に長さを指定できる
    let mut v1 = vec![0; size]; // Vec<i32>型
    assert_eq!(v1.len(), 100);

    // ベクタには要素を追加したり、削除したり出来る
    v1.push(1); // 最後尾に要素を追加
    assert_eq!(v1.len(), 101)
    assert_eq!(v1.pop(), Some(1)); // 最後尾から要素を取り除く
    assert_eq!(v1.len(), 100);



    // インデックスを用いた要素へのアクセス

    let array1 = ['H', 'e', 'l', 'l', 'o'];
    assert_eq!(array1[1], 'e');

    let mut array2 = [0, 1, 2];
    array2[1] = 10;
    assert_eq!(array2, [0, 10, 2]);

    // インデックスは定数でなくても良い(Javaと同じ)

    // 範囲外にアクセスするとコンパイル時にエラー、わからなければパニックしてしまう
    let array3 = [0, 1];
    // array3[2];
    // 上記はコンパイルエラー
    let index = 2;
    // array3[index];
    // パニックしてしまう(変数はコンパイル時に通ってしまう)

    // 下はパニックしない`get()`メソッド
    assert_eq!(array3.get(1), Some(&1)); // インデックスが範囲内の場合はSome(&値)を返す
    assert_eq!(array3.get(2), None); // さもなければNoneを返す



    // イテレータを使用して要素へアクセス
    // 範囲外にアクセスする心配がない
    
}