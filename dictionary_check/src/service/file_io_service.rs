use crate::common::*;

use crate::models::search_dictionary::*;

use crate::env_config::env_config::*;

pub trait FileIOService {
    fn process_file(&self, dictionary: &SearchDictionary) -> Result<(), anyhow::Error>;
}

#[derive(Debug, new)]
pub struct FileIOServicePub;

impl FileIOService for FileIOServicePub {
    
    #[doc = "중복된 단어를 찾는 함수"]
    fn process_file(&self, dictionary: &SearchDictionary) -> Result<(), anyhow::Error> {
        /* 1. 파일 열기(읽기) */
        let file: File = File::open(dictionary.dictionary_path())
            .map_err(|err| anyhow!("[Error][process_file()] Failed to open file: {}", err))?;
        let reader: BufReader<File> = BufReader::new(file);

        /* 2. 파일 생성&열기(쓰기) */
        let mut write_file_path: PathBuf = PathBuf::from(RESULT_PATH.to_string());
        write_file_path.push(dictionary.dictionary_name());

        let mut write_file: File = File::create(&write_file_path)
            .map_err(|err| anyhow!("[Error][process_file()] Failed to create file: {}", err))?;
        
        /* 3. 모든 줄을 벡터에 저장 */
        let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

        /* 4. HashSet 을 사용하여 중복되는 단어 검증 */
        let mut hash_set: HashSet<String> = HashSet::new();

        let mut duplicate_cnt: i32 = 0;

        /*  5. 기준줄 하나씩 돌면서 다음 줄들과 비교 */
        for i in 0..lines.len() {

            let current: &String = &lines[i];

            if let Some(left_part) = current.split("=>").next() {
                if let Some(first_token) = left_part.trim().split(",").next() {
                    if let Some(second_token) = first_token.trim().split(" ").next() {
                        let insert_result: bool = hash_set.insert(second_token.to_string());

                        if !insert_result {
                            write_file.write_all(format!("Duplicate occurrence in line {} => duplicated: {}, full text: {}\n", i+1, second_token, current).as_bytes())
                                .map_err(|err| {
                                    anyhow!("[Error][FileIOService -> process_file] Failed to write to file: {}", err)
                                })?;
                            duplicate_cnt += 1;
                        }
                    }
                }
            }
        }

        /* 중복되는 단어가 없는 경우 */
        if duplicate_cnt == 0 {
            write_file.write_all(format!("Duplicate word does not exist.").as_bytes())
                                .map_err(|err| {
                                    anyhow!("[Error][FileIOService -> process_file] Failed to write to file: {}", err)
                                })?;
        }
        
        Ok(())
    }
}
