pub struct SyntaxChecking {}

impl SyntaxChecking {
    pub fn check(&self, code_to_check:&String) -> i8{
        /*
        Function that handle the syntax checking, return bool
        error codes :
            => 5 : Brackets are not close
            => 10 : out of memory
        */

        let number_of_close_bracket = code_to_check.matches("]").count();
        let number_of_open_bracket = code_to_check.matches("[").count();

        if number_of_open_bracket != number_of_close_bracket{
            return 5;
        }
        0
    }
}