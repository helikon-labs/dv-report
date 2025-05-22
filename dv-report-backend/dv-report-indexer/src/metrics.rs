use dv_report_metrics::registry::IntGauge;
use once_cell::sync::Lazy;

const _METRIC_PREFIX: &str = "dv_report_indexer";

pub fn _indexed_finalized_block_number() -> IntGauge {
    static METER: Lazy<IntGauge> = Lazy::new(|| {
        dv_report_metrics::registry::register_int_gauge(
            _METRIC_PREFIX,
            "indexed_finalized_block_number",
            "Number of the last processed block",
        )
        .unwrap()
    });
    METER.clone()
}
