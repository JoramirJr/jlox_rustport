pub trait ScanningParsingCommon {
    fn error(line: &u32, message: &str) -> ();
    fn report(line: &u32, location: String, message: &str);
}