use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\r\n\f]+")]
pub enum Token {
    #[regex(r"solid [_a-zA-Z0-9]*")]
    Solid,
    #[token("facet")]
    Facet,
    #[token("normal")]
    Normal,
    #[token("outer")]
    Outer,
    #[token("loop")]
    Loop,
    #[token("vertex")]
    Vertex,
    #[token("endloop")]
    EndLoop,
    #[token("endfacet")]
    EndFacet,
    #[regex(r"endsolid")]
    EndSolid,
    #[regex(r"[-+]?[0-9]*\.?[0-9]+([eE][-+]?[0-9]+)?", |lex| lex.slice().parse::<f32>().unwrap())]
    Float(f32),
}
