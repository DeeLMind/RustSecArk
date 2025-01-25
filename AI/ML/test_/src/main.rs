use linfa::prelude::*;
use linfa_linear::LinearRegression;
use ndarray::{Array, Array1, Array2};

fn main() -> Result<()> {
    // Sample data - in real scenarios, you'd load this from a file or database
    let x = Array::from_shape_vec((4, 1), vec![1.0, 2.0, 3.0, 4.0])?;
    let y = Array1::from(vec![3.0, 5.0, 7.0, 9.0]);

    // Create a dataset from the data
    let dataset = Dataset::new(x, y);

    // Fit the linear regression model
    let model = LinearRegression::default().fit(&dataset)?;

    // Print the parameters of the model
    println!("Intercept: {:?}", model.params().intercept());
    println!("Coefficients: {:?}", model.params().weights());

    // Prepare new data for prediction
    let new_x = Array2::from_shape_vec((2, 1), vec![5.0, 6.0])?;

    // Use the model to make predictions
    let predictions = model.predict(&new_x);

    // Print predictions
    predictions.iter().enumerate().for_each(|(i, &pred)| {
        println!("Prediction for x = {}: {}", i + 5, pred);
    });

    Ok(())
}