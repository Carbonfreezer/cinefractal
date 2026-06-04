//! This is a simple test program to test base functionality without using python.
//! It renders a Julia set with a fixed seed and animates a fly-through of the
//! interesting focal points found by the focal system, zooming in and out at
//! each point along the way.

use fractal_core::prelude::*;
use macroquad::prelude::*;
use ndarray::Array3;

const WINDOW_SIZE: [i32; 2] = [600, 400];

/// The total time we take for the traversal of the path.
const TRAVERSAL_SECONDS: f32 = 4.0;

/// The time we take for a zoom phase (separate for in and out).
const ZOOM_TIME: f32 = 4.0;

/// The detail scale we want to zoom to.
const DETAIL_SCALE : f64 = 0.0001;

/// The coarse scale we hover at.
const COARSE_SCALE : f64 = 1.5;

/// The scale we search at.
const COARSE_SEARCH_SCALE : f64 = 2.0;

/// Sets the window's name and the required size.
fn window_conf() -> Conf {
    Conf {
        window_title: "Fractal".to_owned(),
        window_width: WINDOW_SIZE[0],
        window_height: WINDOW_SIZE[1],
        ..Default::default()
    }
}

/// Builds the animation: a Julia set with a fixed seed of (-0.8, 0.156) whose
/// render center point traverses the focal points found by the focal system.
/// The path traversal takes `TRAVERSAL_SECONDS`; at each focal point the view
/// zooms from `COARSE_SCALE` down to `DETAIL_SCALE` and back, each leg taking
/// `ZOOM_TIME`. The seed itself is not animated, only the center point and zoom.
fn build_animation() -> AnimationPlayer {
    // The configuration seeds time 0 with the start values set below.
    let mut animation = AnimationRecorder::new(AnimationStartConfiguration {
        exponent: 2,
        // A lower iteration count keeps the per-frame recompute interactive.
        max_iterations: 300,
        log_strength: 2.0,
        extension :  COARSE_SCALE,
        seed_real: -0.8,
        seed_imag: 0.156,
        ..Default::default()
    });

    let mut focal = FocalSystemInterface::default();
    focal.set_fractal_type(FractalType::Julia);
    focal.set_fractal_seed_value(-0.8, 0.156);
    // focal.set_fractal_type(FractalType::Mandelbrot);
    // We search for points in the Coarse region.
    focal.set_extension(COARSE_SEARCH_SCALE).unwrap();
    // The detail of each candidate is now measured at its own scale, decoupled from
    // the search radius. Set it to the extension we actually view at (here 1.5, so the
    // current circle test is unchanged). Lower it independently to judge points at a
    // deeper zoom than the search area.
    focal.set_evaluation_extension(DETAIL_SCALE);
    focal.set_maximum_iterations(300);

    let result = focal.create_collection_of_points(20);
    let list = result.get_path_from_start_point_with_distance(PathGenerationCriterion::ShortPath, -1.5, 0.0);
    let (_,_,end_dist) = list.last().unwrap();
    let time_scale = TRAVERSAL_SECONDS as f64 / end_dist;
    let mut time_passed = 0.0;
    for (x,y,dist) in list {

        time_passed += (dist * time_scale) as f32;
        animation.set_keyframe_render_center_point(time_passed, x,y );
        // Only zoom if not first point.
        if dist <= 0.0 { continue; }

        animation.set_keyframe_render_center_point(time_passed, x,y);
        animation.set_keyframe_extension(time_passed, COARSE_SCALE).unwrap();
        time_passed += ZOOM_TIME;
        animation.set_keyframe_extension(time_passed, DETAIL_SCALE).unwrap();
        time_passed += ZOOM_TIME;
        animation.set_keyframe_extension(time_passed, COARSE_SCALE).unwrap();
        // Set the position again to avoid moving out while zooming.
        animation.set_keyframe_render_center_point(time_passed, x,y);
    }

    animation.into_player()
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut image = Image::gen_image_color(WINDOW_SIZE[0] as u16, WINDOW_SIZE[1] as u16, BLANK);

    let mut interface = IterationFieldInterface::default();
    interface.set_numpy_extension(WINDOW_SIZE[1] as usize, WINDOW_SIZE[0] as usize);
    // The fractal *type* is not animated, only its parameters; so we set it
    // once. apply_animation patches the exponent and the seed value every frame.
    interface.set_fractal_type(FractalType::Julia);
    // interface.set_fractal_type(FractalType::Mandelbrot);
    // Log color strength is driven by the animation's log_strength track.
    interface.set_colorization_information(ColorSystem::Inferno, false);

    let animation = build_animation();
    let duration = animation.max_time();

    let texture = Texture2D::from_image(&image);

    loop {
        // Ping-pong the playback time so the animation runs back and forth
        // smoothly instead of snapping at the loop boundary.
        let time = ping_pong(get_time() as f32, duration);

        animation.apply_animation(time, &mut interface);
        let array = interface.get_color_field();
        generate_image_from_field(&mut image, array);
        texture.update(&image);

        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(screen_width(), screen_height())),
                ..Default::default()
            },
        );

       // draw_circle(screen_width() / 2.0, screen_height() / 2.0, 3.0, BLACK);

        next_frame().await;
    }
}

/// Maps elapsed wall-clock time onto a back-and-forth playback position within
/// `[0, duration]`. Falls back to a static frame for a zero-length animation.
fn ping_pong(elapsed: f32, duration: f32) -> f32 {
    if duration <= 0.0 {
        return 0.0;
    }
    let cycle = 2.0 * duration;
    let phase = elapsed % cycle;
    if phase > duration {
        cycle - phase
    } else {
        phase
    }
}

/// Takes the color field and writes it into the image.
fn generate_image_from_field(image: &mut Image, iteration_field: Array3<u8>) {
    for x in 0..WINDOW_SIZE[0] {
        for y in 0..WINDOW_SIZE[1] {
            // Numpy encodes row, column
            let r = iteration_field[(y as usize, x as usize, 0)];
            let g = iteration_field[(y as usize, x as usize, 1)];
            let b = iteration_field[(y as usize, x as usize, 2)];
            let color = Color::from_rgba(r, g, b, 255);
            image.set_pixel(x as u32, y as u32, color);
        }
    }
}
