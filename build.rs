fn main() {
    cc::Build::new()
        .cpp(true)
        .file("src/math.cpp")
        .compile("math");
}
