pub struct SourceGenerator { }

impl SourceGenerator {
    pub fn generate(&self, squashed: Vec<u8>) -> String {
        let joined = squashed
            .into_iter()
            .map(|value| value.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        return format!("unsigned char sprites[] = {{{}}};", joined);
    }
}
