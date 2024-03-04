use crate::patterns::validate;

mod patterns;
mod schema;
mod pre_fetch;
mod file_loading;


fn main() {
    let src = file_loading::load_file("sample.txt").unwrap();

    dbg!(
        pre_fetch::data_initialization(&src)
    );
}