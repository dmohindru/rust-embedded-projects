// use crate::display_driver::DisplayDriver;
// use crate::frame::Frame;

// pub struct RefreshEngine {}

// impl RefreshEngine {
//     pub fn new() -> Self {
//         Self {}
//     }

//     /// One tick of the refresh cycle: ask the driver to render the current frame
//     pub fn tick<D: DisplayDriver>(&mut self, driver: &mut D, frame: &Frame) {
//         driver.render(frame);
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::frame::{Frame, FrameData};
//     use heapless::Vec;

//     // Simple synchronous mock driver
//     struct MockDriver {
//         rendered_frames: Vec<Frame, 3>,
//     }

//     impl DisplayDriver for MockDriver {
//         fn render(&mut self, frame: &Frame) {
//             self.rendered_frames
//                 .push(*frame)
//                 .expect("Rendering frame failed");
//         }
//     }

//     // Three simple frames for testing
//     const F1: Frame = [0x1, 0x1, 0x1, 0x1, 0x1];
//     const F2: Frame = [0x2, 0x2, 0x2, 0x2, 0x2];
//     const F3: Frame = [0x3, 0x3, 0x3, 0x3, 0x3];

//     static FRAMES: &[Frame] = &[F1, F2, F3];

//     #[test]
//     fn test_refresh_calls_driver() {
//         let frame_data = FrameData::<3>::new(&FRAMES);
//         let mut engine = RefreshEngine::new();

//         let mut mock = MockDriver {
//             rendered_frames: Vec::new(),
//         };

//         // Call tick on current frame (F1)
//         engine.tick(&mut mock, frame_data.current_frame());

//         // Check that F1 was rendered
//         assert_eq!(mock.rendered_frames.len(), 1);
//         assert_eq!(mock.rendered_frames[0], F2);
//     }
// }
