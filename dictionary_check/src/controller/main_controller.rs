use crate::common::*;

use crate::service::file_io_service::*;
use crate::service::std_io_service::*;

use crate::models::search_dictionaries::*;

#[derive(Debug, new)]
pub struct MainController<F: FileIOService, S: StdIOService> {
    file_io_service: F,
    std_io_service: S,
}

impl<F: FileIOService, S: StdIOService> MainController<F, S> {
    
    #[doc = "main task"]
    pub async fn main_task(
        &mut self,
        dictionaries: SearchDictionaries,
    ) -> Result<(), anyhow::Error> {

        loop {

            /* 화면 출력을 이쁘게 하기 위함 */
            for _i in 0..=50 {
                self.std_io_service.wirte_to_newline();
            }
            
            self.std_io_service
                .write_to_stdout("[================ ALBA DICTIONARY CHECK CLI ================]");
            self.std_io_service
                .write_to_stdout("Select the dictionary you want to perform.");

            let mut idx: i32 = 0; /* Dictionary indexing */

            for dic in dictionaries.dictionary.iter() {
                idx += 1;
                self.std_io_service
                    .write_to_stdout(format!("[{}] {}", idx, dic.dictionary_name).as_str());
            }
            
            self.std_io_service.wirte_to_newline();
            self.std_io_service
                .write_to_stdout("Please enter your number: ");

            let input: String = self.std_io_service.read_to_stdin();

            match input.trim().parse::<i32>() {
                Ok(number) => {
                    if number > 0 && number <= idx {
                        self.std_io_service.write_to_stdout("Start Processing...");

                        match self
                            .file_io_service
                            .process_file(&dictionaries.dictionary[(number - 1) as usize])
                        {
                            Ok(_) => {
                                self.std_io_service
                                    .write_to_stdout("The task has been completed.");
                                
                                self.std_io_service
                                    .write_to_stdout("**Press any key to exit.: ");
                                self.std_io_service.read_to_stdin();
                                
                                break;
                            }
                            Err(e) => {
                                error!("[Error][MainController -> main_task] {:?}", e);
                                self.std_io_service.write_to_stdout("The task failed.");
                                self.std_io_service
                                    .write_to_stdout("**Press any key to exit.: ");
                                self.std_io_service.read_to_stdin();

                                break;
                            }
                        }
                    } else {
                        self.std_io_service
                            .write_to_stdout("**Invalid number, please try again.");
                        self.std_io_service
                            .write_to_stdout("**Press any key to continue.: ");
                        self.std_io_service.read_to_stdin();

                    }
                }
                Err(_) => {
                    self.std_io_service
                        .write_to_stdout("**Invalid number, please try again.");
                    self.std_io_service
                        .write_to_stdout("**Press any key to continue.: ");
                    self.std_io_service.read_to_stdin();
                }
            }
        }

        Ok(())
    }
}
