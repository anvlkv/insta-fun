# Plotters SVG Feature and Limits Notes

This note prioritizes documented Plotters capabilities and configuration options for chart design, then summarizes SVG backend limits briefly.

## Documented Features to Use First

1. Chart construction pipeline
- `ChartBuilder`, cartesian coordinate setup, mesh/axis configuration, series drawing, and series labels.
- Reference: https://docs.rs/plotters/latest/plotters/index.html

2. Series and element feature flags
- `line_series`, `point_series`, `area_series`, `histogram`, `candlestick` and related flags can be enabled selectively.
- Reference: https://docs.rs/crate/plotters/latest/features

3. Styling and palettes
- Built-in colors, `ShapeStyle`, `TextStyle`, and palettes (`Palette99`, `Palette100`, full palette).
- Reference: https://docs.rs/plotters/latest/plotters/style/index.html

4. Backend portability
- Plotters chart code is backend-agnostic at API level; backend swap is usually initialization-level.
- Reference: https://docs.rs/plotters/latest/plotters/index.html#concepts-by-example

5. SVG output modes
- `SVGBackend::new(path, size)` for file output.
- `SVGBackend::with_string(buf, size)` for in-memory generation.
- Reference: https://docs.rs/plotters/latest/plotters/backend/struct.SVGBackend.html

## Example: Minimal SVG Render with Strict Error Handling

```rust
use plotters::prelude::*;

fn render_svg_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = SVGBackend::new(path, (800, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("CPU Load", ("sans-serif", 28).into_font())
        .margin(16)
        .x_label_area_size(32)
        .y_label_area_size(48)
        .build_cartesian_2d(0u32..60u32, 0f32..100f32)?;

    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(
        (0..=60).map(|x| (x, (x as f32 / 10.0).sin() * 25.0 + 50.0)),
        &GREEN,
    ))?;

    root.present()?;
    Ok(())
}
```

## Example: SVG to String for HTTP Response

```rust
use plotters::prelude::*;

fn render_svg_string() -> Result<String, Box<dyn std::error::Error>> {
    let mut svg = String::new();
    {
        let area = SVGBackend::with_string(&mut svg, (600, 300)).into_drawing_area();
        area.fill(&WHITE)?;
        area.draw(&Text::new(
            "Telemetry",
            (20, 30),
            ("sans-serif", 24).into_font(),
        ))?;
        area.present()?;
    }
    Ok(svg)
}
```

## Example: Slim Feature Configuration

```toml
[dependencies]
plotters = { version = "0.3.7", default-features = false, features = ["svg_backend", "line_series"] }
```

If you need bitmap embedding operations in SVG, ensure image-related features are enabled and confirm target architecture constraints.

## SVG Limits (Brief, Compared to Richer Backends)

1. Error handling behavior
- Saving via `drop` can hide write errors; call `present()` explicitly when correctness matters.
- References:
  - https://docs.rs/plotters/latest/plotters/index.html#misc
  - https://docs.rs/plotters-svg/latest/src/plotters_svg/svg.rs.html

2. Bitmap and target constraints
- SVG bitmap blit support is cfg/feature-dependent and not universally available across targets.
- Reference: https://docs.rs/plotters-svg/latest/src/plotters_svg/svg.rs.html

3. Text rendering differences
- Text sizing and anchor behavior use backend heuristics and may vary by viewer/font environment.
- Reference: https://docs.rs/plotters-svg/latest/src/plotters_svg/svg.rs.html
