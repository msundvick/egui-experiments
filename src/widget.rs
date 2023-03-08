use egui::Widget;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Default)]
enum JibForceModel {
    #[default]
    EmptyForceModel,
    DummyForceModel {
        InputForces: [f64; 3],
        InputMoments: [f64; 3],
    },
    JibForces {
        coe_e: [f64; 3],
        area: f64,
        coefficients: String,
        ref_height: f64,
    },
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone)]
struct JibProperties {
    pos_hounds_yacht: [f64; 3],
}
#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct JibElement {
    name: String,
    local_frame_origin: [f64; 3],
    mass: f64,
    cog: [f64; 3],
    inertia: [[f64; 3]; 3],
    force_model_config: JibForceModel,
    properties: JibProperties,
}

impl JibElement {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("Element");
        ui.horizontal(|ui| {
            ui.label("name: ");
            ui.text_edit_singleline(&mut self.name);
        });
        ui.horizontal(|ui| {
            ui.label("local frame origin: ");
            for i in 0..3 {
                ui.add(egui::DragValue::new(&mut self.local_frame_origin[i]));
            }
        });

        ui.label("inertia: ");
        for j in 0..3 {
            ui.horizontal(|ui| {
                for i in 0..3 {
                    self.inertia[j][i] = self.inertia[i][j];
                    ui.add(egui::DragValue::new(&mut self.inertia[j][i]));
                }
            });
        }
    }
}
