use rustface::{Detector, FaceInfo, ImageData};

fn main() {
    let mut detector = rustface::create_detector("/model/seeta_fd_frontal_v1.0.bin").unwrap();
    detector.set_min_face_size(20);
    detector.set_score_thresh(2.0);
    detector.set_pyramid_scale_factor(0.8);
    detector.set_slide_window_step(4, 4);

    let binding = image::open("/123.png").unwrap();
    let b = binding.as_bytes();
    let mut image = ImageData::new(b, 100, 100);
    for face in detector.detect(&mut image).into_iter() {
        // print confidence score and coordinates
        println!("found face: {:?}", face);
    }
}
