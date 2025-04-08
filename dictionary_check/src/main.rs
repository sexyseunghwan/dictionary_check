/*
Author      : Seunghwan Shin
Create date : 2025-04-08
Description : 검색사전에서 중복되는 단어를 찾아내주는 프로그램

History     : 2025-04-08 Seunghwan Shin       # [v.1.0.0] first create.
*/
mod common;
use crate::common::*;

mod utils;
use crate::utils::io_utils::*;
use crate::utils::logger_utils::*;

mod service;
use crate::service::file_io_service::*;
use crate::service::std_io_service::*;

mod controller;
use crate::controller::main_controller::*;

mod env_config;
use crate::env_config::env_config::*;

mod models;
use crate::models::search_dictionaries::*;

#[tokio::main]
async fn main() {
    
    dotenv().ok();
    set_global_logger();

    info!("Starting the application...");
    
    let search_dictionaries: SearchDictionaries =
        read_toml_from_file::<SearchDictionaries>(&DICTIONARY_LIST_PATH)
            .expect("[Error][main] Failed to read search_dictionaries.toml file");

    let file_io_service: FileIOServicePub = FileIOServicePub::new();
    let std_io_service: StdIOServicePub = StdIOServicePub::new();
    let mut controller: MainController<FileIOServicePub, StdIOServicePub> =
        MainController::new(file_io_service, std_io_service);

    controller
        .main_task(search_dictionaries)
        .await
        .expect("[Error][main] Failed to execute main task");

    info!("Application finished successfully.");
}
