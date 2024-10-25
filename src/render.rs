pub fn replace_text(text: String) -> String {
    let mut text = text;
    text = text.replace("\n", "<br>");
    text
}