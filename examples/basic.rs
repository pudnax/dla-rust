use dla::Builder;

fn main() {
    let mut model = Builder::flat();

    model.add([0., 0.].into(), 1);

    for _ in 0..1000 {
        model.add_particle();
    }

    model.save_csv("out.csv").unwrap();
    dla::Raycaster::flat(model.index)
        .with_color([1., 1., 1.])
        .render("render.png");
}
