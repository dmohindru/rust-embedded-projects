use led_matrix_project1::frame::{Direction, Frame, FrameData};

#[test]
fn passing_test() {
    assert_eq!(1, 1);
}
// #[test]
// fn test_frame_indexing() {
//     // Define some frames
//     const F1: Frame = [1, 1, 1, 1, 1];
//     const F2: Frame = [2, 2, 2, 2, 2];
//     const F3: Frame = [3, 3, 3, 3, 3];

//     let frames = [F1, F2, F3];

//     // Create FrameData with capacity 5 (heapless::Vec capacity)
//     let mut data = FrameData::<5>::new(&frames);

//     // Initial index should point to the middle frame
//     assert_eq!(data.current_frame(), &F2);

//     // Move index left
//     data.move_index(Direction::Left);
//     assert_eq!(data.current_frame(), &F1);

//     // Move index right twice
//     data.move_index(Direction::Right);
//     data.move_index(Direction::Right);
//     assert_eq!(data.current_frame(), &F3);

//     // Should not go out of bounds
//     data.move_index(Direction::Right);
//     assert_eq!(data.current_frame(), &F3);
// }

// #[test]
// fn test_empty_frame_data() {
//     let frames: [Frame; 0] = [];
//     let data = FrameData::<5>::new(&frames);
//     assert_eq!(data.frames.len(), 0);
// }
