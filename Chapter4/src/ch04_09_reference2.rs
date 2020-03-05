fn main()
{
    let c1 = 'A';
    let c1_ptr = &c1; // &char型。イミュータブル(不変の参照)
    assert_eq!(*c1_ptr, 'A');

    let mut n1 = 0;
    let n1_ptr = &mut n1;
    assert_eq!(*n1_ptr, 0);

    // 可変の参照では参照先の値を変更できる
    *n1_ptr = 1_000;
    assert_eq!(*n1_ptr, 1_000);
}