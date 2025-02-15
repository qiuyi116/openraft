use crate::core::is_matched_upto_date;
use crate::Config;
use crate::LogId;

#[test]
fn test_is_line_rate() -> anyhow::Result<()> {
    let m = Some(LogId { term: 1, index: 10 });

    let cfg = |n| Config {
        replication_lag_threshold: n,
        ..Default::default()
    };

    assert!(is_matched_upto_date(&None, &None, &cfg(0)), "matched, threshold=0");
    assert!(
        is_matched_upto_date(&None, &Some(LogId { term: 2, index: 0 }), &cfg(1)),
        "matched, threshold=1"
    );
    assert!(
        !is_matched_upto_date(&None, &Some(LogId { term: 2, index: 0 }), &cfg(0)),
        "not matched, threshold=1"
    );

    assert!(
        is_matched_upto_date(&Some(LogId::new(0, 0)), &None, &cfg(0)),
        "matched, threshold=0"
    );

    assert!(
        is_matched_upto_date(&m, &Some(LogId { term: 2, index: 10 }), &cfg(0)),
        "matched, threshold=0"
    );
    assert!(
        is_matched_upto_date(&m, &Some(LogId { term: 2, index: 9 }), &cfg(0)),
        "overflow, threshold=0"
    );
    assert!(
        !is_matched_upto_date(&m, &Some(LogId { term: 2, index: 11 }), &cfg(0)),
        "not caught up, threshold=0"
    );
    assert!(
        is_matched_upto_date(&m, &Some(LogId { term: 2, index: 11 }), &cfg(1)),
        "caught up, threshold=1"
    );
    Ok(())
}
