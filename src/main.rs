use roxmltree::{Document, Node};

fn main() {
    let args: Vec<_> = std::env::args().collect();

    match maven_to_gradle(args[1].clone()) {
        Ok(g) => println!("\nfor gradle: \n\t{g}"),
        Err(e) => eprintln!("\nError: \n\t{e}"),
    };
}

fn maven_to_gradle(input: String) -> Result<String, String> {
    let curated = input.replace("\n", "").replace(" ", "");
    let xml = Document::parse(&curated).map_err(|_| "Error parsing XML")?;

    let root = xml.root_element();
    if root.tag_name().name() == "dependency" {
        let group = get_tag(&root, &"groupId")?;
        let artifact = get_tag(&root, &"artifactId")?;
        match get_tag(&root, &"version") {
            Ok(version) => Ok(format!("implementation \"{group}:{artifact}:{version}\"")),
            Err(_) => Ok(format!("implementation \"{group}:{artifact}\""))
        }
    } else {
        Err("The provided string is not a correct maven dependency!".to_string())
    }
}

fn get_tag<'a>(root: &Node<'a, 'a>, name: &'static str) -> Result<&'a str, String> {
    root.children()
        .find(|child| child.tag_name().name() == name)
        .ok_or(format!("Missing {name}"))?
        .text()
        .ok_or(format!("Invalid content for {name}"))
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
            maven_to_gradle(input).unwrap(),
            "implementation \"com.squareup.retrofit2:retrofit:34\""
        )
    }

    #[test]
    fn translate_single_dependency_without_version() {
        let input = String::from(
            r#"
            <dependency>
              <groupId>com.squareup.retrofit2</groupId>
              <artifactId>retrofit</artifactId>
            </dependency>
            "#,
        );

        assert_eq!(
            maven_to_gradle(input).unwrap(),
            "implementation \"com.squareup.retrofit2:retrofit\""
        )
    }

    #[test]
    fn translate_single_dependency_with_template_version() {
        let input = String::from(
            r#"
            <dependency>
              <groupId>com.squareup.retrofit2</groupId>
              <artifactId>retrofit</artifactId>
              <version>${version}</version>
            </dependency>
            "#,
        );

        assert_eq!(
            maven_to_gradle(input).unwrap(),
            "implementation \"com.squareup.retrofit2:retrofit:${version}\""
        )
    }
}
