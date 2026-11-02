use std::fs;
use std::env;
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
        scopes: Vec<Vec<(String, String)>>,
        html_output: String,
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
    // Advance to the next token and update the current token
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
    // Enter a new scope by pushing a new vector onto the scopes stack
    fn enter_scope(&mut self){
        self.scopes.push(Vec::new());
    }
    // Exit the current scope by popping the top vector off the scopes stack
    fn exit_scope(&mut self){
        if !self.scopes.is_empty(){
            self.scopes.pop();
        }

    }
    // Save a variable in the current scope by adding it to the top vector on the scopes stack
    fn save_variable(&mut self, name: String, value: String){
        if let Some(current_scope) = self.scopes.last_mut(){
            current_scope.push((name, value));
        }
    }
    // Find a variable in the current and outer scopes by searching the scopes stack from top to bottom
    fn find_variable(&self, name: &str) -> Option<String>{
        for scope in self.scopes.iter().rev(){
            for (var_name, var_value) in scope.iter().rev() {
                if var_name == name{
                    return Some(var_value.clone());
                }
            }
        }
        None
    }



}
// Implement the SyntaxAnalyzer trait for the LolCompiler struct
impl SyntaxAnalyzer for LolCompiler{
    // Parse the LOLCODE program, which must start with #HAI and end with #KBYE
    fn parse_lolcode(&mut self){
        self.expect("#HAI");
        self.html_output.push_str("<html>\n");
        self.enter_scope();

        if self.current_token().eq_ignore_ascii_case("#IHAZ"){
            self.parse_variable_define();
        }  
        while self.current_token().eq_ignore_ascii_case("#OBTW"){
            self.parse_comment();
        }
        if self.current_token().eq_ignore_ascii_case("#MAEK HEAD"){
            self.parse_head();
        }
        self.parse_body();
        self.expect("#KBYE");
        self.html_output.push_str("</html>\n");
        self.exit_scope();
    }
    // Parse the head section of the LOLCODE program, which may contain a title
    fn parse_head(&mut self){
        self.expect("#MAEK HEAD");
        self.html_output.push_str("<head>\n");
        if self.current_token().eq_ignore_ascii_case("#GIMMEH TITLE"){
            self.parse_title();
        }
        self.expect("#MKAY");
        self.html_output.push_str("</head>\n")
    }
    // Parse the title section of the LOLCODE program, which is optional and must be followed by #OIC
    fn parse_title(&mut self){
        self.expect("#GIMMEH TITLE");
        self.html_output.push_str("<title>");
        while !self.current_token().eq_ignore_ascii_case("#OIC"){
            if self.current_token().is_empty(){
                println!("Syntax Error: expected #OIC, found end of input");
                std::process::exit(1);
            }
            if self.current_token().starts_with('#'){
                println!("Syntax Error: expected #OIC, found {}", self.current_token());
                std::process::exit(1);
            }
            self.html_output.push_str(&self.current_token());
            self.html_output.push(' ');
            self.advance();
        }
        self.expect("#OIC");
        self.html_output.push_str("</title>\n");
    }
    // Parse the comment section of the LOLCODE program, which is optional and must be followed by #TLDR
    fn parse_comment(&mut self){
        self.expect("#OBTW");
        self.html_output.push_str("<!-- ");
        while !self.current_token().eq_ignore_ascii_case("#TLDR"){
            if self.current_token().is_empty(){
                println!("Syntax Error: expected #TLDR, found end of input");
                std::process::exit(1);
            }
            self.html_output.push_str(&self.current_token());
            self.html_output.push(' ');
            self.advance();
        }
        self.expect("#TLDR");
        self.html_output.push_str(" -->\n")
    }
    // Parse the body section of the LOLCODE program, which may contain multiple paragraphs
    fn parse_body(&mut self){
        while !self.current_token().eq_ignore_ascii_case("#KBYE"){
            if self.current_token().is_empty(){
                println!("Syntax Error: expected #KBYE, found end of input");
                std::process::exit(1);
            } else if !self.current_token().starts_with('#'){
                self.parse_text();
            } else if self.current_token().eq_ignore_ascii_case("#OBTW"){
                self.parse_comment();
            } else if self.current_token().eq_ignore_ascii_case("#MAEK PARAGRAF"){
                self.parse_paragraph();
            } else if self.current_token().eq_ignore_ascii_case("#GIMMEH BOLD"){
                self.parse_bold();
            } else if self.current_token().eq_ignore_ascii_case("#NEWLINE"){
                self.parse_newline();
            } else if self.current_token().eq_ignore_ascii_case("#GIMMEH LINX"){
                self.parse_link();
            } else if self.current_token().eq_ignore_ascii_case("#MAEK LIST"){
                self.parse_list();
            } else if self.current_token().eq_ignore_ascii_case("#GIMMEH VAR"){
                self.parse_variable_use();
            } else{
                println!("Syntax Error: unexpected token {} for body", self.current_token());
                std::process::exit(1);
            }
        }
    }
    // Parse a paragraph section of the LOLCODE program, which must be followed by #MKAY
    fn parse_paragraph(&mut self){
        self.expect("#MAEK PARAGRAF");
        self.html_output.push_str("<p>\n");
        self.enter_scope();
        if self.current_token().eq_ignore_ascii_case("#IHAZ"){
            self.parse_variable_define();
        }

        while !self.current_token().eq_ignore_ascii_case("#MKAY"){
            if self.current_token().is_empty(){
                println!("Syntax Error: expected #MKAY, found end of input");
                std::process::exit(1);
            }
            self.parse_inner_paragraph();
        }

        self.html_output.push_str("</p>\n");
        self.expect("#MKAY");
        self.exit_scope();
    }
    // Parse the inner content of a paragraph, which may contain text, bold, newline, link, list, or variable use
    fn parse_inner_paragraph(&mut self){
        if !self.current_token().starts_with('#'){
            self.parse_text();

        } else if self.current_token().eq_ignore_ascii_case("#GIMMEH BOLD"){
            self.parse_bold();
        } else if self.current_token().eq_ignore_ascii_case("#NEWLINE"){
            self.parse_newline();
        } else if self.current_token().eq_ignore_ascii_case("#GIMMEH LINX"){ 
            self.parse_link();
        }else if self.current_token().eq_ignore_ascii_case("#MAEK LIST"){
            self.parse_list();
        }else if self.current_token().eq_ignore_ascii_case("#GIMMEH VAR"){
            self.parse_variable_use();
        }else{
            println!("Syntax Error: unexpected token {} for inner paragraph", self.current_token());
            std::process::exit(1);
        }
    }
    // Parse the inner content of a list item, which may contain text, bold, or variable use
    fn parse_inner_text(&mut self){
        if !self.current_token().starts_with('#'){
            self.parse_text();
        } else if self.current_token().eq_ignore_ascii_case("#GIMMEH VAR"){
            self.parse_variable_use();
        } else{
            println!("Syntax Error: unexpected token {} for inner text", self.current_token());
            std::process::exit(1);
        }
    }

    // Parse a variable definition section of the LOLCODE program, which must be followed by #ITIZ
    fn parse_variable_define(&mut self){
        self.expect("#IHAZ");
        if self.current_token().is_empty() || self.current_token().starts_with('#'){
            println!("Syntax Error: expected variable name, found end of input");
            std::process::exit(1);
        }
        let variable_name = self.current_token().clone();
        self.advance();
        self.expect("#ITIZ");

        let mut variable_value = String::new();
        while !self.current_token().eq_ignore_ascii_case("#MKAY"){
            if self.current_token().is_empty(){
                println!("Syntax Error: expected #MKAY, found end of input");
                std::process::exit(1);
            }
            if self.current_token().starts_with('#'){
                println!("Syntax Error: expected #MKAY, found {}", self.current_token());
                std::process::exit(1);
            }
            if !variable_value.is_empty(){
                variable_value.push(' ');
            }
            variable_value.push_str(&self.current_token());
            self.advance();
        }
        self.save_variable(variable_name, variable_value);
        self.expect("#MKAY");
    }
    // Parse a variable use section of the LOLCODE program, which must be followed by #OIC
    fn parse_variable_use(&mut self){
        self.expect("#GIMMEH VAR");
        if self.current_token().is_empty() || self.current_token().starts_with('#'){
            println!("Syntax Error: expected variable name, found end of input");
            std::process::exit(1);
        }
        let variable_name = self.current_token();

        let variable_value = match self.find_variable(&variable_name){
            Some(value) => value, None => {
                println!("Static Semantic Error: variable {} not define", variable_name);
                std::process::exit(1);
            }
        };
        self.advance();
        self.html_output.push_str(&variable_value);
        self.html_output.push(' ');
        self.expect("#OIC");
    }
    // Parse a bold section of the LOLCODE program, which must be followed by #OIC
    fn parse_bold(&mut self){
        self.expect("#GIMMEH BOLD");   
        self.html_output.push_str("<b>");
        while !self.current_token().eq_ignore_ascii_case("#OIC"){
            if self.current_token().is_empty(){
                println!("Syntax Error: expected #OIC, found end of input");
                std::process::exit(1);
            }
            self.parse_inner_text();
        }
        self.expect("#OIC");
        self.html_output.push_str("</b>");
    }
    // Parse a list section of the LOLCODE program, which must be followed by #MKAY
    fn parse_list(&mut self){
        self.expect("#MAEK LIST");
        self.html_output.push_str("<ul>\n");
        if !self.current_token().eq_ignore_ascii_case("#GIMMEH ITEM"){
            println!("Syntax Error: expected #GIMMEH ITEM, found {}", self.current_token());
            std::process::exit(1);
        }
        while self.current_token().eq_ignore_ascii_case("#GIMMEH ITEM"){
            self.parse_list_items();
        }
        self.expect("#MKAY");
        self.html_output.push_str("</ul>\n");

    }
    // Parse a list item section of the LOLCODE program, which must be followed by #OIC
    fn parse_list_items(&mut self){
        self.expect("#GIMMEH ITEM");
        self.html_output.push_str("<li>\n");

        if self.current_token().eq_ignore_ascii_case("#IHAZ"){
            self.parse_variable_define();
        }
        while !self.current_token().eq_ignore_ascii_case("#OIC"){
            if self.current_token().is_empty(){
                println!("Syntax Error: expected #OIC, found end of input");
                std::process::exit(1);
            }
            self.parse_inner_list();
        }
        self.expect("#OIC");
        self.html_output.push_str("</li>\n");

    }
    // Parse the inner content of a list item, which may contain text, bold, or variable use
    fn parse_inner_list(&mut self){
        if !self.current_token().starts_with('#'){
            self.parse_text();
        } else if self.current_token().eq_ignore_ascii_case("#GIMMEH BOLD"){
            self.parse_bold();
        } else if self.current_token().eq_ignore_ascii_case("#GIMMEH VAR"){
            self.parse_variable_use();
        } else{
            println!("Syntax Error: unexpected token {} for inner list", self.current_token());
            std::process::exit(1);

        }
    }
    // Parse a link section of the LOLCODE program, which must be followed by #OIC
    fn parse_link(&mut self){
        self.expect("#GIMMEH LINX");
        let mut address = String::new();
        while !self.current_token().eq_ignore_ascii_case("#OIC"){
            if self.current_token().is_empty(){
                println!("Syntax Error: expected #OIC, found end of input");
                std::process::exit(1);
            }
            if self.current_token().starts_with('#'){
                println!("Syntax Error: expected #OIC, found {}", self.current_token());
                std::process::exit(1);
            }
            if !address.is_empty(){
                address.push(' ');

            }
            address.push_str(&self.current_token());
            self.advance();
        }
        self.expect("#OIC");
        self.html_output.push_str("<a href=\"");
        self.html_output.push_str(&address);
        self.html_output.push_str("\">");
        self.html_output.push_str(&address);
        self.html_output.push_str("</a>");
    }

    // Parse a newline section of the LOLCODE program, which must be followed by #MKAY
    fn parse_newline(&mut self){
        self.expect("#NEWLINE");
        self.html_output.push_str("<br>\n");

    }
    // Parse normal text until the next annotation or end of input
    fn parse_text(&mut self){
        while !self.current_token().is_empty() && !self.current_token().starts_with('#'){
            self.html_output.push_str(&self.current_token());
            self.html_output.push(' ');
            self.advance();
        }
    }



}

fn main(){

let test_source: &str =         
    "#HAI
    #OBTW This is my test page #TLDR
    #MAEK HEAD
    #GIMMEH TITLE My LOLCODE Page #OIC
    #MKAY
    #MAEK PARAGRAF
    #IHAZ lastname #ITIZ Tran #MKAY
    Hello #GIMMEH BOLD everyone #OIC
    #NEWLINE
    Visit #GIMMEH LINX www.towson.edu #OIC
    #NEWLINE
    #MAEK LIST
    #GIMMEH ITEM Huy #GIMMEH VAR lastname #OIC #OIC
    #GIMMEH ITEM Bo #GIMMEH VAR lastname #OIC #OIC
    #MKAY
    #MKAY
    #KBYE";
     
    let mut compiler = LolCompiler {
        source: test_source.chars().collect(),
        position: 0,
        lexeme: String::new(),
        current_token: String::new(),
        scopes: Vec::new(),
        html_output: String::new(),
    };
    compiler.parse();
    println!("{}", compiler.html_output);
        println!("Syntax is valid!");

}
