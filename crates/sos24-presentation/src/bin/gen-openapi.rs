use sos24_presentation::route::ApiDoc;
use utoipa::OpenApi;

fn main() {
    let doc = generate_openapi();
    print!("{doc}");
}

fn generate_openapi() -> String {
    ApiDoc::openapi().to_yaml().unwrap()
}
