use eframe::{egui, NativeOptions};
use eframe::egui::{Color32, Direction};

fn main() {
    let option = eframe::NativeOptions {
        initial_window_size: Some(egui::Vec2::new(320.0, 250.0)),
        ..NativeOptions::default()
    };
    eframe::run_native(
        "Calculator",
        option,
        Box::new(|_cc| Box::new(MyAPP::default())),
    )
}

#[derive(Default)]
struct MyAPP {
    result: String,
}

impl eframe::App for MyAPP {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::right_to_left()
                               .with_cross_align(egui::Align::TOP),
                           |ui| {
                               ui.heading(egui::RichText::new(self.result.to_string()).size(45.0));
                           });
            ui.with_layout(egui::Layout::left_to_right()
                               .with_cross_align(egui::Align::TOP)
                               .with_main_wrap(true),
                           |ui| {
                               if ui.add_sized([70.0, 30.0], egui::Button::new("1/x")).clicked() {
                                   self.result.insert_str(0, "1/")
                               };
                               if ui.add_sized([70.0, 30.0], egui::Button::new("x^2")).clicked() {
                                   self.result.push_str("^2")
                               };
                               if ui.add_sized([70.0, 30.0], egui::Button::new("x^(1/2)")).clicked() {
                                   self.result.push_str("^(1/2)")
                               };
                               if ui.add_sized([70.0, 30.0], egui::Button::new("÷")).clicked() {
                                   self.result.push_str("/")
                               };
                               if ui.add_sized([70.0, 30.0], egui::Button::new("7")).clicked() {
                                   self.result.push_str("7")
                               };
                               if ui.add_sized([70.0, 30.0], egui::Button::new("8")).clicked() {
                                   self.result.push_str("8")
                               };
                               if ui.add_sized([70.0, 30.0], egui::Button::new("9")).clicked() {
                                   self.result.push_str("9")
                               };
                               if ui.add_sized([70.0, 30.0], egui::Button::new("x")).clicked() {
                                   self.result.push_str("*")
                               };
                               if ui.add_sized([70.0, 30.0], egui::Button::new("4")).clicked() {
                                   self.result.push_str("4")
                               };
                               if ui.add_sized([70.0, 30.0], egui::Button::new("5")).clicked() {
                                   self.result.push_str("5")
                               };
                               if ui.add_sized([70.0, 30.0], egui::Button::new("6")).clicked() {
                                   self.result.push_str("7")
                               };
                               if ui.add_sized([70.0, 30.0], egui::Button::new("-")).clicked() {
                                   self.result.push_str("-")
                               };
                               if ui.add_sized([70.0, 30.0], egui::Button::new("1")).clicked() {
                                   self.result.push_str("1")
                               };
                               if ui.add_sized([70.0, 30.0], egui::Button::new("2")).clicked() {
                                   self.result.push_str("2")
                               };
                               if ui.add_sized([70.0, 30.0], egui::Button::new("3")).clicked() {
                                   self.result.push_str("3")
                               };
                               if ui.add_sized([70.0, 30.0], egui::Button::new("+")).clicked() {
                                   self.result.push_str("+")
                               };
                               if ui.add_sized([70.0, 30.0], egui::Button::new("C")).clicked() {
                                   self.result.clear()
                               };
                               if ui.add_sized([70.0, 30.0], egui::Button::new("0")).clicked() {
                                   self.result.push_str("0")
                               };
                               if ui.add_sized([70.0, 30.0], egui::Button::new(".")).clicked() {
                                   self.result.push_str(".")
                               };
                               if ui.add_sized([70.0, 30.0], egui::Button::new("=")).clicked() {
                                   let res = self.result.parse::<i32>();
                                   match res {
                                       Ok(f) => {
                                           self.result.clear();
                                           self.result.push_str(&f.to_string())
                                       },
                                       Err(_) => {
                                           self.result.clear();
                                           self.result.push_str("Error")
                                       }
                                   }
                               };
                           })
        });
    }
}