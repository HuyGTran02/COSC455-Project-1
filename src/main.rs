
pub trait Compiler{
    // Compiles the source code into an intermediate representation or target code.
    fn compile(&mut self, source: &str);
    // Returns the next token from the source code.
    fn next_token(&mut self) -> String;
    // Parses the source code and builds an abstract syntax tree (AST).
    fn parse(&mut self);
    // Generates the final output code from the AST or intermediate representation.
    fn current_token(&self) -> String;
    // Sets the current token to the specified value.
    fn set_current_token(&mut self, token: String);
}
// A trait for lexical analysis, which is responsible for breaking the source code into tokens.
pub trait LexicalAnalyzer {
    fn get_char(&mut self) -> char;
    fn add_char(&mut self, c: char);
    fn lookup(&self, s: &str) -> bool;
}

// A struct representing the LolCompiler, which implements the Compiler and LexicalAnalyzer traits.
struct LolCompiler{
        source: Vec<char>,
        position: usize,
        lexeme: String,
        current_token: String,
}

impl LexicalAnalyzer for LolCompiler {

    // Get the next character from the source
    fn get_char(&mut self) -> char {
        if self.position < self.source.len() {
            let c: char = self.source[self.position];
            self.position += 1;
            c
        } else {
            '\0'
        }
    }

    // Add a character to the current lexeme
    fn add_char(&mut self, c: char) {
        self.lexeme.push(c);
    }

    // Check if the lexeme is a valid LOLCODE annotation
    fn lookup(&self, s: &str) -> bool {
        let token: String = s.to_uppercase();

        matches!(
            token.as_str(),
            "#HAI"
                | "#KBYE"
                | "#OBTW"
                | "#TLDR"
                | "#MAEK HEAD"
                | "#GIMMEH TITLE"
                | "#MAEK PARAGRAF"
                | "#GIMMEH BOLD"
                | "#MAEK LIST"
                | "#GIMMEH ITEM"
                | "#NEWLINE"
                | "#GIMMEH LINX"
                | "#GIMMEH VAR"
                | "#IHAZ" 
                | "#ITIZ"
                | "#MKAY"
                | "#OIC"
        )
    }
}

impl Compiler for LolCompiler {

    // Compile the source code into an intermediate representation or target code
    fn compile(&mut self, _source: &str) {

    }

    // Get the next token from the source code
    fn next_token(&mut self) -> String {
        self.lexeme.clear();
        // Skip whitespace characters
        while self.position < self.source.len()
            && self.source[self.position].is_whitespace()
        {
            self.position += 1;
        }

        // If there are no characters left, return an empty string
        if self.position >= self.source.len() {
            return String::new();
        }

        // Get the first character of the next token
        let c = self.get_char();

        // If the token begins with #, it is a LOLCODE annotation
        if c == '#' {
            self.add_char(c);

            // Read the first word of the annotation
            while self.position < self.source.len()
                && !self.source[self.position].is_whitespace()
            {
                let next = self.get_char();
                self.add_char(next);
            }

            // Some annotations contain two words
            let first_part = self.lexeme.to_uppercase();

            if first_part == "#MAEK" || first_part == "#GIMMEH" {
                // Skip spaces between the two annotation words
                while self.position < self.source.len()
                    && self.source[self.position].is_whitespace()
                {
                    self.position += 1;
                }

                // Add one space between the two words
                self.add_char(' ');

                // Read the second word
                while self.position < self.source.len()
                    && !self.source[self.position].is_whitespace()
                {
                    let next = self.get_char();
                    self.add_char(next);
                }
            }

            // Make sure the annotation is legal
            if !self.lookup(&self.lexeme) {
                println!("Lexical Error: invalid token {}", self.lexeme);
                std::process::exit(1);
            }
        } else {
            // Normal text: read until whitespace or another annotation begins
            self.add_char(c);

            while self.position < self.source.len()
                && !self.source[self.position].is_whitespace()
                && self.source[self.position] != '#'
            {
                let next = self.get_char();
                self.add_char(next);
            }
        }

        // Save the token so the parser can use it later
        self.current_token = self.lexeme.clone();

        // Return the completed token
        self.lexeme.clone()
    }

    // Parser will be implemented in Task 2
    fn parse(&mut self) {
    }

    // Return the token currently being processed
    fn current_token(&self) -> String {
        self.current_token.clone()
    }

    // Change the current token
    fn set_current_token(&mut self, token: String) {
        self.current_token = token;
    }
}





fn main() {
    let test_souce = "#HAI #HEY #KBYE";

    let mut compiler = LolCompiler {
        source: test_souce.chars().collect(),
        position: 0,
        lexeme: String::new(),
        current_token: String::new(),
    };

    println! ("Next token: {}", compiler.next_token());
    println! ("Next token: {}", compiler.next_token());


}
