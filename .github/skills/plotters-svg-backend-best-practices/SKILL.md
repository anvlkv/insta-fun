---
name: plotters-svg-backend-best-practices
description: 'Best practices for using plotters with SVGBackend, focused on documented chart design features, configuration options, style systems, and backend selection. Use when designing charts, choosing feature flags, and summarizing SVG backend limits versus richer backends.'
argument-hint: 'Describe your chart goal, target environment (native or wasm), and whether you need file or in-memory SVG output.'
---

# Plotters SVG Backend Best Practices

## When to Use

- Creating vector charts with Plotters using SVG output
- Designing chart layouts with documented Plotters APIs (ChartBuilder, mesh, series, style)
- Choosing cargo features for smaller builds or specific chart capabilities
- Rendering to a file or to an in-memory string
- Explaining what SVG can and cannot do compared with richer backends

## Decision Flow

| Decision | Use This |
|---|---|
| Need editable/scalable vector output | SVGBackend |
| Need raster pixel output or GIF | BitMapBackend |
| Need SVG as a string (API response, tests, embedding) | SVGBackend::with_string |
| Need to draw embedded bitmaps into SVG | Build with image support and non-wasm target |
| Need strict error handling for writes | Always call present explicitly |

## Documented Plotters Capabilities to Lean On

- Drawing composition: split drawing areas, margins, titled areas, layered elements
- Chart construction: cartesian coordinates, mesh/axis configuration, label formatting
- Built-in series support: line, point, area, histogram, candlestick (feature dependent)
- Style system: ShapeStyle, TextStyle, color constants, Palette99/Palette100/full palette
- Text and font controls: family, style, transform, and optional font registration paths
- Multi-target portability: same chart code can target bitmap, SVG, canvas, and other backends

## Procedure

### 1. Choose Features by Chart Intent

Minimal SVG-first dependency:

```toml
[dependencies]
plotters = { version = "0.3.7", default-features = false, features = ["svg_backend", "line_series", "point_series"] }
```

Notes:
- In plotters, svg_backend is enabled by default, but explicit feature sets reduce dependency weight.
- Add feature flags only for what your chart needs: `line_series`, `point_series`, `area_series`, `histogram`, `candlestick`, etc.
- For richer text/font behavior, include relevant text/font features (`ttf` or `ab_glyph`, depending on strategy).

### 2. Build Charts Using the Standard Plotters Design Pattern

Use this sequence consistently:
1. Create drawing area and fill background.
2. Configure chart frame with caption/margins/label areas.
3. Build coordinate system.
4. Draw mesh and axis formatting.
5. Draw one or more series.
6. Add legends/labels and call `present`.

Reference example:

```rust
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = SVGBackend::new("out/chart.svg", (960, 540)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Sine", ("sans-serif", 36).into_font())
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(0f32..10f32, -1.2f32..1.2f32)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        (0..1000).map(|i| {
            let x = i as f32 / 100.0;
            (x, x.sin())
        }),
        &BLUE,
    ))?;

    // Do not rely on drop for important error handling.
    root.present()?;
    Ok(())
}
```

### 3. Design for Readability and Composition

Recommended chart design knobs:
- `margin`, `x_label_area_size`, `y_label_area_size` for spacing and clipping control
- `configure_mesh()` for grid density, axis labels, and formatter customization
- `configure_series_labels()` for legend position and styling
- Palette-based color picking (`Palette99::pick(i)`) for multi-series readability
- Split layouts for dashboards: `split_evenly`, child drawing areas, and shared style conventions

### 4. Use In-Memory SVG for Tests and Web Responses

```rust
use plotters::prelude::*;

fn build_svg() -> Result<String, Box<dyn std::error::Error>> {
    let mut svg = String::new();
    {
        let root = SVGBackend::with_string(&mut svg, (640, 360)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption("Quadratic", ("sans-serif", 24).into_font())
            .margin(10)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(-1f32..1f32, 0f32..1f32)?;

        chart.configure_mesh().draw()?;
        chart.draw_series(LineSeries::new(
            (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
            &RED,
        ))?;

        root.present()?;
    }
    Ok(svg)
}
```

### 5. Validate the Configuration and Output

Use these checks:
- Build with only intended features enabled.
- Verify expected series/features compile (for example histogram or candlestick).
- Open output in at least two SVG renderers.
- Confirm labels and legend are readable at target dimensions.
- Keep `present()` explicit where write errors matter.

### 6. Keep SVG Limits Brief and Backend-Aware

- SVG is excellent for scalable static vector output and web embedding.
- Richer raster/interactive backends may be preferable for animation-heavy workflows, pixel-level operations, or live rendering stacks.
- Bitmap embedding in SVG and some behaviors depend on compile-time cfg/features; verify target + features early.

## Verified Limitations and Caveats

- `SVGBackend` is feature-gated (`svg_backend`) when defaults are disabled.
- `drop`-time save ignores errors by design; explicit `present` is safer.
- Some operations (for example bitmap blit) are cfg/feature dependent.
- Text placement can vary by font/rendering environment.

See detailed references in [references/limitations.md](./references/limitations.md).

## Quick Troubleshooting

| Symptom | Likely Cause | Fix |
|---|---|---|
| Empty or missing output file | No explicit present call and write failed during drop | Call present and handle result |
| Text appears shifted between environments | Different font availability/metrics in renderer | Use common font families and test in target viewer |
| Bitmap overlay missing in wasm build | blit_bitmap not compiled on wasm32 | Avoid bitmap blit on wasm or switch backend strategy |
| Build unexpectedly pulls heavy deps | Using default feature set | Disable defaults and opt in only required features |

## Backend Comparison (Brief)

| Backend | Strong At | Tradeoff |
|---|---|---|
| SVGBackend | Scalable vector output, easy embedding, clean static export | Less suited to bitmap-first or highly interactive rendering paths |
| BitMapBackend | Pixel operations, raster outputs, GIF workflows | Not resolution-independent like vector SVG |
| Canvas/Cairo style richer stacks | UI/runtime integration and richer rendering pipelines | Platform/runtime complexity is usually higher |

## References

- [Plotters crate docs](https://docs.rs/plotters/latest/plotters/index.html)
- [SVGBackend in plotters](https://docs.rs/plotters/latest/plotters/backend/struct.SVGBackend.html)
- [plotters feature flags](https://docs.rs/crate/plotters/latest/features)
- [plotters-svg feature flags](https://docs.rs/crate/plotters-svg/latest/features)
- [plotters-svg source svg.rs](https://docs.rs/plotters-svg/latest/src/plotters_svg/svg.rs.html)
