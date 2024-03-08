use candle_core::{DType, Device, Tensor};
use candle_roberta::initialize_tensor;

fn main() {
    initialize_tensor().unwrap();
}