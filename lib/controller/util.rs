const KEY_PREFIX: &str = "exapi";
const KEY_DELIM: &str = "::";

pub const GROUP_COLLECTIONS: &str = "colls";
pub const GROUP_OBJECTS: &str = "objs";

pub fn build_key(parts: &[&str]) -> String {
    let size = parts.iter().map(|p| p.len()).sum::<usize>()
        + KEY_PREFIX.len()
        + parts.len() * KEY_DELIM.len();

    let mut res = String::with_capacity(size);

    res.push_str(KEY_PREFIX);
    for part in parts {
        res.push_str(KEY_DELIM);
        res.push_str(part);
    }

    res
}
