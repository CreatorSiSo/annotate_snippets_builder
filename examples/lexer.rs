use annotate_snippets_builder::{Annotation, AnnotationType, DisplayList, Slice, Snippet};
use stringx::Join;

fn main() {
	let src_1 = include_str!("./lex_invalid.rym").lines().skip(2).join("\n");

	let mut snippet = Snippet::new().title(
		Annotation::new(AnnotationType::Error)
			.id("E4320")
			.label("Unexpected character"),
	);
	snippet
		.add_slice(
			Slice {
				source: &src_1,
				line_start: 3,
			}
			.origin("./lex_invalid.rym")
			.annotation((3, 4), AnnotationType::Error, "invalid")
			.annotation((26, 32), AnnotationType::Error, "invalid"),
		)
		.add_slice(
			Slice {
				source: "Ident::Test",
				line_start: 129,
			}
			.origin("src/display.rs")
			.annotation((0, 11), AnnotationType::Warning, "Unused"),
		);

	let dl = DisplayList::from(snippet);
	println!("\n{}", dl.to_string());
}
