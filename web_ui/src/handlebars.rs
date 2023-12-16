use handlebars::Handlebars;

pub fn get_handlebars<'a>() -> Handlebars<'a> {
    let mut handlebars = Handlebars::new();

    let target_dir = "./src/templates/";

    let dir = std::fs::read_dir(target_dir).unwrap();

    for entry in dir.into_iter() {
        let path_buf = entry.unwrap().path();

        let file = path_buf.to_str().unwrap();

        if file.ends_with(".hbs") {
            let template_name = file
                .trim_start_matches(target_dir)
                .trim_end_matches(".hbs");

            handlebars.register_template_file(
                template_name, 
                file
            ).expect(&format!("Failed to register template {:?}", file));
        }
    }

    handlebars
}