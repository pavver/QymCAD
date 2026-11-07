//! THE PICKER OF THE REGIONS KEEPS ITS COMMAND: "Pick contours" in an extrusion opened for editing goes to the flat view
//! with the extrusion still in hand, its bar up and its sketch shown.
//!
//! Reported behaviour, with two pictures: an extrusion just made, opened for editing, then Alt+U / U / "Pick contours" -
//! the view went flat and that was all: no bar, no Apply, no Cancel, no region to pick, as if another mode. The editor
//! of a sketch's label, run on every flat frame, ended a text edit whenever the selection was not a sketch - and ending
//! it put the tool in hand down; an extrusion opened for editing has its node selected.
#[cfg(test)]
mod tests {
    use super::super::hand::Hand;

    /// The picker opened from the bar of a reopened extrusion, in the language in force.
    fn the_picker_keeps_the_extrusion(language: &str) {
        let mut app = super::super::screen_keys::tests::plate();
        qymcad_ui_state::rebuild_if_dirty(&mut app.rebuild_ctx());
        let fid = app.project.timeline.iter().find(|n| matches!(n.kind, qymcad_core::feature::FeatureKind::Extrude { .. })).map(|n| n.id).expect("setup: the extrusion");
        crate::gui::commands::start_feat_cmd_edit(&mut app.part_ctx(), fid);
        let mut hand = Hand::new(&mut app);
        hand.frame(Vec::new());
        assert_eq!(hand.app.tools.armed.part_tool(), Some(qymcad_ui_state::PartTool::Extrude), "{language}: setup: the extrusion is not reopened");
        // the bar is one row: what its words leave no room for is reached by the wheel, as a person reaches it
        let pick = crate::i18n::tr("cmd-pick-contours");
        let apply = crate::i18n::tr("cmd-apply-enter");
        let row = hand.words_drawn().iter().find(|(t, _)| t.contains(&apply)).map(|(_, r)| r.center().y).expect("setup: the bar ends with Apply");
        hand.wheel3d(egui::pos2(300.0, row), -8).settle();
        assert!(hand.press_word(&pick, egui::pos2(640.0, 24.0)), "{language}: the button of the regions is not in the bar");
        hand.frame(Vec::new());
        let applies = hand.shows(&apply);
        assert!(!hand.app.viewing.mode_3d, "{language}: the button did not open the flat picker");
        assert_eq!(hand.app.tools.armed.part_tool(), Some(qymcad_ui_state::PartTool::Extrude), "{language}: the picker put the extrusion down - the bare flat view of the report");
        assert!(applies, "{language}: the picker shows no bar with Apply");
        assert!(matches!(hand.app.chosen.sel, qymcad_ui_state::Sel::Sketch(_)), "{language}: the picker shows no sketch to pick a region of");
        assert!(!hand.app.tools.gsel.profiles.is_empty(), "{language}: the regions of the extrusion are not picked in the picker");
    }

    /// EVERY LANGUAGE, whatever the machine is set to: the width of the words decides what stands where in the bar.
    /// Reported behaviour: under Ukrainian the press of "Pick contours" came to the Cancel laid over it.
    #[test]
    fn the_picker_of_a_reopened_extrusion_keeps_the_extrusion_in_hand() {
        let prev = crate::i18n::language();
        for (language, _) in crate::i18n::available() {
            crate::i18n::set_language(&language);
            the_picker_keeps_the_extrusion(&language);
        }
        crate::i18n::set_language(&prev);
    }
}
