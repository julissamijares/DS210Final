use plotters::prelude::*;
use std::collections::HashMap;

const OUT_FILE_NAME: &str = "degree_distribution.png";

pub fn plot_degree_distribution(degrees: &[usize]) -> Result<(), Box<dyn std::error::Error>> {
    let mut degree_counts = HashMap::new();
    for degree in degrees {
        let count = degree_counts.entry(*degree).or_insert(0);
        *count += 1;
    }

    let root = BitMapBackend::new(OUT_FILE_NAME, (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let max_degree = degree_counts.keys().max().unwrap_or(&0) + 1;
    let max_count = degree_counts.values().max().unwrap_or(&0) + 1;


    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Degree Distribution", ("sans-serif", 50.0))
        .build_cartesian_2d(0..max_degree, 0..max_count)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("Count")
        .x_desc("Degree")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    let mut data = Vec::new();
    for (degree, count) in degree_counts {
        data.push((degree, count));
    }

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(GREEN.filled())
            .margin(0)
            .data(data),
    )?;

    root.present()?;
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}

#[test]
fn test_plot_degree_distribution() -> Result<(), Box<dyn std::error::Error>> {
    let degrees = vec![2, 3, 3, 4, 4, 4, 5, 5, 5, 5];

    plot_degree_distribution(&degrees)?;

    assert!(std::path::Path::new(OUT_FILE_NAME).exists());

    Ok(())
}
