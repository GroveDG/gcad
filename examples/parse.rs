use gcad::{draw::draw_terminal, parse::GCADFigure};

fn main() {
    let gcad_str = "
A B _|_ B C _|_ C D _|_ D A
|A B| = 20
|B C| = 10
";
    let figure: GCADFigure = gcad_str.parse().unwrap();
    println!("{:?}", figure);
    let positions = figure.solve().unwrap();
    draw_terminal(positions, &figure);
}
