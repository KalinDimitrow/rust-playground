use std::error::Error;
use std::fs;
use std::env;

pub struct ProgramArguments {
    pub query : String,
    pub filename : String,
    pub case_sensitive : bool,
}

impl ProgramArguments {
    pub fn new<T>(mut arguments : T) -> Result<ProgramArguments,&'static str>
    where T : Iterator<Item = String>,
    {
        arguments.next();

        let _query = match arguments.next() {
            Some(arg) => arg,
            None => return Err("Require query to search for"),
        };

        let _filename = match arguments.next() {
            Some(arg) => arg,
            None => return Err("Require file to search in"),
        };

        let _case_sensitive = env::var("sensitive").is_err();
        Ok(ProgramArguments{query: _query, filename: _filename, case_sensitive : _case_sensitive})
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
    contents
        .lines()
        .filter(|line|{line.contains(&query)})
        .collect()
}


pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query_lower = query.to_lowercase();
    contents
        .lines()
        .filter(|line|{line.contains(&query_lower)})
        .collect()
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
        let program_name : String = String::from("program name");
        let query : String = String::from("query");
        let filename : String = String::from("file name");
        let arguments = vec![program_name, query.clone(), filename.clone()];
        let _data = ProgramArguments::new(arguments.into_iter())?;
        assert_eq!(_data.query, query);
        assert_eq!(_data.filename, filename);
        Ok(())
    }

    #[test]
    fn program_parameters_2() ->Result<(), Box<dyn Error>> {
        let program_name : String = String::from("program name");
        let query : String = String::from("query");
        let arguments = vec![program_name, query];
        let _data = ProgramArguments::new(arguments.into_iter());
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