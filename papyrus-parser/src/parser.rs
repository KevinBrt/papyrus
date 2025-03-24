use std::fs;

/**
 * Parser struct
 * @param source: String
 */
pub struct Parser {
    source: String,
}

impl Parser {
    pub fn new(source: String) -> Self {
        Self { source }
    }

    /**
    * Take a XML source file and parse it
    * @param source: &str
    * @return: ()
    */
    pub fn parse(&self) {
        println!("Parse fn : {} ", self.source);
        // TODO
    }

    /**
    * Convert the parsed XML to a Taffy layout
    * @param source: &str
    * @return: ()
    */
    pub fn to_taffy(&self) {
        println!("to_taffy fn : {} ", self.source);
        // TODO
    }
}
