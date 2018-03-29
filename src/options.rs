extern crate clap;

use failure::Error;
use clap::{App, Arg};

arg_enum! {
    #[derive(PartialEq, Debug, Eq, Clone, Copy)]
    pub enum Type {
        RSpec,
    }
}

impl Default for Type {
    fn default() -> Type {
        Type::RSpec
    }
}

#[derive(PartialEq, Debug, Eq, Clone, Copy)]
pub enum Action {
    List,
    Execute,
}

impl Default for Action {
    fn default() -> Action {
        Action::List
    }
}

#[derive(Debug, PartialEq)]
pub struct Options {
    pub tests_type: Type,
    pub action: Action,
    pub branch_name: Option<String>,
    pub whole_files: bool,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            tests_type: Type::default(),
            action: Action::default(),
            branch_name: None,
            whole_files: false,
        }
    }
}

fn build_clap_app<'a, 'b>() -> App<'a, 'b> {
    app_from_crate!()
            .arg(
                Arg::with_name("type")
                    .takes_value(true)
                    .value_name("TYPE")
                    .case_insensitive(true)
                    .help("The type of tests to search for")
                    .long_help("The type of tests to search for. This option is case insensitive.")
                    .possible_values(&Type::variants())
                    .default_value(&Type::variants()[0]),
            )
            .arg(
                Arg::with_name("execute")
                    .short("e")
                    .long("execute")
                    .conflicts_with("list")
                    .help("Run tests instead of just listing them."),
            )
            .arg(
                Arg::with_name("list")
                    .short("l")
                    .long("list")
                    .conflicts_with("execute")
                    .help("Only list tests to STDOUT."),
            )
            .arg(
                Arg::with_name("whole-files")
                    .short("w")
                    .long("whole-files")
                    .help("Always run the whole test files instead of trying to extract subsets of them (where supported)."),
            )
            .arg(
                Arg::with_name("branch")
                    .short("b")
                    .long("branch")
                    .help("Diff against given branch name instead of HEAD commit. Can be specified without a value to diff against upstream master.")
                    .takes_value(true)
                    .value_name("BRANCH")
                    .default_value("origin/master"),
            )
}

impl Options {
    fn from_matches(matches: clap::ArgMatches) -> Result<Options, Error> {
        Ok(Options {
            tests_type: value_t!(matches, "type", Type)?,
            action: if matches.is_present("execute") {
                Action::Execute
            } else {
                Action::List
            },
            whole_files: matches.is_present("whole-files"),
            branch_name: if matches.occurrences_of("branch") > 0 {
                // branch argument has a default value so it should always be there
                Some(String::from(matches.value_of("branch").unwrap()))
            } else {
                None
            },
            ..Default::default()
        })
    }

    pub fn from_args() -> Result<Options, Error> {
        let app = build_clap_app();
        let matches = app.get_matches();
        Self::from_matches(matches)
    }

    #[cfg(test)]
    fn from_args_with<'a>(args: &[&'a str]) -> Result<Options, Error> {
        let app = build_clap_app();
        let matches = app.get_matches_from(args);
        Self::from_matches(matches)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod type_enum {
        use super::*;

        #[test]
        fn it_has_default_matching_first_variant() {
            let variant = Type::variants()[0];
            assert_eq!(Type::default(), variant.parse().unwrap());
        }
    }

    mod options {
        use super::*;

        #[test]
        fn it_has_matching_defaults_and_blank_arguments_parsing() {
            let default = Options::default();
            let from_args = Options::from_args_with(&["x"]).unwrap();

            assert_eq!(default, from_args);
        }

        #[test]
        fn it_does_not_match_against_branch_by_default() {
            let options = Options::from_args_with(&["x"]).unwrap();
            assert_eq!(options.branch_name, None);
        }

        #[test]
        fn it_compares_against_origin_master_when_no_branch_specified() {
            let options = Options::from_args_with(&["x", "-b"]).unwrap();
            assert_eq!(options.branch_name, Some(String::from("origin/master")));
        }

        #[test]
        fn it_compares_against_specified_branch() {
            let options = Options::from_args_with(&["x", "-b", "ticket-1"]).unwrap();
            assert_eq!(options.branch_name, Some(String::from("ticket-1")));
        }
    }
}
