use std::error::Error;
use std::fs;
use std::env;

pub struct ProgramArguments {
    pub query : String,
    pub filename : String,
    pub case_sensitive : bool,
}

impl ProgramArguments {
    pub fn new(arguments : &[&str]) -> Result<ProgramArguments,&'static str> {
        if arguments.len() != 3 {
            return Err("Wrong number of arguments, two are required, first query abd second file name")
        }
        let case_sensitive = env::var("sensitive").is_err();
        Ok(ProgramArguments{query: String::from(arguments[1]), filename:String::from(arguments[2]), case_sensitive : case_sensitive})
    }
}


pub fn run(arguments : ProgramArguments) ->Result<(),  Box<dyn Error>>{
    let content = fs::read_to_string(&arguments.filename)?;
    let result = if arguments.case_sensitive {
        search(&arguments.query, &content)
    } else {
        search_case_insensitive(&arguments.query, &content)
    };

    for line in result {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}


pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn program_parameters_1() ->Result<(), Box<dyn Error>> {
        const QUERY : &str = "query";
        const FILENAME : &str = "file name";
        let arguments = vec!["program name",QUERY,FILENAME];
        let _data = ProgramArguments::new(&arguments[..])?;
        assert_eq!(_data.query, QUERY);
        assert_eq!(_data.filename, FILENAME);
        Ok(())
    }

    #[test]
    fn program_parameters_2() ->Result<(), Box<dyn Error>> {
        const QUERY : &str = "query";
        const FILENAME : &str = "file name";
        let arguments = vec!["program name",QUERY,FILENAME];
        let _data = ProgramArguments::new(&arguments[..])?;
        assert_eq!(_data.query, QUERY);
        assert_eq!(_data.filename, FILENAME);

        Ok(())
    }

    #[test]
    fn program_parameters_3() ->Result<(), Box<dyn Error>> {
        const QUERY : &str = "query";
        let arguments = vec!["program name",QUERY];
        let _data = ProgramArguments::new(&arguments[..]);
        assert!(_data.is_err());

        Ok(())
    }

    #[test]
    fn search_case_insensitive_test() {
        let query = "Duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search_case_insensitive(query, contents));
    }
}