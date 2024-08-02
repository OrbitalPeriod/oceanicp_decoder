mod uddf_elements;

fn main() {
    
}

pub fn get_deserializer(s: &str) -> serde_xml_rs::Deserializer<&[u8]> {
    serde_xml_rs::Deserializer::new_from_reader(s.as_bytes()).non_contiguous_seq_elements(true)
}

#[macro_export]
macro_rules! test_deserialization {
    ($type:ty, $xml:expr) => {
        #[cfg(test)]
        mod tests {
            #[test]
            fn deserialize() {
                let mut de = serde_xml_rs::Deserializer::new_from_reader($xml.as_bytes())
                    .non_contiguous_seq_elements(true);
                let result: Result<$type, _> = serde::Deserialize::deserialize(&mut de);
                if let Err(e) = result {
                    panic!("Error deserializing: {}", e);
                }
            }
        }
    };
}



//https://www.streit.cc/extern/uddf_v321/en/output.html
