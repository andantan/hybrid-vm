pub trait Executable {
    fn run(&mut self) -> Option<std::ffi::c_int>;
}