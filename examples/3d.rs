use dla::Builder;

fn main() {
    let mut model = Builder::convex();

    model.add([0., 0., 0.].into(), 1);

    for _ in 0..100000 {
        model.add_particle();
    }

    model.save_csv("out.csv").unwrap();
    dla::Raycaster::convex(model.index)
        .with_color([0., 1., 0.])
        .render("render2.png");
}
