use askama::Template;

#[derive(Template)]
#[template(path = "../templates/hello_world.html")]
pub struct HelloTemplate {
    pub time: String,
}

