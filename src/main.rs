use roxmltree::Document;

fn main() {
    let args: Vec<_> = std::env::args().collect();

    let out = maven_to_gradle(args[1].clone());

    println!("\nfor gradle: \n\t{out}");
}

fn maven_to_gradle(input: String) -> String {
    let curated = input.replace("\n", "").replace(" ", "");
    let xml = match Document::parse(&curated) {
        Ok(d) => d,
        Err(e) => {
            println!("error: {}", e);
            std::process::exit(1)
        }
    };

    let children_attrs: Vec<_> = xml
        .root_element()
        .children()
        .filter_map(|child| child.text())
        .collect();

    let uri = children_attrs.join(":");
    format!("implementation '{uri}'")
}

#[cfg(test)]
mod tests {
    use crate::maven_to_gradle;

    #[test]
    fn translate_single_dependency() {
        let input = String::from(
            r#"
            <dependency>
              <groupId>com.squareup.retrofit2</groupId>
              <artifactId>retrofit</artifactId>
              <version>34</version>
            </dependency>
            "#,
        );

        assert_eq!(
            maven_to_gradle(input),
            "implementation 'com.squareup.retrofit2:retrofit:34'"
        )
    }
}
