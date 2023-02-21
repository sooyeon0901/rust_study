

fn main(){
    // 점검
    assert_eq!(10_u8.checked_add(20), Some(30));
    assert_eq!(100_u8.checked_add(200), None); 
    // Some(300) 은 u8의 범위를 벗어나므로 에러 발생

    // 순환
    assert_eq!(100_u16.wrapping_mul(200), 20000);
    assert_eq!(500_u16.wrapping_mul(500), 53392);
    // 53392는 i16의 값을 벗어나므로 2의 16으로 나눈 나머지가 출력된다

    // 포화
    assert_eq!(32760_i16.saturating_add(10), 32767);
    // assert_eq!();
    // // 넘침
    assert_eq!(255_u8.overflowing_add(2), (1, true));
    // assert_eq!();
}