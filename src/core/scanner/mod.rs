use std::path::Path;

///
///
///
pub struct ScanResult {

    pub total_scan_size: f64

}

///
///
///
pub trait Scanner {

    ///
    ///
    ///
    fn do_scan(path: Path) -> ScanResult;

}
