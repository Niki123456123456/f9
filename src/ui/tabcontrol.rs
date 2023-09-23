use egui::*;

pub fn show_tabs<T>(
    ui: &mut Ui,
    items: &mut Vec<T>,
    selected_index: &mut usize,
    get_name: impl Fn(&T) -> String,
    create_new: impl Fn() -> T,
    show_tab: impl Fn(&mut Ui, &mut T) -> (),
) 
{
    ui.spacing_mut().item_spacing = egui::vec2(0., 0.);

    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            let mut document_to_remove = None;

            for (i, item) in items.iter().enumerate() {
                if ui.button((get_name)(item)).clicked() {
                    *selected_index = i;
                }
                if ui.button("x").clicked() {
                    document_to_remove = Some(i);
                }
            }

            if ui.button("+").clicked() {
                items.push((create_new)());
                *selected_index = items.len() - 1;
            }

            if let Some(index) = document_to_remove {
                items.remove(index);
                if items.len() == 0 {
                    items.push((create_new)());
                }
                if *selected_index >= items.len() {
                    *selected_index = items.len() - 1;
                }
            }
        });

        (show_tab)(ui, &mut items[*selected_index]);
    });
}
