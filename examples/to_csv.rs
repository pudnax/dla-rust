use dla::Builder;

fn main() {
    let mut model = Builder::flat();

    model.add([60., 0.].into(), 0);
    model.add([-60., 0.].into(), 1);

    for _ in 0..10000 {
        model.add_particle();
    }

    model.save_csv("out.csv").unwrap();
    model.render("render3.png");
}
