use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[token("FILE_DESCRIPTION(.*);")]
    Description,
    #[token("FILE_NAME(.*);")]
    Name,
    #[token("FILE_SCHEMA(.*)")]
    Schema,
    #[token("ISO-10303-21;")]
    StartStandard,
    #[token("HEADER;")]
    Header,
    #[token("ENDSEC;")]
    EndSection,
    #[token("DATA;")]
    Data,
    // Identifier
    // Entity
    // Argument
    #[token("END-ISO-10303-21")]
    EndStandard,
}
