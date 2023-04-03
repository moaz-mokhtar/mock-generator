use crate::models::Object;
use log::info;

#[test]
fn test_is_mocks_generate_required_count_objects() {
    use rand::Rng;

    let mut rng = rand::thread_rng();

    let count: i32 = rng.gen_range(0..1_000);

    info!(
        "Test Function: test_is_mocks_generate_required_count_objects. Count: {}",
        count
    );

    let (object, duration) = Object::mocks(count);
    assert_eq!(object.len(), count as usize);

    info!("|\tCost (nanoseconds): {:?}", duration.as_nanos());
    info!("|\tCost (seconds): {:?}", duration.as_secs_f64());
}
