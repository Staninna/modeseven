use std::time::Instant;

pub struct FpsCounter {
    last_time: Instant,
    frame_count: u32,
    current_fps: f32,
    update_interval: f32,
    elapsed: f32,
}

impl FpsCounter {
    pub fn new(update_interval: f32) -> Self {
        Self {
            last_time: Instant::now(),
            frame_count: 0,
            current_fps: 0.0,
            update_interval,
            elapsed: 0.0,
        }
    }

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