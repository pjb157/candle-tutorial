use candle_core::{DType, Device, Tensor};

const DEVICE: Device = Device::Cpu;

pub fn initialize_tensor() -> Result<(), anyhow::Error> {

    let data: [u32; 3] = [1u32, 2, 3];
    let tensor = Tensor::new(&data, &DEVICE)?;
    println!("tensor: {:?}", tensor.to_vec1::<u32>()?);

    let nested_data: [[u32; 3]; 3] = [[1u32, 2, 3], [4, 5, 6], [7, 8, 9]];
    let nested_tensor = Tensor::new(&nested_data, &DEVICE)?;
    println!("nested_tensor: {:?}", nested_tensor.to_vec2::<u32>()?);
    Ok(())
}
