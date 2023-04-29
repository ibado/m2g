use std::process::exit;

use roxmltree::{Document, Node};

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("usage: m2g <dependencies>");
        exit(1);
    }

    match maven_to_gradle(args[1].clone()) {
        Ok(g) => {
            let r: Vec<_> = g.split("\n").into_iter().collect();
            println!("\nfor gradle: \n\t{}", r.join("\n\t"));
        }
        Err(e) => eprintln!("\nError: \n\t{e}"),
    };
}

fn maven_to_gradle(input: String) -> Result<String, String> {
    let curated = soround_str(input, "<p>", "</p>")
        .replace("\n", "")
        .replace(" ", "");
    let xml = Document::parse(&curated).map_err(|_| "Error parsing XML")?;

    let mut deps: Vec<String> = vec![];
    for node in xml.root_element().children() {
        if node.is_comment() {
            continue;
        }
        let d = get_dep(&node)?;
        deps.push(d);
    }

    Ok(deps.join("\n"))
}

fn get_dep<'a>(node: &Node<'a, 'a>) -> Result<String, String> {
    if node.tag_name().name() == "dependency" {
        let group = get_tag(node, "groupId")?;
        let artifact = get_tag(node, "artifactId")?;
        match get_tag(node, "version") {
            Ok(version) => Ok(format!("implementation \"{group}:{artifact}:{version}\"")),
            Err(_) => Ok(format!("implementation \"{group}:{artifact}\"")),
        }
    } else {
        Err("The provided string is not a correct maven dependency!".to_string())
    }
}

fn get_tag<'a>(root: &Node<'a, 'a>, name: &'static str) -> Result<&'a str, String> {
    root.children()
        .find(|child| child.tag_name().name() == name)
        .ok_or(format!("Missing {name}!"))?
        .text()
        .ok_or(format!("Invalid content for {name}!"))
}

fn soround_str(mut input: String, prefix: &str, suffix: &str) -> String {
    let mut out = prefix.to_string();
    input.push_str(suffix);
    out.push_str(&input[..]);

    out
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

    #[test]
    fn translate_multiple_dependencies() {
        let input = String::from(
            r#"
            <dependency>
                <groupId>com.dependencygroup</groupId>
                <artifactId>dependency1</artifactId>
                <version>1.0.0</version>
            </dependency>
            <!-- this is a comment -->
            <dependency>
                <groupId>com.dependencygroup</groupId>
                <artifactId>dependency2</artifactId>
                <version>0.1.2</version>
            </dependency>
            <dependency>
                <groupId>com.dependencygroup</groupId>
                <artifactId>dependency3</artifactId>
                <version>2.1.0</version>
            </dependency>
            "#,
        );

        assert_eq!(
            maven_to_gradle(input).unwrap(),
            "implementation \"com.dependencygroup:dependency1:1.0.0\"\nimplementation \"com.dependencygroup:dependency2:0.1.2\"\nimplementation \"com.dependencygroup:dependency3:2.1.0\""
        )
    }
}
