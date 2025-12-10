mod inputs;

use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("polyline.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let points: Vec<_> = inputs::INPUT
        .lines()
        .map(|l| {
            let pos: Vec<_> = l.split(',').map(|n| n.parse().unwrap()).collect();
            (pos[0], pos[1])
        })
        .collect();

    let x_min = points.iter().map(|p| p.0).fold(f64::INFINITY, f64::min);
    let x_max = points.iter().map(|p| p.0).fold(f64::NEG_INFINITY, f64::max);
    let y_min = points.iter().map(|p| p.1).fold(f64::INFINITY, f64::min);
    let y_max = points.iter().map(|p| p.1).fold(f64::NEG_INFINITY, f64::max);

    // Ajouter une marge de 5%
    let x_margin = (x_max - x_min) * 0.05;
    let y_margin = (y_max - y_min) * 0.05;

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(
            (x_min - x_margin)..(x_max + x_margin),
            (y_min - y_margin)..(y_max + y_margin),
        )?;

    chart.configure_mesh().x_desc("X").y_desc("Y").draw()?;

    // Ligne connectée
    chart.draw_series(LineSeries::new(points.clone(), &BLUE.mix(0.8)))?;

    // Points
    chart.draw_series([Circle::new(points[1], 1, RED.filled())])?;

    root.present()?;
    Ok(())
}
