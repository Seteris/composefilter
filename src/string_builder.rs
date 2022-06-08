use regex::Captures;

pub fn get_info_string(matches: Captures) -> (String, i32, i32) {
    let mut info_string = String::from("Step ");
    info_string.push_str(&matches[1]);
    info_string.push_str("/");
    info_string.push_str(&matches[2]);
    info_string.push_str(" ");
    (info_string, (&matches[1]).parse().unwrap(), (&matches[2]).parse().unwrap())
}

pub fn get_build_string(matches: Captures, build_count: &mut i32) -> String {
    let mut build_string = String::from("Build ");
    *build_count += 1;
    build_string.push_str(&*build_count.to_string());
    build_string.push_str("(");
    build_string.push_str(&matches[1]);
    build_string.push_str(") ");
    build_string
}
