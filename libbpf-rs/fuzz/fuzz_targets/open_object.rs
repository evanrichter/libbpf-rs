#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    libbpf_rs::set_print(None);
    if let Ok(obj) = libbpf_rs::ObjectBuilder::default().open_memory("fuzz.o", data) {
        let _maps: Vec<_> = obj.maps_iter().collect();
        let _progs: Vec<_> = obj.progs_iter().collect();
    }
});
