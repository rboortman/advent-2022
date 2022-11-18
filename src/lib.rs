pub trait Assignment {
    // fn new<T: Assignment>(is_debug: bool) -> T;
    // fn parse_input<TInput>(input: String) -> TInput;
    // fn silver<TInput, TOutput>(input: TInput) -> TOutput;
    // fn gold<TInput, TOutput>(input: TInput) -> TOutput;
    fn run(&self, input: String, is_debug: bool) -> (String, String);
}
