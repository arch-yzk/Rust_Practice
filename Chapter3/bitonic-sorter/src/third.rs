// pubはこのsort関数が他のモジュールからアクセスできることを示す
// 引数xの型 &mut [u32]について
//  &は値をポインタ経由で借用することを示す
//  mutは値が変更可能であることを示す
//  u32型は32ビット符号なし整数
//  [u32]型はu32のスライス(一次元の配列みたいなものだと思ってくれ)

use super::SortOrder;
use std::cmp::Ordering;

pub fn sort_by<T, F>(x: &mut [T], comparator: &F) -> Result<(), String>
where F: Fn(&T, &T) -> Ordering
{
    if is_power_of_two(x.len())
    {
        do_sort(x, true, comparator);
        Ok(())
    }
    else
    {
        Err(format!("The length of x is not a power of two. (x.len(): {})", x.len()))
    }
}

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
/*
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
*/
fn do_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
where F: Fn(&T, &T) -> Ordering
{
    if x.len() > 1
    {
        let mid_point = x.len() / 2;
        
        // xをバイトニックソートする
        // 第2引数がtrueのときはcomparatorで示される順序でソート
        do_sort(&mut x[..mid_point], true, comparator);
        // 第2引数がfalseのときはcomparatorと逆順でソート
        do_sort(&mut x[mid_point..], false, comparator);
        
        sub_sort(x, forward, comparator);
    }
}

/*
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
*/
fn sub_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
where F: Fn(&T, &T) -> Ordering
{
    if x.len() > 1
    {
        compare_and_swap(x, forward, comparator);
        let mid_point = x.len() / 2;
        sub_sort(&mut x[..mid_point], forward, comparator);
        sub_sort(&mut x[mid_point..], forward, comparator);
    }
}

/*
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
*/
fn compare_amd_swap<T, F>(x: &mut [T], forward: bool, comparator: &F)
    where F: Fn(&T, &T) -> Ordering
{
    // 比較に先立ちforward(bool値)をOrdering値に変換しておく
    let swap_condition = if forward
    {
        Ordering::Greater
    }
    else
    {
        Ordering::Less
    };
    let mid_point = x.len() / 2;
    for i in 0..mid_point
    {
        // comparatorクロージャで2要素を比較し、返されたOrderingのバリアントが
        // swap_conditionと等しいなら要素を交換する
        if comparator(&x[i], &x[mid_point + i]) == swap_condition
        {
            x.swap(i, mid_point + i);
        }
    }
}

// 成功時はOk(())を、失敗時はErr(文字列)を返す
/*
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
*/
pub fn sort<T: Ord>(x: &mut [T], order: &SortOrder) -> Result<(), String>
{
    //do_sortを呼ぶ代わりに、sort_byを呼ぶようにする
    // is_power_of_twoはsort_byが呼ぶので、ここからは削除した
    match *order
    {
        // 昇順ならa.cmp(b)、降順ならb.cmp(a)を行う
        SortOrder::Ascending => sort_by(x, &|a, b| a.cmp(b)),
        SortOrder::Descending => sort_by(x, &|a, b| b.cmp(a)),
    }
}

// このモジュールはcargo testを実行したときのみコンパイルされる
#[cfg(test)]
mod tests
{
    // 親モジュール(first)のsort関数を使用する
    use super::sort;
    use super::{is_power_of_two, sort, sort_by}; // 変更
    use crate::SortOrder::*; // 追加モジュール
    use crate::utils::{new_u32_vec, is_sorted_ascending, is_sorted_descending};

    // 構造体Studentを定義する
    // 構造体は関連する値を1つにまとめたデータ構造
    // 複数のデータフィールドをもつ
    struct Student
    {
        first_name: String, // 名前
        last_name: String, // 苗字
        age: u8, // 年齢。u8(8ビット符号なし整数)
    }

    // implブロックを使うと対象の型に関連関数やメソッドを実装できる
    impl Student
    {
        // 関連関数newを定義する
        fn new(first_name: &str, last_name: &str, age: u8) -> Self
        {
            // 構造体Studentを初期化して返す。Selfはimpl対象の型(Student)の別名
            Self
            {
                // to_stringメソッドで&str型の引数からString型の値を作る
                first_name: first_name.to_string(),
                last_name: last_name.to_string(),
                age, // 値を設定
                // フィールドと変数が同名の場合は省略形で記述できる
            }
        }
    }

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

    #[test]
    fn sort_students_by_age_ascending()
    {
        let taro = Student::new("Taro", "Yamada", 16);
        let hanako = Student::new("Hanako", "Yamada",14);
        let kyoko = Student::new("Kyoko", "Ito", 15);
        let ryosuke = Student::new("Ryosuke", "Hayashi", 17);
        
        // ソート対象のベクタを作成する
        let mut x = vec![&taro, &hanako, &kyoko, &ryosuke];
        
        // ソート後の期待値を作成する
        let expected = vec![&hanako, &kyoko, &taro, &ryosuke];
        
        // sort_by関数でソートする。第2引数はソート順を決めるクロージャ
        // 2つのStudent構造体を取り、ageフィールドの値をcmpメソッドで比較する
        assert_eq!(sort_by(&mut x, &|a, b| a.age.cmp(&b.age)),Ok(()));
        /*
        assert_eq!(
        sort_by(&mut x, &|a, b| a.age.cmp(&b.age)),
        Ok(())
        );
        */
        
        assert_eq!(x, expected);
    }

    #[test]
    fn sort_students_by_name_ascending()
    {
        let taro = Student::new("Taro", "Yamada", 16);
        let hanako = Student::new("Hanako", "Yamada", 14);
        let kyoko = Student::new("Kyoko", "Ito", 15);
        let ryosuke = Student::new("Ryosuke", "Hayashi", 17);
        
        let mut x = vec![&taro, &hanako, &kyoko, &ryosuke];
        let expected = vec![&ryosuke, &kyoko, &hanako, &taro];
        
        assert_eq!
        (
            sort_by
            (
                &mut x,
                
                // まずlast_nameを比較する
                &|a, b| a.last_name.cmp(&b.last_name)
                 // もしlast_nameが等しくない(Less or Greater)ならそれを返す
                 // last_nameが等しいならfirst_nameを比較する
                 .then_with(|| a.first_name.cmp(&b.first_name))
            ), OK(())
        );
        assert_eq!(x, expected);
    }


    
    #[test]
    fn sort_u32_large()
    {
        {
            // 乱数で65,536要素のデータ列を作る(65,536は2^16)
            let mut x = new_u32_vec(65536);
            //昇順ソート
            assert_eq!(sort(&mut x, &Ascending), Ok(()));
            // ソートが正しいことを検証する
            assert!(is_sorted_ascending(&x));
        }
        {
            let mut x = new_u32_vec(65536);
            assert_eq!(sort(&mut x, &descending), Ok(()));
            assert!(is_sorted_descending(&x));
        }
    }
}
