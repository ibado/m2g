use roxmltree::{Document, Node};

fn main() {
    let args: Vec<_> = std::env::args().collect();

    let out = maven_to_gradle(args[1].clone());

    println!("\nfor gradle: \n\t{out}");
}

fn maven_to_gradle(input: String) -> String {
    let curated = input.replace("\n", "").replace(" ", "");
    let xml = match Document::parse(&curated) {
        Ok(doc) => doc,
        Err(e) => {
            println!("error: {}", e);
            std::process::exit(1)
        }
    };

    let root = xml.root_element();
    if root.tag_name().name() == "dependency" {
        let (group, artifact, version) = parse_dep(root).expect("Invalid dependency!");
        format!("implementation '{group}:{artifact}:{version}'")
    } else {
        println!("The provided string is not a dependency!");
        std::process::exit(1);
    }
}

fn parse_dep<'a>(root: Node<'a, 'a>) -> Option<(&str, &str, &str)> {
    let group_id = root
        .children()
        .find(|child| child.tag_name().name() == "groupId")?
        .text()?;
    let artifact_id = root
        .children()
        .find(|child| child.tag_name().name() == "artifactId")?
        .text()?;
    let version = root
        .children()
        .find(|child| child.tag_name().name() == "version")?
        .text()?;
    Some((group_id, artifact_id, version))
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
