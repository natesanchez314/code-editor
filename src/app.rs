use std::{collections::HashMap, fs::File};

use egui::Widget;

use crate::file_handler::{open_dir, open_file};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct MyApp {
	label: String,
	terminal_out: String,
	terminal_in: String,
	working_dir: String,
	open_files: Vec<String>,
	#[serde(skip)]
	value: f32
}

impl Default for MyApp {
	fn default() -> Self {
		Self {
			label: "Hello World!".to_owned(),
			terminal_in: "Terminal stuff".to_owned(),
			terminal_out: "Terminal out".to_owned(),
			working_dir: "/mnt/f80962e1-0834-440c-9374-0dd1dc213694/GitHub/shrimp-lang".to_owned(),
			open_files: Vec::new(),
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
					if ui.button("Open File...").clicked() {
						let file_name = "src/main.rs";
						let data = open_file(file_name.to_owned());
						//self.open_file_data.insert(file_name.to_owned(), data.clone());
						self.label.clear();
						self.label = data;
					}
					if ui.button("Open Folder...").clicked() {
						let read_dir = open_dir(&self.working_dir);
						println!("{:?}", read_dir);
					}
					if ui.button("Quit").clicked() {
						ctx.send_viewport_cmd(egui::ViewportCommand::Close);
					}
				});
				ui.menu_button("Edit", |_ui| {});
				ui.add_space(16.0);
				egui::widgets::global_dark_light_mode_buttons(ui);
			});
		});
		egui::TopBottomPanel::bottom("bottom_panel").resizable(true).show(ctx, |ui| {
			ui.collapsing("Terminal", |ui| {
				ui.available_height();
				ui.vertical_centered_justified(|ui| {
					//egui::ScrollArea::vertical().show(ui, |ui| {
						ui.add(
							egui::TextEdit::multiline(&mut self.terminal_out)
								.cursor_at_end(true)
								.desired_rows(8),
						);
					//});
				//ui.add(
				//	egui::TextEdit::singleline(&mut self.terminal_in),
				//);
				});
			});
		});
		egui::SidePanel::left("test").show(ctx, |ui| {
			ui.heading("Folders");
			egui::ScrollArea::both().stick_to_bottom(true).auto_shrink(false).show(ui, |ui| {
				ui.collapsing("Example Folder", |ui| {
					ui.collapsing("Example Subfolder", |ui| {
						if ui.selectable_label(false, "Example File 1").clicked() {}	
					});
					if ui.selectable_label(false, "Example File 2").clicked() {}	
				});
			});
		});

		let mut theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ctx);

		let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
            let mut layout_job =
                egui_extras::syntax_highlighting::highlight(ui.ctx(), &theme, string, "rs".into());
            layout_job.wrap.max_width = wrap_width;
            ui.fonts(|f| f.layout_job(layout_job))
        };

		egui::CentralPanel::default().show(ctx, |ui| {
			ui.horizontal_top(|ui| {
				ui.available_size();
				ui.vertical(|ui| {
					ui.horizontal(|ui| {
						if ui.selectable_label(false, "Example File").clicked() {}	
					});
					egui::ScrollArea::both().show(ui, |ui| {
						ui.add_sized(
						ui.available_size(),
							egui::TextEdit::multiline(&mut self.label)
								.code_editor()
								.layouter(&mut layouter),
						);
					})
				});
			});	
		});
	}
}

// struct Dir {
// 	dir_name: String,
// 	sub_dirs: Vec<Dir>,
// 	files: Vec<String>,
// }

// impl Dir {	
// 	fn ui(self, ui: &mut egui::Ui) -> egui::CollaResponse {
// 		ui.collapsing(self.dir_name, |ui| {
// 			ui.collapsing("Example Subfolder", |ui| {
// 				if ui.selectable_label(false, "Example File 1").clicked() {}	
// 			});
// 			if ui.selectable_label(false, "Example File 2").clicked() {}	
// 		})
// 	}
// }