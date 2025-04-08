use crate::common::*;

use crate::models::search_dictionary::*;

#[doc = "SearchDictionary struct to hold dictionary name and path"]
#[derive(Debug, Serialize, Deserialize, Setters, new)]
#[getset(get = "pub", set = "pub")]
pub struct SearchDictionaries {
    pub dictionary: Vec<SearchDictionary>,
}
