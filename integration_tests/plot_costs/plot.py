import numpy as np
import pandas as pd
import plotly.graph_objects as go
import fire
from scipy.stats import linregress
from pathlib import Path


def load_data(path):
    data = pd.read_csv(path)
    return data


def linear_regression(x, y):

    x_transformed = np.log2(x)

    # Perform linear regression
    slope, intercept, r_value, p_value, std_err = linregress(x_transformed, y)
    print("Slope: ", slope)
    print("Intercept: ", intercept)
    print("R-squared: ", r_value ** 2)
    print("P-value: ", p_value)
    print("Standard error: ", std_err)
    return slope, intercept


def main(dir: str = "data", path: str = "insert_delete_costs.csv"):
    dir_path = Path(dir)
    path = dir_path / path
    data = load_data(path)
    np_x = data.iloc[:, 0].values
    np_y = data.iloc[:, 1].values
    slope, intercept = linear_regression(np_x, np_y)
    x_fit = np.logspace(0, 8)
    y_fit = slope * np.log2(x_fit) + intercept
    print(x_fit)

    scatter = go.Scatter(x=np_x, y=np_y, mode="lines", name="Inserts")
    # Calculate y values for the fitted line using the regression parameters and the inverse transformation of x
    y_fit = intercept + slope * np.log2(x_fit)

    # Create a line plot for the fitted line
    line = go.Scatter(x=x_fit, y=y_fit, mode="lines", name="Fitted Line")
    max_power = int(np.log2(max(np_x)))

    fig = go.Figure(data=[scatter, line])
    fig.update_xaxes(type="log")
    fig.update_layout(
        title="Random insert into and delete on the avltree",
        xaxis_title="Number of random inserts",
        yaxis_title="Costs in XRD",
        legend_title="Legend",
    )
    # Generate powers of 2 from 2^2 onwards, stopping before exceeding the max x value
    # line_positions = [2 ** p for p in range(2, max_power + 1) if 2 ** p <= max(np_x)]
    # # Add vertical lines for each calculated position
    # for pos in line_positions:
    #     fig.add_shape(
    #         type="line",
    #         x0=pos,
    #         y0=0,
    #         x1=pos,
    #         y1=1,
    #         line=dict(color="green", width=2),
    #         xref="x",
    #         yref="paper",
    #     )  # xref set to "x" to use the x-axis values, yref set to "paper" to span the entire plot height
    # Combine the plots
    # Update layout

    # Show plot
    fig.write_image(dir_path / "plot.png")
    fig.write_html(dir_path / "plot.html")


if __name__ == "__main__":
    fire.Fire(main)
