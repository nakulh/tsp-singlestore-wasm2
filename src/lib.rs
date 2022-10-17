use rusticsom::SOM;
wit_bindgen_rust::export!("tsp.wit");
struct Tsp;

impl tsp::Tsp for Tsp {
    fn tsp_of(length: i32, breath: i32) -> bool {
        let length: usize = length as usize;
        let breath: usize = breath as usize;
        let mut map = SOM::create(length, breath, 2, false, None, None, None, None);
        true
    }
}
