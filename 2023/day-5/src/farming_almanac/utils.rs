pub fn extract_map_from_almanac(almanac: &str, map_id_string: &str) -> Vec<String> {
    let map_iter = almanac
        .split('\n')
        .skip_while(|line| !line.to_lowercase().contains(map_id_string));

    map_iter
        .take_while(|element| !element.is_empty())
        .filter(|line| !line.to_lowercase().contains(map_id_string))
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
}
