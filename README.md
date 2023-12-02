# CSE411 - Advanced Programming Techniques

# Homework 4

**Due Date: 11/14/2023 EOD**

## Learning Outcomes:

- Use a GUI framework to implement a data visualization user interface
- Deploy an application across different platforms (Windows, Mac, Linux, Web)
- Understand different GUI programming tecniques

## Background:

In this programming assignment, you will develop a Rust application with a graphical user interface (GUI) using the egui crate. The application will allow users to open a file containing data, display this data in the GUI, edit the data, add to the dataset, and use the data to drive a simulation, such as an N-body simulation. The user will be able to configure simulation parameters, run the simulation, and visualize the results in a graph. Additionally, the application should support saving the simulation results to disk.

## Assignment Description:

You are tasked with creating a Rust application that provides a comprehensive user interface for data manipulation, simulation, and visualization. This application will follow these key steps:

### Data Import and Export:

- The application should allow users to open a file containing data. The data can be in a plain text format, and the application should be able to parse it.
- Enable users to save the results of the simulation to a file for future reference.

### Data Display and Editing:

- Display the data from the file in the GUI, allowing users to view and edit it.
- Users should be able to add new data points or modify existing ones.

### Simulation:

- Implement a data-driven simulation using the imported data. Consider using a simulation model like an N-body simulation, or choose a similar physics-based simulation.
- Allow users to configure simulation parameters, such as time steps, physical constants, or simulation duration.

### Simulation Execution:

- Provide buttons and controls for users to start and stop and configure the simulation.
- Display real-time visualizations or feedback during the simulation's execution.

### Data Visualization:

- Create a graph or chart that visualizes the results of the simulation.
- Ensure the graph is updated in real-time as the simulation progresses.

## Requirements:

To successfully complete this assignment, you must meet the following requirements:

- Implement file input/output for data import and export.
- Integrate a data-driven simulation, such as an N-body simulation or a physics-based model of your choice.
- Allow users to configure simulation parameters.
- Provide a real-time visualization of the simulation results.
- Ensure a user-friendly interface that enables data editing and interaction.
- The application must be able to be deployed across different platforms and hardware architectures.
- Implement appropriate error handling for file operations and user inputs.

## Grade Distribution:

Your grade for this assignment will be distributed as follows:

- Code Structure and Clarity (20%): The clarity of your code's structure and organization.
- Functionality (30%): How well your application meets the specified requirements, including data import/export, simulation, and user interaction.
- User Interface (15%): The usability and design of your GUI.
- Data Visualization (15%): The quality of the data visualization component.
- Error Handling (10%): The effectiveness of error handling for potential issues.
- Documentation and Comments (10%): The presence of helpful comments and documentation in the code.

## Submission:

For your project, please submit the source code and documentation in your repository, along with compiled binaries for the following platforms (Windows (x86), Linux (x86), Mac (x86/aarch64) and Web( wasm32).
