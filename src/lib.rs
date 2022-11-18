pub trait Assignment {
    fn run(&self, input: String, is_debug: bool) -> (String, String);
}
