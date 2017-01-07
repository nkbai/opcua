use types::*;

#[test]
fn test_epoch() {
    let epoch = DateTime::ymd_hms_nano(1601, 1, 1, 0, 0 , 0, 0);
    assert_eq!(epoch.ticks(), 0);
    assert_eq!(epoch.checked_ticks(), 0);
}

#[test]
fn test_before_epoch() {
    let epoch = DateTime::ymd_hms_nano(1600, 12, 31, 23, 59 , 59, 999_999);
    assert_eq!(epoch.checked_ticks(), 0);
}

#[test]
fn test_epoch_plus_1tick() {
    let epoch = DateTime::ymd_hms_nano(1601, 1, 1, 0, 0 , 0, 100);
    assert_eq!(epoch.ticks(), 1);
}

#[test]
fn test_endtimes() {
    let endtimes = DateTime::ymd_hms_nano(9999, 12, 31, 23, 59 , 59, 999_999);
    assert_eq!(endtimes.checked_ticks(), i64::max_value());

    let endtimes = DateTime::ymd_hms_nano(10000, 1, 1, 0, 0 , 0, 0);
    assert_eq!(endtimes.checked_ticks(), i64::max_value());
}

#[test]
fn test_time() {
    let now = DateTime::now();
    assert!(now.year > 2000 && now.year < 2050);
    assert!(now.month >= 1 && now.month <= 12);
}