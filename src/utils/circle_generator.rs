use std::f64::consts::PI;
use std::cell::RefCell;

thread_local! {
    static RNG_STATE: RefCell<u64> = const { RefCell::new(42) };
}

fn random_f64() -> f64 {
    #[cfg(target_arch = "wasm32")]
    {
        js_sys::Math::random()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        RNG_STATE.with(|state| {
            let mut s = state.borrow_mut();
            *s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            (*s >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
        })
    }
}

fn random_in_range(min: f64, max: f64) -> f64 {
    min + (random_f64() * (max - min))
}

#[derive(Clone, Debug)]
pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

#[derive(Debug)]
pub struct GenerationConfig {
    pub width: f64,
    pub height: f64,
    pub gap: f64,
    pub min_radius: f64,
    pub max_radius: f64,
    pub max_retries: Option<u32>,
}

fn circles_overlap(a: &Circle, b: &Circle, gap: f64) -> bool {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let dist = (dx * dx + dy * dy).sqrt();
    dist < a.radius + b.radius + gap
}

pub fn generate_circles(config: &GenerationConfig) -> Vec<Circle> {
    let width = config.width;
    let height = config.height;
    let gap = config.gap;
    let min_radius = config.min_radius;
    let max_radius = config.max_radius;
    let max_retries = config.max_retries.unwrap_or(500);

    if width <= 0.0 || height <= 0.0 {
        return Vec::new();
    }

    let mut circles: Vec<Circle> = Vec::new();

    let r1 = random_in_range(min_radius, max_radius);
    let x1 = random_in_range(r1 + gap, width - r1 - gap);
    let y1 = random_in_range(r1 + gap, height - r1 - gap);
    circles.push(Circle { x: x1, y: y1, radius: r1 });

    let mut failed_streak = 0;
    let max_failed_streak = 500;

    loop {
        let mut placed = false;

        for _ in 0..max_retries {
            let new_radius = random_in_range(min_radius, max_radius);

            let anchor_idx = (random_f64() * circles.len() as f64) as usize;
            let anchor = &circles[anchor_idx];

            let angle = random_f64() * PI * 2.0;
            let target_dist = anchor.radius + new_radius + gap;

            let new_x = anchor.x + angle.cos() * target_dist;
            let new_y = anchor.y + angle.sin() * target_dist;

            let candidate = Circle { x: new_x, y: new_y, radius: new_radius };

            if candidate.x - candidate.radius - gap < 0.0
                || candidate.x + candidate.radius + gap > width
                || candidate.y - candidate.radius - gap < 0.0
                || candidate.y + candidate.radius + gap > height
            {
                continue;
            }

            if circles.iter().any(|c| circles_overlap(&candidate, c, gap)) {
                continue;
            }
            circles.push(candidate);
            placed = true;
            failed_streak = 0;
            break;
        }

        if !placed {
            failed_streak += 1;
            if failed_streak >= max_failed_streak {
                break;
            }
        }
    }

    circles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_circles_basic() {
        let config = GenerationConfig {
            width: 800.0,
            height: 600.0,
            gap: 10.0,
            min_radius: 10.0,
            max_radius: 50.0,
            max_retries: None,
        };
        let circles = generate_circles(&config);
        assert!(!circles.is_empty(), "Should generate at least one circle");
    }

    #[test]
    fn test_generate_circles_no_overlap() {
        let config = GenerationConfig {
            width: 500.0,
            height: 500.0,
            gap: 5.0,
            min_radius: 20.0,
            max_radius: 40.0,
            max_retries: Some(1000),
        };
        let circles = generate_circles(&config);

        for i in 0..circles.len() {
            for j in (i + 1)..circles.len() {
                assert!(
                    !circles_overlap(&circles[i], &circles[j], config.gap),
                    "Circles {} and {} should not overlap",
                    i,
                    j
                );
            }
        }
    }

    #[test]
    fn test_generate_circles_within_bounds() {
        let config = GenerationConfig {
            width: 400.0,
            height: 300.0,
            gap: 5.0,
            min_radius: 10.0,
            max_radius: 30.0,
            max_retries: None,
        };
        let circles = generate_circles(&config);

        for (i, circle) in circles.iter().enumerate() {
            assert!(
                circle.x - circle.radius - config.gap >= 0.0,
                "Circle {} x out of bounds (left)",
                i
            );
            assert!(
                circle.x + circle.radius + config.gap <= config.width,
                "Circle {} x out of bounds (right)",
                i
            );
            assert!(
                circle.y - circle.radius - config.gap >= 0.0,
                "Circle {} y out of bounds (top)",
                i
            );
            assert!(
                circle.y + circle.radius + config.gap <= config.height,
                "Circle {} y out of bounds (bottom)",
                i
            );
        }
    }

    #[test]
    fn test_generate_circles_zero_dimensions() {
        let config = GenerationConfig {
            width: 0.0,
            height: 0.0,
            gap: 10.0,
            min_radius: 10.0,
            max_radius: 50.0,
            max_retries: None,
        };
        let circles = generate_circles(&config);
        assert!(circles.is_empty(), "Should return empty for zero dimensions");
    }

    #[test]
    fn test_generate_circles_negative_dimensions() {
        let config = GenerationConfig {
            width: -100.0,
            height: 200.0,
            gap: 10.0,
            min_radius: 10.0,
            max_radius: 50.0,
            max_retries: None,
        };
        let circles = generate_circles(&config);
        assert!(circles.is_empty(), "Should return empty for negative dimensions");
    }
}