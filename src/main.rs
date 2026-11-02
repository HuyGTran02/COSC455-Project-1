
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
pub trait LexicalAnalyzer{
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
    fn get_char(&mut self) -> char{
        if self.position < self.source.len(){
            let c: char = self.source[self.position];
            self.position += 1;
            c
        } else {
            '\0'
        }
    }

    // Add a character to the current lexeme
    fn add_char(&mut self, c: char){
        self.lexeme.push(c);
    }

    // Check if the lexeme is a valid LOLCODE annotation
    fn lookup(&self, s: &str) -> bool{
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

impl Compiler for LolCompiler{
    // Compile the source code into an intermediate representation or target code
    fn compile(&mut self, _source: &str){
    }
    // Get the next token from the source code
    fn next_token(&mut self) -> String{
        self.lexeme.clear();
        // Skip whitespace characters
        while self.position < self.source.len() && self.source[self.position].is_whitespace(){
            self.position += 1;
        }

        // If there are no characters left, return an empty string
        if self.position >= self.source.len(){
            return String::new();
        }

        // Get the first character of the next token
        let c = self.get_char();

        // If the token begins with #, it is a LOLCODE annotation
        if c == '#' {
            self.add_char(c);

            // Read the first word of the annotation
            while self.position < self.source.len() && !self.source[self.position].is_whitespace(){
                let next = self.get_char();
                self.add_char(next);
            }

            // Some annotations contain two words
            let first_part = self.lexeme.to_uppercase();

            if first_part == "#MAEK" || first_part == "#GIMMEH"{
                // Skip spaces between the two annotation words
                while self.position < self.source.len() && self.source[self.position].is_whitespace(){
                    self.position += 1;
                }
                // Add one space between the two words
                self.add_char(' ');

                // Read the second word
                while self.position < self.source.len()&& !self.source[self.position].is_whitespace(){
                    let next = self.get_char();
                    self.add_char(next);
                }
            }

            // Make sure the annotation is legal
            if !self.lookup(&self.lexeme){
                println!("Lexical Error: invalid token {}", self.lexeme);
                std::process::exit(1);
            }
        } else{
            // Normal text: read until whitespace or another annotation begins
            self.add_char(c);

            while self.position < self.source.len() && !self.source[self.position].is_whitespace() && self.source[self.position] != '#'{
                let next = self.get_char();
                self.add_char(next);
            }
        }

        // Save the token so the parser can use it later
        self.current_token = self.lexeme.clone();

        // Return the completed token
        self.lexeme.clone()
    }


    fn parse(&mut self){
        self.position = 0;
        self.advance(); 
        self.parse_lolcode();
    }

    // Return the token currently being processed
    fn current_token(&self) -> String{
        self.current_token.clone()
    }

    // Change the current token
    fn set_current_token(&mut self, token: String){
        self.current_token = token;
    }
}
//First Option
pub trait SyntaxAnalyzer {
fn parse_lolcode(&mut self);
fn parse_head(&mut self);
fn parse_title(&mut self);
fn parse_comment(&mut self);
fn parse_body(&mut self);
fn parse_paragraph(&mut self);
fn parse_inner_paragraph(&mut self);
fn parse_inner_text(&mut self);
fn parse_variable_define(&mut self);
fn parse_variable_use(&mut self);
fn parse_bold(&mut self);
fn parse_list(&mut self);
fn parse_list_items(&mut self);
fn parse_inner_list(&mut self);
fn parse_link(&mut self);
fn parse_newline(&mut self);
fn parse_text(&mut self);

}
// Implement the SyntaxAnalyzer trait for the LolCompiler struct
impl LolCompiler{
    fn advance(&mut self){
        let token = self.next_token();
        self.set_current_token(token);
    }
    // Check if the current token matches the expected token, and advance to the next token if it does
    fn expect(&mut self, expected_token: &str){
        if !self.current_token().eq_ignore_ascii_case(expected_token){
            println!("Syntax Error: expected {}, found {}", expected_token, self.current_token());
            std::process::exit(1);
        }
        self.advance();
    }
}
// Implement the SyntaxAnalyzer trait for the LolCompiler struct
impl SyntaxAnalyzer for LolCompiler{
    fn parse_lolcode(&mut self){
        self.expect("#HAI");   
        while self.current_token().eq_ignore_ascii_case("#OBTW"){
            self.parse_comment();
        }
        if self.current_token().eq_ignore_ascii_case("#MAEK HEAD"){
            self.parse_head();
        }
        self.expect("#KBYE");
    }

    fn parse_head(&mut self){}
    fn parse_title(&mut self){}
    fn parse_comment(&mut self){}
    fn parse_body(&mut self){}
    fn parse_paragraph(&mut self){}
    fn parse_inner_paragraph(&mut self){}
    fn parse_inner_text(&mut self){}
    fn parse_variable_define(&mut self){}
    fn parse_variable_use(&mut self){}
    fn parse_bold(&mut self){}
    fn parse_list(&mut self){}
    fn parse_list_items(&mut self){}
    fn parse_inner_list(&mut self){}
    fn parse_link(&mut self){}
    fn parse_newline(&mut self){}
    fn parse_text(&mut self){}



}

fn main(){
    let test_souce: &str = "#HAI #KBYE";

    let mut compiler = LolCompiler {
        source: test_souce.chars().collect(),
        position: 0,
        lexeme: String::new(),
        current_token: String::new(),
    };
    compiler.parse();
    println!("Syntax is valid!");

}
