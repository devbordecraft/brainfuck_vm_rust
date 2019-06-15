pub struct SyntaxChecking {}

impl SyntaxChecking {
    pub fn check(&self, code_to_check:&String) -> bool{

        // Check if numbers of bracket is equals
        let number_of_close_bracket = code_to_check.matches("]").count();
        let number_of_open_bracket = code_to_check.matches("[").count();

        if number_of_open_bracket != number_of_close_bracket{
            return false
        }
        true
    }
}