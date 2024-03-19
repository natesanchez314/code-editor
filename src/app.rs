#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct MyApp {
	label: String,
	#[serde(skip)]
	value: f32
}

impl Default for MyApp {
	fn default() -> Self {
		Self {
			label: "Hello World!".to_owned(),
			value: 2.7,
		}
	}
}

impl MyApp {
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		if let Some(storage) = cc.storage {
			return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
		}
		Default::default()
	}
}

impl eframe::App for MyApp {
	fn save(&mut self, storage: &mut dyn eframe::Storage) {
		eframe::set_value(storage, eframe::APP_KEY, self);
	}

	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			egui::menu::bar(ui, |ui| {
				ui.menu_button("File", |ui| {
					if ui.button("Quit").clicked() {
						ctx.send_viewport_cmd(egui::ViewportCommand::Close);
					}
				});
				ui.add_space(16.0);
				egui::widgets::global_dark_light_mode_buttons(ui);
			});
		});

		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("eframe template");
			ui.horizontal(|ui| {
				ui.label("Write something!");
				ui.text_edit_singleline(&mut self.label);
			});
		});
	}
}