use esp_idf_sys as _;

#[derive(Debug, Clone, Copy)]
pub struct EspInstant {
    micros: i64,
}

impl EspInstant {
    pub fn now() -> Self {
        Self {
            micros: unsafe { esp_idf_sys::esp_timer_get_time() },
        }
    }

    pub fn elapsed_us(&self) -> i64 {
        let current = unsafe { esp_idf_sys::esp_timer_get_time() };
        current - self.micros
    }

    pub fn elapsed_ms(&self) -> u32 {
        (self.elapsed_us() / 1000) as u32
    }
}

pub struct FrameAnimation {
    frames: Vec<&'static [u8]>,
    current_frame: usize,
    frame_duration_us: i64,
    last_update_time: i64,
    loop_animation: bool,
    is_finished: bool,
}

impl FrameAnimation {
    pub fn new(frame_duration_ms: u32) -> Self {
        Self {
            frames: Vec::new(),
            current_frame: 0,
            frame_duration_us: (frame_duration_ms as i64) * 1000,
            last_update_time: unsafe { esp_idf_sys::esp_timer_get_time() },
            loop_animation: true,
            is_finished: false,
        }
    }

    pub fn with_fps(fps: u32) -> Self {
        let frame_duration_ms = 1000 / fps;
        Self::new(frame_duration_ms)
    }

    pub fn add_frame(&mut self, frame_data: &'static [u8]) {
        self.frames.push(frame_data);
    }

    pub fn set_loop(&mut self, should_loop: bool) {
        self.loop_animation = should_loop;
    }

    pub fn set_fps(&mut self, fps: u32) {
        self.frame_duration_us = (1000 / fps) as i64 * 1000;
    }

    pub fn set_frame_duration_ms(&mut self, duration_ms: u32) {
        self.frame_duration_us = (duration_ms as i64) * 1000;
    }

    pub fn update(&mut self) -> bool {
        if self.is_finished || self.frames.is_empty() {
            return false;
        }

        let current_time = unsafe { esp_idf_sys::esp_timer_get_time() };
        let elapsed_us = current_time - self.last_update_time;

        if elapsed_us >= self.frame_duration_us {
            self.last_update_time = current_time;
            self.current_frame += 1;

            if self.current_frame >= self.frames.len() {
                if self.loop_animation {
                    self.current_frame = 0;
                } else {
                    self.current_frame = self.frames.len() - 1;
                    self.is_finished = true;
                }
            }

            return true;
        }

        false
    }

    pub fn get_current_frame(&self) -> Option<&'static [u8]> {
        self.frames.get(self.current_frame).copied()
    }

    pub fn get_current_frame_index(&self) -> usize {
        self.current_frame
    }

    pub fn get_frame_count(&self) -> usize {
        self.frames.len()
    }

    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.last_update_time = unsafe { esp_idf_sys::esp_timer_get_time() };
        self.is_finished = false;
    }

    pub fn is_finished(&self) -> bool {
        self.is_finished
    }

    pub fn jump_to_frame(&mut self, frame_index: usize) {
        if frame_index < self.frames.len() {
            self.current_frame = frame_index;
            self.last_update_time = unsafe { esp_idf_sys::esp_timer_get_time() };
            self.is_finished = false;
        }
    }
}
