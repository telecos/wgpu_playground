/// Performance metrics tracking for GPU operations
use std::collections::VecDeque;
use std::time::Instant;

/// Performance metrics tracker
#[derive(Debug)]
pub struct PerformanceMetrics {
    /// Frame times for FPS calculation (in milliseconds)
    frame_times: VecDeque<f32>,
    /// Last frame start time
    last_frame_time: Option<Instant>,
    /// Current FPS
    current_fps: f32,
    /// Current frame time in milliseconds
    current_frame_time: f32,
    /// Peak frame time in milliseconds
    peak_frame_time: f32,
    /// Average frame time in milliseconds
    average_frame_time: f32,
    /// GPU memory usage (simulated/estimated for now)
    gpu_memory_usage_mb: f32,
    /// Command buffer count
    command_buffer_count: usize,
    /// Draw call count
    draw_call_count: usize,
    /// Compute dispatch count
    compute_dispatch_count: usize,
    /// Maximum samples to keep for graphs
    max_samples: usize,
    /// Whether metrics are paused
    paused: bool,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl PerformanceMetrics {
    /// Create a new performance metrics tracker
    pub fn new() -> Self {
        Self {
            frame_times: VecDeque::with_capacity(300), // 5 seconds at 60 FPS
            last_frame_time: None,
            current_fps: 0.0,
            current_frame_time: 0.0,
            peak_frame_time: 0.0,
            average_frame_time: 0.0,
            gpu_memory_usage_mb: 0.0,
            command_buffer_count: 0,
            draw_call_count: 0,
            compute_dispatch_count: 0,
            max_samples: 300,
            paused: false,
        }
    }

    /// Start a new frame measurement
    pub fn start_frame(&mut self) {
        if self.paused {
            return;
        }
        self.last_frame_time = Some(Instant::now());
    }

    /// End frame measurement and update metrics
    pub fn end_frame(&mut self) {
        if self.paused {
            return;
        }

        if let Some(start) = self.last_frame_time {
            let elapsed = start.elapsed();
            let frame_time_ms = elapsed.as_secs_f32() * 1000.0;

            // Update current frame time
            self.current_frame_time = frame_time_ms;

            // Update peak frame time
            if frame_time_ms > self.peak_frame_time {
                self.peak_frame_time = frame_time_ms;
            }

            // Add to history
            self.frame_times.push_back(frame_time_ms);
            if self.frame_times.len() > self.max_samples {
                self.frame_times.pop_front();
            }

            // Calculate average
            if !self.frame_times.is_empty() {
                let sum: f32 = self.frame_times.iter().sum();
                self.average_frame_time = sum / self.frame_times.len() as f32;
            }

            // Calculate FPS (avoid division by zero)
            if self.average_frame_time > 0.0 {
                self.current_fps = 1000.0 / self.average_frame_time;
            } else {
                self.current_fps = 0.0;
            }
        }
    }

    /// Get current FPS
    pub fn fps(&self) -> f32 {
        self.current_fps
    }

    /// Get current frame time in milliseconds
    pub fn frame_time_ms(&self) -> f32 {
        self.current_frame_time
    }

    /// Get average frame time in milliseconds
    pub fn average_frame_time_ms(&self) -> f32 {
        self.average_frame_time
    }

    /// Get peak frame time in milliseconds
    pub fn peak_frame_time_ms(&self) -> f32 {
        self.peak_frame_time
    }

    /// Get frame time history for graphing
    pub fn frame_time_history(&self) -> &VecDeque<f32> {
        &self.frame_times
    }

    /// Get GPU memory usage in MB (estimated)
    pub fn gpu_memory_mb(&self) -> f32 {
        self.gpu_memory_usage_mb
    }

    /// Set GPU memory usage in MB
    pub fn set_gpu_memory_mb(&mut self, mb: f32) {
        self.gpu_memory_usage_mb = mb;
    }

    /// Get command buffer count
    pub fn command_buffer_count(&self) -> usize {
        self.command_buffer_count
    }

    /// Set command buffer count
    pub fn set_command_buffer_count(&mut self, count: usize) {
        self.command_buffer_count = count;
    }

    /// Increment command buffer count
    pub fn increment_command_buffer_count(&mut self) {
        self.command_buffer_count += 1;
    }

    /// Get draw call count
    pub fn draw_call_count(&self) -> usize {
        self.draw_call_count
    }

    /// Set draw call count
    pub fn set_draw_call_count(&mut self, count: usize) {
        self.draw_call_count = count;
    }

    /// Increment draw call count
    pub fn increment_draw_call_count(&mut self) {
        self.draw_call_count += 1;
    }

    /// Get compute dispatch count
    pub fn compute_dispatch_count(&self) -> usize {
        self.compute_dispatch_count
    }

    /// Set compute dispatch count
    pub fn set_compute_dispatch_count(&mut self, count: usize) {
        self.compute_dispatch_count = count;
    }

    /// Increment compute dispatch count
    pub fn increment_compute_dispatch_count(&mut self) {
        self.compute_dispatch_count += 1;
    }

    /// Reset all counters
    pub fn reset(&mut self) {
        self.frame_times.clear();
        self.last_frame_time = None;
        self.current_fps = 0.0;
        self.current_frame_time = 0.0;
        self.peak_frame_time = 0.0;
        self.average_frame_time = 0.0;
        self.command_buffer_count = 0;
        self.draw_call_count = 0;
        self.compute_dispatch_count = 0;
    }

    /// Reset peak values
    pub fn reset_peaks(&mut self) {
        self.peak_frame_time = 0.0;
    }

    /// Pause/resume metrics collection
    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }

    /// Check if metrics are paused
    pub fn is_paused(&self) -> bool {
        self.paused
    }

    /// Get the 1% low FPS (worst 1% of frame times)
    pub fn fps_1_percent_low(&self) -> f32 {
        if self.frame_times.is_empty() {
            return 0.0;
        }

        let mut sorted: Vec<f32> = self.frame_times.iter().copied().collect();
        sorted.sort_by(|a, b| b.partial_cmp(a).unwrap());

        let index = (sorted.len() as f32 * 0.01).ceil() as usize;
        let index = index.max(1).min(sorted.len());

        if let Some(&worst_frame_time) = sorted.get(index - 1) {
            if worst_frame_time > 0.0 {
                return 1000.0 / worst_frame_time;
            }
        }

        0.0
    }

    /// Get the 0.1% low FPS (worst 0.1% of frame times)
    pub fn fps_0_1_percent_low(&self) -> f32 {
        if self.frame_times.is_empty() {
            return 0.0;
        }

        let mut sorted: Vec<f32> = self.frame_times.iter().copied().collect();
        sorted.sort_by(|a, b| b.partial_cmp(a).unwrap());

        let index = (sorted.len() as f32 * 0.001).ceil() as usize;
        let index = index.max(1).min(sorted.len());

        if let Some(&worst_frame_time) = sorted.get(index - 1) {
            if worst_frame_time > 0.0 {
                return 1000.0 / worst_frame_time;
            }
        }

        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_performance_metrics_creation() {
        let metrics = PerformanceMetrics::new();
        assert_eq!(metrics.fps(), 0.0);
        assert_eq!(metrics.frame_time_ms(), 0.0);
        assert_eq!(metrics.command_buffer_count(), 0);
    }

    #[test]
    fn test_frame_measurement() {
        let mut metrics = PerformanceMetrics::new();

        // Simulate a frame
        metrics.start_frame();
        sleep(Duration::from_millis(16)); // ~60 FPS
        metrics.end_frame();

        // Frame time should be around 16ms
        assert!(metrics.frame_time_ms() > 15.0);
        assert!(metrics.frame_time_ms() < 20.0);

        // FPS should be around 60
        assert!(metrics.fps() > 50.0);
        assert!(metrics.fps() < 70.0);
    }

    #[test]
    fn test_counter_increments() {
        let mut metrics = PerformanceMetrics::new();

        metrics.increment_command_buffer_count();
        metrics.increment_draw_call_count();
        metrics.increment_compute_dispatch_count();

        assert_eq!(metrics.command_buffer_count(), 1);
        assert_eq!(metrics.draw_call_count(), 1);
        assert_eq!(metrics.compute_dispatch_count(), 1);
    }

    #[test]
    fn test_reset() {
        let mut metrics = PerformanceMetrics::new();

        metrics.set_command_buffer_count(10);
        metrics.set_draw_call_count(20);
        metrics.set_compute_dispatch_count(5);

        metrics.reset();

        assert_eq!(metrics.command_buffer_count(), 0);
        assert_eq!(metrics.draw_call_count(), 0);
        assert_eq!(metrics.compute_dispatch_count(), 0);
        assert_eq!(metrics.fps(), 0.0);
    }

    #[test]
    fn test_paused_metrics() {
        let mut metrics = PerformanceMetrics::new();

        // Pause metrics
        metrics.set_paused(true);
        assert!(metrics.is_paused());

        // Frame measurement should not update when paused
        metrics.start_frame();
        sleep(Duration::from_millis(16));
        metrics.end_frame();

        assert_eq!(metrics.frame_time_ms(), 0.0);
        assert_eq!(metrics.fps(), 0.0);

        // Resume
        metrics.set_paused(false);
        assert!(!metrics.is_paused());
    }

    #[test]
    fn test_peak_frame_time() {
        let mut metrics = PerformanceMetrics::new();

        // Simulate multiple frames
        for _ in 0..3 {
            metrics.start_frame();
            sleep(Duration::from_millis(10));
            metrics.end_frame();
        }

        // Simulate a slow frame
        metrics.start_frame();
        sleep(Duration::from_millis(50));
        metrics.end_frame();

        // Peak should be around 50ms
        assert!(metrics.peak_frame_time_ms() > 45.0);
    }

    #[test]
    fn test_gpu_memory() {
        let mut metrics = PerformanceMetrics::new();

        metrics.set_gpu_memory_mb(256.5);
        assert_eq!(metrics.gpu_memory_mb(), 256.5);
    }

    #[test]
    fn test_frame_time_history() {
        let mut metrics = PerformanceMetrics::new();

        // Add some frame times
        for _ in 0..5 {
            metrics.start_frame();
            sleep(Duration::from_millis(16));
            metrics.end_frame();
        }

        let history = metrics.frame_time_history();
        assert_eq!(history.len(), 5);
    }

    #[test]
    fn test_max_samples() {
        let mut metrics = PerformanceMetrics::new();
        metrics.max_samples = 10;

        // Add more than max samples
        for _ in 0..20 {
            metrics.start_frame();
            sleep(Duration::from_millis(1));
            metrics.end_frame();
        }

        // Should only keep max_samples
        assert_eq!(metrics.frame_time_history().len(), 10);
    }

    #[test]
    fn test_1_percent_low_fps() {
        let mut metrics = PerformanceMetrics::new();

        // Add 100 frame times, with one really slow frame
        for i in 0..100 {
            metrics.start_frame();
            if i == 99 {
                sleep(Duration::from_millis(100)); // Slow frame
            } else {
                sleep(Duration::from_millis(16)); // Normal frames
            }
            metrics.end_frame();
        }

        let fps_1_low = metrics.fps_1_percent_low();
        // 1% low should be affected by the slow frame
        assert!(fps_1_low < 60.0);
        assert!(fps_1_low > 0.0);
    }
}
