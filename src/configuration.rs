use core::panic;
use std::env;
use std::collections::HashSet;
use std::path::PathBuf;
use std::str::FromStr;

const DELIMITER: char = ',';

pub fn get_valid_user_form_env() -> HashSet<u64> {
    HashSet::from_iter(
        get_unique_words_from_env("VALID_USERS")
            .iter()
            .map(|number| number.parse::<u64>()
                .expect("Word for valid user should be number of user id")
            )
    )
}


pub fn get_tapes_paths() -> HashSet<PathBuf> {
    HashSet::from_iter(
        get_unique_words_from_env("TAPE_PATHS")
            .iter()
            .map(|path| PathBuf::from_str(path.as_str())
                .expect(format!("In tape paths is not valid path: {}", path).as_str()))
    )
}


fn get_unique_words_from_env(name_variable: &str) -> HashSet<String> {
    let variable = env::var(name_variable)
        .expect(format!("Environment variable {} not found", name_variable).as_str());

    if variable.is_empty() {
        panic!("Variable {} should not be empty", name_variable);
    }

    let data: HashSet<String> = variable
        .split(DELIMITER)
        .filter(|var| !var.is_empty())
        .map(|var| var.to_string()).collect();
    data
}
