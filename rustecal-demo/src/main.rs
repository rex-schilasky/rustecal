fn main() {
    rustecal::Ecal::initialize(Some("rust_test_node")).expect("eCAL init failed");
    println!("eCAL started ✅");
    rustecal::Ecal::finalize();
}