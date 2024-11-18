use std::time::Instant;

/// A frame rate counter with configurable update intervals
///
/// The FpsCounter provides a way to measure and monitor frame rates in real-time.
/// It accumulates frame counts over a specified interval and calculates the average
/// frames per second. The counter is designed to minimize overhead by only performing
/// FPS calculations at the configured update interval.
///
/// The counter uses Instant for precise timing and supports configurable update
/// intervals to balance between measurement accuracy and update frequency.
///
/// # Example
///
/// ```rust
/// // Create an FPS counter that updates every 0.5 seconds
/// let mut fps = FpsCounter::new(0.5);
///
/// // In game loop:
/// if let Some(current_fps) = fps.update() {
///     println!("Current FPS: {:.1}", current_fps);
/// }
/// ```
pub struct FpsCounter {
    /// Time of the last frame
    last_time: Instant,
    /// Number of frames counted in current interval
    frame_count: u32,
    /// Most recently calculated FPS value
    current_fps: f32,
    /// Time between FPS calculations (in seconds)
    update_interval: f32,
    /// Time accumulated in current interval
    elapsed: f32,
}

impl FpsCounter {
    /// Creates a new FPS counter with the specified update interval
    ///
    /// Initializes a counter that will calculate and return the average FPS
    /// every `update_interval` seconds. A shorter interval provides more frequent
    /// updates but may be more susceptible to variation, while a longer interval
    /// provides more stable readings.
    ///
    /// # Arguments
    ///
    /// * `update_interval` - Time in seconds between FPS calculations
    ///
    /// # Returns
    ///
    /// A new FpsCounter instance configured with the specified update interval
    ///
    /// # Example
    ///
    /// ```rust
    /// // Update FPS display twice per second
    /// let fps = FpsCounter::new(0.5);
    ///
    /// // Update FPS display every 100ms for more responsive display
    /// let fps_responsive = FpsCounter::new(0.1);
    /// ```
    pub fn new(update_interval: f32) -> Self {
        Self {
            last_time: Instant::now(),
            frame_count: 0,
            current_fps: 0.0,
            update_interval,
            elapsed: 0.0,
        }
    }

    /// Updates the counter and potentially returns a new FPS calculation
    ///
    /// This method should be called once per frame. It tracks the time between
    /// calls and accumulates frame counts. When the update interval is reached,
    /// it calculates and returns the current frames per second.
    ///
    /// The calculation process:
    /// 1. Increments the frame counter
    /// 2. Measures time since last update
    /// 3. Accumulates elapsed time
    /// 4. If update interval reached:
    ///    - Calculates average FPS for the interval
    ///    - Resets counters for next interval
    ///    - Returns the calculated FPS
    ///
    /// # Returns
    ///
    /// * `Some(fps)` - If the update interval was reached, returns the calculated FPS
    /// * `None` - If still accumulating frames for the current interval
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut fps = FpsCounter::new(1.0);
    ///
    /// // In game loop:
    /// match fps.update() {
    ///     Some(current_fps) => {
    ///         // Update FPS display
    ///         ui.set_fps_text(&format!("FPS: {:.1}", current_fps));
    ///     }
    ///     None => {
    ///         // Still accumulating frames, no update needed
    ///     }
    /// }
    /// ```
    pub fn update(&mut self) -> Option<f32> {
        // Increment frame counter for this interval
        self.frame_count += 1;

        // Calculate time since last frame
        let current_time = Instant::now();
        let dt = current_time.duration_since(self.last_time).as_secs_f32();
        self.last_time = current_time;

        // Accumulate time in current interval
        self.elapsed += dt;

        // Check if we've reached the update interval
        if self.elapsed >= self.update_interval {
            // Calculate average FPS over the interval
            self.current_fps = self.frame_count as f32 / self.elapsed;

            // Reset counters for next interval
            self.frame_count = 0;
            self.elapsed = 0.0;

            Some(self.current_fps)
        } else {
            None
        }
    }
}