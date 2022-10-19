extern crate time;
extern crate travelling_salesman;
wit_bindgen_rust::export!("tsp.wit");
struct Tsp;

impl tsp::Tsp for Tsp {
    fn tsp_of(length: i32, breath: i32) -> Vec<i32> {
        let length: usize = length as usize;
        let breath: usize = breath as usize;
        let tour = travelling_salesman::simulated_annealing::solve(
            &[
                (-74.006f64, 40.7128f64), 
                (-0.1278f64, 51.5074f64,)
            ],
            time::Duration::milliseconds(500),
          );
        
        println!("Tour distance: {}, route: {:?}", tour.distance, tour.route);
        println!("Distance matrix: {:?}", travelling_salesman::get_distance_matrix(&[
            (-74.006f64, 40.7128f64), 
            (-0.1278f64, 51.5074f64,)
         ]));
        tour.route.iter().map(|&e| e as i32).collect()
    }
}
