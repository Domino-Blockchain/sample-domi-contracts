#[link(wasm_import_module = "host")]
extern "C" {
    pub fn hello(p: i32);
}

#[no_mangle]
fn main() {
    unsafe { hello(35) };
}