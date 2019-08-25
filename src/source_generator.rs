pub struct SourceGenerator { }

impl SourceGenerator {
    pub fn generate(&self, squashed: Vec<u8>, variable_number: u32) -> String {
        let joined = squashed
            .into_iter()
            .map(|value| value.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        return format!("unsigned char sprites{}[] = {{{}}};", variable_number, joined);
    }
}
