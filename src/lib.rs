use pyo3::prelude::*;
use numpy::PyReadonlyArray3;
use ndarray::ArrayView3;
use lane_detect::detect_lanes;
use ndarray::Array2;

#[pyfunction]
fn detect_lanes_rust<'py>(py:Python<'py>, frame:PyReadonlyArray3<'_, u8>,) -> PyResult<&'py PyArray2<f64>> {
    let frame_view: ArrayView3<u8> - > frame.as_array();
    let lines = detect_lanes(&frame_view)?;

    // Converting Vec<Line> to 2d ndarray (num_lines x 4);
    let num_lines = lines.len();
    let mut data: Vec<f64> = Vec::with_capacity(num_lines * 4);
    for line in lines {
        data.push(line.0);
        data.push(line.1);
        data.push(line.2);
        data.push(line.3);
    }
    let arr = Array2::from_shape_vec((num_lines as usize, 4), data)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Array creation failed : {}", e)))?;
        let py_array = arr.into_pyarray(py);;
        Ok(py_array)
}

#[pymodule]
fn adas_pilot(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(detect_lanes_rust, m)?)?;
    Ok(())
}