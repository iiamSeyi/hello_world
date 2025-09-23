pub struct Compiler {
    pub current_token: String,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            current_token: String::new(),
        }
    }
}

pub struct LexicalAnalyzer {
    tokens: Vec<String>,
    pub articles: Vec<String>,
    pub verbs: Vec<String>,
    pub nouns: Vec<String>,
    pub adjectives:Vec<String>,
    pub adverbs:Vec<String>
}

impl LexicalAnalyzer {
    pub fn new(input: &str) -> Self {
        let mut tokens: Vec<String> = input
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        tokens.reverse();

        Self {
            tokens,
            articles: vec!["a".into(), "teh".into()],
            verbs: vec!["lovez".into(), "hatez".into(), "ates".into()],
            nouns: vec!["dawg".into(), "kat".into(), "rat".into()],
            adjectives: vec!["fat".into(), "hungry".into(), "happy".into(), "mean".into()],
            adverbs: vec![ "accidently".into(), "quickly".into(), "secretly".into()]
            
        }
    }

    pub fn lookup(&mut self, word: &str) -> bool {
        self.articles.iter().any(|a| a == word)
            || self.nouns.iter().any(|n| n == word)
            || self.verbs.iter().any(|v| v == word)
            || self.adjectives.iter().any(|adj| adj == word)
            || self.adverbs.iter().any(|adv| adv == word)
    }

    pub fn start(&mut self) -> String {
        let candidate_token = self.tokens.pop().unwrap_or_default();

        if self.lookup(&candidate_token) {
            candidate_token
        } else if !candidate_token.is_empty() {
            eprintln!(
                "A lexical error was encountered. '{}' is not a recognized token.",
                candidate_token
            );
            std::process::exit(1);
        } else {
            eprintln!("A user error was encountered. The provided sentence is empty.");
            std::process::exit(1);
        }
    }

    pub fn next(&mut self) -> String {
        let candidate_token = self.tokens.pop().unwrap_or_default();

        if self.lookup(&candidate_token) {
            candidate_token
        } else if self.tokens.is_empty() {
            "".to_string()
        } else {
            eprintln!(
                "A lexical error was encountered. '{}' is not a recognized token.",
                candidate_token
            );
            std::process::exit(1);
        }
    }

    pub fn is_a_article(&self, word: &str) -> bool {
        self.articles.iter().any(|a| a == word)
    }

    pub fn is_a_noun(&self, word: &str) -> bool {
        self.nouns.iter().any(|n| n == word)
    }

    pub fn is_a_verb(&self, word: &str) -> bool {
        self.verbs.iter().any(|v| v == word)
    }
    pub fn is_an_adjective(&self, word: &str) -> bool {
        self.adjectives.iter().any(|adj| adj == word)
    }
    pub fn is_an_adverb(&self, word: &str) -> bool {
        self.adverbs.iter().any(|adv| adv == word)
    }
    
}

pub struct SyntaxAnalyzer<'a> {
    lexer: &'a mut LexicalAnalyzer,
    compiler: &'a mut Compiler,
}

impl<'a> SyntaxAnalyzer<'a> {
    pub fn new(lexer: &'a mut LexicalAnalyzer, compiler: &'a mut Compiler) -> Self {
        Self { lexer, compiler }
    }

    pub fn next_token(&mut self) {
        let tok = self.lexer.next();
        self.compiler.current_token = tok;
    }

    pub fn article(&mut self){
        // TO DO
        if self.lexer.is_a_article(&self.compiler.current_token) {
            self.next_token();//advance to next token
        } else {
            //print error message
            eprintln!(
                "A syntax error was encountered. '{}' was found when an article (a, teh) was expected.",
                self.compiler.current_token
            );
            //exit
            std::process::exit(1);
        }
    }

    pub fn noun(&mut self){
        // TO DO
        if self.lexer.is_a_noun(&self.compiler.current_token){
            self.next_token();
            
        } else {
            //print error message
            eprintln!(
                "A syntax error was encountered. '{}' was found when a noun (dawg, kat, rat) was expected.",
                self.compiler.current_token
            );
            //exit
            std::process::exit(1);
        }
    }
    
    pub fn adjective(&mut self){
        if self.lexer.is_an_adjective(&self.compiler.current_token){
            self.next_token();
            
        } else {
            //print error message
            eprintln!(
                "A syntax error was encountered. '{}' was found when an adjective (fat, hungry, happy or mean) was expected.",
                self.compiler.current_token
            );
            //exit
            std::process::exit(1);
        }
    }

    pub fn noun_phrase(&mut self){
        // TO DO
        self.article();
        self.adjective();
        self.noun();
    }
    
    pub fn adverb(&mut self){
        if self.lexer.is_an_adverb(&self.compiler.current_token){
            self.next_token();
            
        } 
    }
    

    pub fn verb(&mut self){
        // TO DO
        if self.lexer.is_a_verb(&self.compiler.current_token){
            self.next_token();
            
        } else {
            //print error message
            eprintln!(
                "A syntax error was encountered. '{}' was found when a verb (lovez, hatez, ates) was expected.",
                self.compiler.current_token
            );
            //exit
            std::process::exit(1);
        }
    }
    pub fn lolspeak(&mut self) {
        // TO DO
        self.noun_phrase();
        self.adverb();
        self.verb();
        self.noun_phrase();
    }
}

fn main() {
    let test1 = "teh dawg ates a mean kat";
    

    let mut compiler = Compiler::new();
    let mut lexer = LexicalAnalyzer::new(test1);

    compiler.current_token = lexer.start();

    let mut parser = SyntaxAnalyzer::new(&mut lexer, &mut compiler);
    parser.lolspeak();

    if !lexer.tokens.is_empty() || !compiler.current_token.is_empty() {
        eprintln!("A syntax error was encountered. Additional tokens found after the sentence.");
        std::process::exit(1);
    }
    println!("The sentence '{}' follows the lolspeak grammar!", test1);
    
}
