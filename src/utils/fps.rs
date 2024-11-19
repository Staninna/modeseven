//! Frame rate counter

use std::time::Instant;

/// A frame rate counter with configurable measurement intervals
///
/// FpsCounter tracks frame timing and calculates average FPS over intervals.
/// It provides:
/// * Configurable update frequency
/// * Frame counting and timing
/// * Average FPS calculations
/// * Low-overhead measurement
///
/// Uses Instant for precise frame timing.
pub struct FpsCounter {
    /// Time of the last frame
    last_time: Instant,
    /// Frames in current interval
    frame_count: u32,
    /// Last calculated FPS
    current_fps: f32,
    /// Seconds between FPS updates
    update_interval: f32,
    /// Time in current interval
    elapsed: f32,
}

impl FpsCounter {
    /// Creates a new FPS counter with specified update rate
    ///
    /// # Arguments
    ///
    /// * `update_interval` - Seconds between FPS calculations
    ///
    /// # Returns
    ///
    /// A new FpsCounter configured with the interval
    pub fn new(update_interval: f32) -> Self {
        Self {
            last_time: Instant::now(),
            frame_count: 0,
            current_fps: 0.0,
            update_interval,
            elapsed: 0.0,
        }
    }

    /// Updates counter and returns FPS if interval completed
    ///
    /// Tracks time between frames and counts them until the update
    /// interval is reached. Then calculates and returns the average
    /// FPS for that interval.
    ///
    /// # Returns
    ///
    /// * `Some(fps)` - New FPS calculation if interval completed
    /// * `None` - Still counting frames in current interval
    pub fn update(&mut self) -> Option<f32> {
        self.frame_count += 1;

        let current_time = Instant::now();
        let dt = current_time.duration_since(self.last_time).as_secs_f32();
        self.last_time = current_time;

        self.elapsed += dt;

        if self.elapsed >= self.update_interval {
            self.current_fps = self.frame_count as f32 / self.elapsed;
            self.frame_count = 0;
            self.elapsed = 0.0;
            Some(self.current_fps)
        } else {
            None
        }
    }
}
