mod gen;
use gen::language;

fn main() {
    let s = language::GetRequiredPluginsRequest::new();
    println!("{}", s.pwd);
}
