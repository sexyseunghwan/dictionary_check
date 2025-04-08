use crate::common::*;

#[async_trait]
pub trait StdIOService {
    fn write_to_stdout(&mut self, message: &str);
    fn wirte_to_newline(&mut self);
    fn read_to_stdin(&mut self) -> String;
}

#[derive(Debug, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct StdIOServicePub {
    pub stdout: io::Stdout,
}

impl StdIOServicePub {
    pub fn new() -> Self {
        Self {
            stdout: io::stdout(),
        }
    }
}

impl StdIOService for StdIOServicePub {
    #[doc = "Write to stdout"]
    /// # Arguments
    /// * `message` - Index schedule information
    ///
    /// # Returns
    /// * `None` - No return value
    fn write_to_stdout(&mut self, message: &str) {
        writeln!(self.stdout, "{}", message).unwrap();
        self.stdout.flush().unwrap(); /* 즉시출력 */
    }

    #[doc = "새로운 라인을 작성하는 함수"]
    fn wirte_to_newline(&mut self) {
        writeln!(self.stdout, "\n").unwrap();
        self.stdout.flush().unwrap(); /* 즉시출력 */
    }

    #[doc = "Read from stdin"]
    fn read_to_stdin(&mut self) -> String {

        let mut input: String = String::new();
        io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
        input
        
    }
}
