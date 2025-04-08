use crate::common::*;

#[doc = "SearchDictionary struct to hold dictionary name and path"]
#[derive(Debug, Serialize, Setters, Getters, Deserialize, new)]
#[getset(get = "pub", set = "pub")]
pub struct SearchDictionary {
    pub dictionary_name: String,
    pub dictionary_path: String,
    pub result_name: String
}
