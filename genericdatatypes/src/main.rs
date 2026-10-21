pub trait Logger {
    fn log(&self, verbosity: u8, message: &str);
}
struct StdErrLogger;
impl Logger for StdErrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}
struct VerbosityFilter<L> {
    max_verbosity: u8,
    inner: L,
}
impl<L: Logger> Logger for VerbosityFilter<L>{
    fn log(&self, verbosity: u8, message: &str) {
        if verbosity <= self.max_verbosity {
            self.inner.log(verbosity,message);
        }
    }
}
fn main() {
    let logger = VerbosityFilter {max_verbosity: 3, inner: StdErrLogger};
    logger.log(5,"FYI");
    logger.log(2, "Uhoh");
}
