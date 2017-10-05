extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/foo.c")
        .file("src/bar.c")
        .compile("foobar");
}

