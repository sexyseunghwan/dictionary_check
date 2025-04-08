use crate::common::*;

#[doc = "Function to globally initialize the 'DICTIONARY_LIST_PATH' variable"]
pub static DICTIONARY_LIST_PATH: once_lazy<String> = once_lazy::new(|| {
    env::var("DICTIONARY_LIST_PATH")
        .expect("[ENV file read Error] 'DICTIONARY_LIST_PATH' must be set")
});

#[doc = "Function to globally initialize the 'RESULT_PATH' variable"]
pub static RESULT_PATH: once_lazy<String> = once_lazy::new(|| {
    env::var("RESULT_PATH").expect("[ENV file read Error] 'RESULT_PATH' must be set")
});