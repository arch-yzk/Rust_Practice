// pubはこのsort関数が他のモジュールからアクセスできることを示す
// 引数xの型 &mut [u32]について
//  &は値をポインタ経由で借用することを示す
//  mutは値が変更可能であることを示す
//  u32型は32ビット符号なし整数
//  [u32]型はu32のスライス(一次元の配列みたいなものだと思ってくれ)

use super::SortOrder;

/*
pub fn sort<T: Ord>(x: &mut [T], order: &SortOrder)
{
    match *order
    {
        SortOrder::Ascending => do_sort(x, true),
        SortOrder::Descending => do_sort(x, false),
    };
}
*/

//pub fn sort(x: &mut [u32], up: bool) {
fn do_sort<T: Ord>(x: &mut [T], up: bool)
{
    /* // unimplementedマクロは「未実装」の意味。実行するとpanicする
    unimplemented!();
    */
    if x.len() > 1
    {
        let mid_point = x.len() / 2;
        do_sort(&mut x[..mid_point], true);
        do_sort(&mut x[mid_point..], false);
        sub_sort(x, up);
    }
}

fn sub_sort<T: Ord>(x: &mut [T], up: bool)
{
    /*
    unimplemented!();
    */
    if x.len() > 1 {
        compare_and_swap(x, up);
        let mid_point = x.len() / 2;
        sub_sort(&mut x[..mid_point], up);
        sub_sort(&mut x[mid_point..], up);
    }
}

fn compare_and_swap<T: Ord>(x: &mut [T], up: bool)
{
    /*
    unimplemented!();
    */
    let mid_point = x.len() / 2;
    for i in 0..mid_point {
        if (x[i] > x[mid_point + i]) == up {
            // 要素を交換する
            x.swap(i, mid_point + i);
        }
    }
}

// 成功時はOk(())を、失敗時はErr(文字列)を返す
pub fn sort<T: Ord>(x: &mut [T], order: &SortOrder) -> Result<(), String>
{
    if x.len().is_power_of_two()
    {
        match *order
        {
            SortOrder::Ascending => do_sort(x, true),
            SortOrder::Descending => do_sort(x, false),
        };
        Ok(())
    }
    else
    {
        Err(format!("The length of x is not a power of two. (x.len(): {})", x.len()))
    }
}

// このモジュールはcargo testを実行したときのみコンパイルされる
#[cfg(test)]
mod tests
{
    // 親モジュール(first)のsort関数を使用する
    use super::sort;
    use crate::SortOrder::*; // 追加モジュール

    // #[test]のついた関数はcargo testしたときに実行される
    #[test]
    fn sort_u32_ascending()
    {
        // テストデータとしてu32型のベクタを作成しxに束縛する
        // sort関数によって内容が更新されるので、可変を表すmutキーワードが必要
        
        //let mut x = vec![10, 30, 11, 20, 4, 330, 21, 110];
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];

        // xのスライスを作成し、sort関数を呼び出す
        // &mut xは &mut x[..]と書いても良い
        assert_eq!(sort(&mut x, &Ascending), Ok(()));

        // xの要素が昇順にソートされていることを確認する
        assert_eq!(x, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }

    #[test]
    fn sort_u32_descending()
    {
        //let mut x = vec![10, 30, 11, 20, 4, 330, 21, 110];
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];
        assert_eq!(sort(&mut x, &Descending), Ok(()));
        // xの要素が降順にソートされていることを確認する
        assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4]);
    }

    #[test]
    fn sort_str_ascending()
    {
        let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];
        assert_eq!(sort(&mut x, &Ascending), Ok(()));
        assert_eq!(x, vec!["GC", "Rust", "and", "fast", "is", "memory-efficient", "no", "with"]);
    }
    
    #[test]
    fn sort_str_descending()
    {
        let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];
        assert_eq!(sort(&mut x, &Descending), Ok(()));
        assert_eq!(x, vec!["with", "no", "memory-efficient", "is", "fast", "and", "Rust", "GC"]);
    }

    #[test]
    fn sort_to_fail()
    {
        let mut x = vec![10, 30, 11]; // x.len()が2の累乗になっていない
        assert!(sort(&mut x, &Ascending).is_err()); // 戻り値はErr
    }
}