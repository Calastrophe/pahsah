use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    // FILE_DESCRIPTION,
    // FILE_NAME,
    // FILE_SCHEMA
    #[token("HEADER;")]
    Header,
    #[token("DATA;")]
    Data,
    #[token("ENDSEC;")]
    EndSection,
    // INSTANCE
}
