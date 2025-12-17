use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords existentes
    #[token("fn")] Function,
    #[token("val")] Value,
    #[token("var")] Variable, 
    #[token("mold")] Mold,
    #[token("new")] New,
    #[token("print")] Print,
    #[token("input")] Input, 
    #[token("if")] If,
    #[token("else")] Else,
    #[token("while")] While, 
    #[token("match")] Match, 
    #[token("use")] Use, 

    // --- NUEVO: Para Arrays y For Loops ---
    #[token("for")] For,
    #[token("in")] In,
    #[token("[")] LBracket,
    #[token("]")] RBracket,
    // -------------------------------------

    #[token("off")] LitOff,
    #[token("on")] LitOn,
    #[token("loading")] LitLoading,
    #[token("error")] LitError,
    #[token("unknown")] LitUnknown,

    #[token("vbit")] TypeVBit,
    #[token("int")] TypeInt, 
    
    #[regex("-?[0-9]+", |lex| lex.slice().parse().ok())] NumberLiteral(i32), 

    #[token(":=")] AssignPascal,
    #[token("=")]  AssignC,
    #[token("=>")] Arrow,

    #[token(":")] Colon,
    #[token(";")] Semicolon,
    #[token(".")] Dot,
    #[token(",")] Comma,
    #[token("{")] LBrace,
    #[token("}")] RBrace,
    #[token("(")] LParen,
    #[token(")")] RParen,
    #[token("==")] Equals,
    #[token("!=")] NotEquals, 
    #[token("<")] LessThan,
    #[token(">")] GreaterThan,
    
    #[token("+=")] PlusAssign, 
    #[token("-=")] MinusAssign, 
    #[token("*=")] MulAssign,   
    #[token("/=")] DivAssign,   
    #[token("+")] Plus,
    #[token("-")] Minus,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())] Identifier(String),
    
    #[regex(r#""(?:[^"\\]|\\.)*""#, |lex| {
        let slice = lex.slice();
        let inner = &slice[1..slice.len()-1];
        inner.replace(r#"\""#, "\"")
             .replace(r#"\\"#, "\\")
             .replace(r#"\n"#, "\n")  // <--- ¡MAGIA! Convierte \n en salto real
             .replace(r#"\t"#, "\t")  // <--- Convierte \t en tabulacion
    })] 
    StringLiteral(String),

    #[regex(r"[ \t\n\f\r]+", logos::skip)]
    #[regex(r"//.*", logos::skip)] 
    Error,
}