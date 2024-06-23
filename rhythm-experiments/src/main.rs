use std::{env, fs, thread, time::Duration};

use brainflow::{
    board_shim,
    brainflow_input_params::{BrainFlowInputParams, BrainFlowInputParamsBuilder},
    data_filter, BoardIds, BrainFlowPresets,
};

fn main() {
    brainflow::board_shim::enable_dev_board_logger().unwrap();
    let params = BrainFlowInputParamsBuilder::default()
        .serial_port("/dev/cu.usbserial-DN0094WP")
        .build();
    let board = board_shim::BoardShim::new(BoardIds::CytonBoard, params).unwrap();

    board.prepare_session().unwrap();
    board.start_stream(45000, "").unwrap();
    thread::sleep(Duration::from_secs(5));
    board.stop_stream().unwrap();
    let data = board
        .get_board_data(Some(15), BrainFlowPresets::DefaultPreset)
        .unwrap();
    let board_desc =
        board_shim::get_board_descr(board.get_board_id(), BrainFlowPresets::DefaultPreset);
    board.release_session().unwrap();

    println!("{}", data.len());
    println!("{:?}", data);
    println!("{:?}", board_desc);

    let mut cur_dir = env::current_dir().unwrap();
    cur_dir.push("brainflow_data");
    // tmp_dir.push("brainflow_data");
    // fs::create_dir_all(&tmp_dir).unwrap();
    // tmp_dir.push("read-write_file.csv");

    let filename = cur_dir.to_str().unwrap();
    data_filter::write_file(&data, filename, "w").unwrap();
    println!("{:?}", filename);
}
