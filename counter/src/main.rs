use eframe::egui;

fn main() {
    let option = eframe::NativeOptions::default();
    eframe::run_native(
        "Counter",
        option,
        Box::new(|_cc| Box::new(MyAPP::default())),
    )
}

#[derive(Default)]
struct MyAPP {
    counter: i32,
}

impl eframe::App for MyAPP {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut i = 50;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.add_sized([60.0, 20.0], egui::Button::new("+")).clicked() {
                    self.counter += 1
                };
                ui.add_sized([60.0, 20.0], egui::Label::new(self.counter.to_string()));
                if ui.add_sized([60.0, 20.0], egui::Button::new("-")).clicked() {
                    self.counter -= 1
                };
            });

            ui.horizontal(|ui| {
                ui.add_sized([20.0, 80.0], egui::Label::new("Hello, world!"));
            });
        });
    }
}