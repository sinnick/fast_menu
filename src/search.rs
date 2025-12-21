// Search utilities module
// Currently the search logic is embedded in modules
// This file can be extended for more advanced search features

pub fn normalize_query(query: &str) -> String {
    query.trim().to_lowercase()
}
