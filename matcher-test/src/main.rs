use clap::Parser;
use matcher_derive_impl::matcher::{BaseMatcher, Matcher};
mod cli;
mod matcher;

fn match_toto(path: String, toto: matcher::Toto) -> u16 {
    let data = std::fs::read_to_string(path).expect("Can't read config file");
    let toto_matchers =
        serde_yaml::from_str::<Vec<BaseMatcher<<matcher::Toto as Matcher>::AllMatcher>>>(&data)
            .expect("Can't parse config file");
    let mut matches = 0;

    toto_matchers.iter().for_each(|toto_matcher| {
        let res = toto.match_all(toto_matcher.clone());
        if res {
            matches += 1;
        }
    });

    matches
}

pub(crate) fn get_fake_event() -> matcher::Toto {
    let mut map = std::collections::HashMap::new();

    map.insert("tata".to_string(), "titi".to_string());

    matcher::Toto {
        id: "toto".to_string(),
        name: "toto".to_string(),
        toto: map,
    }
}

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();

    println!("Read {}", args.path);

    let res = match_toto(args.path.to_string(), get_fake_event());

    println!("{:?}\n{:?}", args.path, res);
}

#[cfg(test)]
mod tests {
    use crate::{get_fake_event, match_toto};
    fn test_file(path: String, waited_res: u16) {
        let res = match_toto(format!("./tests/{path}"), get_fake_event());

        assert_eq!(res, waited_res, "{} match nuber not valid", path,);
    }

    #[test]
    fn success() {
        test_file("config.yml".to_owned(), 1);
        test_file("partial-config.yml".to_owned(), 1);
    }

    #[test]
    fn fail() {
        test_file("bad-config.yml".to_owned(), 0);
        test_file("bad-partial-config.yml".to_owned(), 0);
    }
}
