//! Plots two lists of data (two vectors).

use plotlib::{
    page::Page,
    repr::Plot,
    view::ContinuousView,
    style::{
        PointStyle,
        PointMarker,
    },
};

use crate::Matrix;
use crate::error::*;

use super::StdFunc;

#[derive(Clone)]
pub struct Plt;

impl Plt {
    /// Evaluates `Plt` while minimizing heap allocation.
    pub fn evalpure(vec1: &Matrix, vec2: &Matrix) -> Matrix {
        let a = vec1.vals();
        let b = vec2.vals();

        if vec1.rows() != vec2.rows()
            || vec1.cols() != vec2.cols()
        {
            throw(ImproperDimensions);
            return Matrix::empty();
        }

        // Zip the two vector values into one vector of tuples
        let data = a.iter().zip(b).map(|(i, j)| (*i, *j)).collect::<Vec<(f64, f64)>>();

        // Create a new plot with the data and a custom point style
        let plot = Plot::new(data).point_style(
            PointStyle::new()
                .marker(PointMarker::Circle)
                .colour("#3264a8"),
        );

        // Plot the data
        let v = ContinuousView::new()
            .add(plot)
            .x_label("Independent")
            .y_label("Dependent");

        // Render the plot in the terminal
        match Page::single(&v).dimensions(150, 40).to_text() {
            Ok(p) => println!("{}", p),
            Err(_) => throw(CouldNotDisplayPlot),
        };
        
        // Save the plot to a file
        match Page::single(&v).save("plot.svg") {
            Ok(_) => (),
            Err(_) => throw(CouldNotWriteToFile),
        };

        Matrix::empty()
    }
}

impl StdFunc for Plt {
    fn eval(&self, args: Vec<Matrix>) -> Matrix {
        if args.len() != 2 {
            throw(WrongNumberOfArgs);
            return Matrix::new(0, 0, Vec::new());
        }

        Self::evalpure(&args[0], &args[1])
    }
}