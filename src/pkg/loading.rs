use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};

pub struct Loading{
    pub is_loading: bool,
    pb : ProgressBar,
}
impl Loading{
    pub fn new() -> Self{
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(Duration::from_millis(120));
        pb.set_style(
            ProgressStyle::with_template("{spinner:.cyan} {msg}")
                .unwrap()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏", "✓"]),
        );
        pb.set_message("Loading...");
        Self{is_loading:true,pb}
    }
    pub fn final_loading(&mut self) {
        if self.is_loading {
            self.is_loading=false;
            self.pb.finish_and_clear();
        }
    }

}