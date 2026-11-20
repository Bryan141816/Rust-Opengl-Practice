use std::time::Instant;
#[allow(dead_code)]
pub struct Time {
    start_time: Instant,
    last_frame: Instant,
    delta_time: f32,
    elapsed_time: f32,
    frame_count: u64,
}
#[allow(dead_code)]
impl Time {
    pub fn new() -> Self {
        let now = Instant::now();

        Self {
            start_time: now,
            last_frame: now,
            delta_time: 0.0,
            elapsed_time: 0.0,
            frame_count: 0,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();

        self.delta_time = now
            .duration_since(self.last_frame)
            .as_secs_f32();

        self.elapsed_time = now
            .duration_since(self.start_time)
            .as_secs_f32();

        self.last_frame = now;
        self.frame_count += 1;
    }

    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }

    pub fn elapsed_time(&self) -> f32 {
        self.elapsed_time
    }

    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }

    pub fn fps(&self) -> f32 {
        if self.delta_time > 0.0 {
            1.0 / self.delta_time
        } else {
            0.0
        }
    }
}