use dla::Builder;

fn main() {
    let mut model = Builder::flat();

    for i in 1..1000 {
        let angle = 2. * std::f64::consts::PI * (i as f64 / 100.);
        let r = 50.;
        let x = r * angle.cos();
        let y = r * angle.sin();
        model.add([x, y].into(), i);
    }

    for _ in 0..1000 {
        model.add_particle();
    }

    model.save_csv("out.csv").unwrap();
    dla::Raycaster::flat(model.index)
        .with_color(|_| [1., 1., 1.])
        .render("render.png");
}
