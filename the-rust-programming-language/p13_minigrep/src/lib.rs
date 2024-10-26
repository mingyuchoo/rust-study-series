use std::env; // 해당 모듈 경로를 현재 범위 안으로 가져오기
use std::error::Error; // 해당 모듈 경로를 현재 범위 안으로 가져오기
use std::fs::File; // 해당 모듈 경로를 현재 범위 안으로 가져오기
use std::io::Read; // 해당 모듈 경로를 현재 범위 안으로 가져오기

#[cfg(test)]
mod tests
{
    // `tests` 모듈을 선언하기
    use super::*; // 상대경로 `super`로 상위 모듈 경로를 현재 범위 안으로 가져오기

    #[test]
    fn one_result()
    {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive()
    {
        let query = "rUsT";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";
        assert_eq!(vec!["Rust:", "Trust me."],
                   search_case_insensitive(query, contents));
    }
}

pub struct Config
{
    pub query:          String,
    pub filename:       String,
    pub case_sensitive: bool,
}

impl Config
{
    pub fn new(args: &[String]) -> Result<Config, &'static str>
    {
        if args.len() < 3 {
            return Err("필요한 인수가 지정되지 않았습니다.");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { query,
                    filename,
                    case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>
{
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &str,
                  contents: &'a str)
                  -> Vec<&'a str>
{
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str,
                                   contents: &'a str)
                                   -> Vec<&'a str>
{
    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase()
               .contains(&query)
        {
            results.push(line);
        }
    }

    results
}
