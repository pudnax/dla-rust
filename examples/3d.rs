use dla::{Builder, Vec3d};

fn main() {
    let mut model = Builder::convex();

    model.add([0., 0., 0.].into(), 1);

    for _ in 0..100_000 {
        model.add_particle();
    }

    model.save_csv("out.csv").unwrap();
    dla::Raycaster::convex(model.index)
        .w_h(4096, 4096)
        .with_color(palette)
        .render("render.png");
}

fn palette(d: f64) -> [f64; 3] {
    let yellow = Vec3d::new(2.38, 2.52, 0.); // note that the color is "hot", i.e. has components >1
    let green = Vec3d::new(0., 1.73, 0.7);
    let blue = Vec3d::new(0.02, 0.48, 0.57);
    let darkblack = Vec3d::new(0.05, 0.19, 0.3);
    let black = Vec3d::new(0., 0., 0.01);

    let x = d.max(0.).min(1.);
    if x < 0.25 {
        return Vec3d::lerp(black, darkblack, x * 4.).as_slice();
    } else if x < 0.5 {
        return Vec3d::lerp(darkblack, blue, x * 4. - 1.).as_slice();
    } else if x < 0.75 {
        return Vec3d::lerp(blue, green, x * 4. - 2.).as_slice();
    }
    Vec3d::lerp(green, yellow, x * 4. - 3.).as_slice()
}
