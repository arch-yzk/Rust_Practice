fn main()
{
    let n1 = 200u8;
    let n2 = 3u8;

    // n1 * n2 = 600を計算する
    //std::u8::MAXは255なので桁溢れする

    // 検査付き乗算 -> Noneになる
    assert_eq!(n1.checked_mul(n2), None);

    // 飽和乗算 -> u8の最大値255にはりつく
    assert_eq!(n1.saturating_mul(n2), std::u8::MAX);

    // ラッピング乗算 -> 600を256で割ったあまりの88になる
    assert_eq!(n1.wrapping_mul(n2), 88);

    // 桁溢れ乗算 -> 88と桁溢れを示すtrueのペアを返す
    assert_eq!(n1.overflowing_mul(n2), (88, true));
}