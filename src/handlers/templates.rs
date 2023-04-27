use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "home.stpl")]
pub struct Home {
}

#[derive(TemplateOnce)]
#[template(path = "tags.stpl")]
pub struct Tags {
}
