use crate::hex_to_decimal;
use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::view::ContinuousView;
use plotlib::style::LineStyle;

use crate::tracker::types::*;

pub async fn line_plot_and_write (name: String, storage: StateChangeList) -> Result<(), Box<dyn std::error::Error>> {
	// Awful
	let state_list = storage.state_changes.iter().map(|state| (state.block_number.as_u64() as f64, hex_to_decimal(&state.value).unwrap() as f64)).collect::<Vec<(f64, f64)>>();

	let plot = Plot::new(state_list).line_style(
		LineStyle::new()
			.colour("#DD3355")
			.width(2.0)
	);

	let v = ContinuousView::new()
		.add(plot)
		.x_label("Block Number, Made with sothis")
		.y_label("Value")
		.x_max_ticks(1)
		.y_max_ticks(1);

	Page::single(&v).save(name)?;

	Ok(())
}