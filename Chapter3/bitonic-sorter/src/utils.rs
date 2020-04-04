use rand::{Rng, SeedableRng};
use rand::distributions::Standard;
use rand_pcg::Pcg64Mcg;

/*
pub fn new_u32_vec(n: usize) -> Vec<u32>
{
    // RNGを初期化する。再現性を持たせるため毎回同じシード値を使用する
    let mut rng = Pcg64Mcg::from_seed([0; 16]);
    // n個の要素が格納できるようベクタを初期化する
    let mut v = Vec::with_capacity(n);
    
    // 0からn - 1までの合計n回、繰り返し乱数を生成、ベクタに追加する
    // 0からn-1の数列は使わないので、_で受け、即時破棄している
    for _ in 0..n
    {
        // RNGのsampleメソッドは引数として与えられた分布にしたがう乱数を1つ生成する
        // Standard分布は生成する値が数値型(ここではu32型)のときは一様分布
        // 型が取りうる全値が同一の確率で出現する
        v.push(rng.sample(&Standard));
    }
    // ベクタを返す
    v
}
*/

pub fn new_u32_vec(n: usize) -> Vec<u32>
{
    let mut rng = Pcg64Mcg::from_seed([0; 16]);
    
    // rng.sample_iter()は乱数を無限に生成するイテレータを返す
    // take(n)は元のイテレータから最初のn要素だけを取り出すイテレータを返す
    // collect()はイテレータから値を収集して、ベクタや
    // ハッシュマップのようなコレクションに格納する
    rng.sample_iter(&Standard).take(n).collect()
}

pub fn is_sorted_ascending<T: Ord>(x: &[T]) -> bool
{
    // windows(2)は元のイテレータから1要素刻みで2要素ずつ値を取り出す
    // 新しいイテレータを返す。例えば元が[1, 2, 3, 4]なら
    // [1, 2], [2, 3], [3, 4]を順に返す
    
    // all(..)はイテレータから値([1, 2]のような)を取り出し、クロージャに渡す
    // クロージャがfalseを返したら。そこで処理を打ち切りfalseを返す
    // クロージャがtrueを返している間は、イテレータから次の値を取り出し
    // クロージャへ与え続ける。イテレータの値が尽きるまで(None)
    // クロージャがfalseを一度も返さなかったら、all(..)はtrueを与える
    x.windows(2).all(|pair| pair[0] <= pair[1])
}

pub fn is_sorted_descending<T: Ord>(x: &[T]) -> bool
{
    x.windows(2).all(|pair| pair[0] >= pair[1])
}