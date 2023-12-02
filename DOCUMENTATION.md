# Code Documentation

## Introduction

This documentation provides an overview and explanation of the Rust code implementing a planetary simulation application using the [eframe](https://github.com/emilk/eframe) and [egui](https://github.com/emilk/egui) libraries. The simulation involves modeling the motion and interactions of celestial bodies in a simplified solar system.

## Table of Contents

1. [Usage and Project Requirements] (#usage-and-project-requirements)
2. [Constants and Static Data](#constants-and-static-data)
3. [Application State Struct](#application-state-struct)
4. [Application Implementation](#application-implementation)
5. [Simulation Functions](#simulation-functions)
6. [Utility Functions](#utility-functions)

## 1. Usage and Project Requirements <a name="usage-and-project-requirements"></a>

The executables for this project are located in the executable folder as follows:
- Windows `homework-4.exe`
- Linux `hw4_linux`
- MacOs `hw4_mac`

The data folder contains the data saved from the simulation. In the simulation itself, they can be found under the `Open` menu option, and can be added to with the `Save` menu option. Both `Open` and `Save` are under `File`

The simulation can be run either by running `cargo run` from the root folder, or by running the executable from the necessary operating system

The simulation data can be updated in the control panel, and then saved as a .json file to be reuploaded from the menu

## 2. Constants and Static Data <a name="constants-and-static-data"></a>

This section contains constants, static data, and initial conditions for the simulation. Key elements include:

- **Constants:**
  - `SOLAR_MASS`: Mass of the Sun.
  - `YEAR`: Duration of one Earth year in simulation time.
  - `N_BODIES`: Number of celestial bodies in the simulation.

- **Static Data:**
  - `PLANET_NAMES`: Names of the celestial bodies.
  - `PLANET_COLORS`: Colors assigned to each celestial body.

- **Simulation Scale Parameters:**
  - `SCALE_FACTOR`: Scaling factor for positioning celestial bodies.
  - `OFFSET_X` and `OFFSET_Y`: Offset values for adjusting initial positions.

- **Initial Conditions:**
  - `PLANETS`: Array of `Object` structs representing initial positions, velocities, masses, and other properties of celestial bodies.

## 3. Application State Struct <a name="application-state-struct"></a>

The `TemplateApp` struct represents the state of the application. It includes fields for managing the simulation, input files, and other parameters. Key elements include:

- **Fields:**
  - `planets`: A vector of `Object` structs representing the current state of celestial bodies.
  - `input_files`: A vector of file paths for storing and loading simulation data.
  - `data_file`: The default data file used for initializing the simulation.
  - `save_count`: Counter for generating unique filenames during save operations.
  - `paused`: Flag indicating whether the simulation is paused.

- **Default Implementation:**
  - The `Default` trait is implemented for initializing default values for a new instance of `TemplateApp`.

## 4. Application Implementation <a name="application-implementation"></a>

This section includes the main implementation of the application using the `eframe` and `egui` libraries. The `TemplateApp` struct implements the `eframe::App` trait, defining methods for initialization, updating, and saving application state.

- **Initialization:**
  - The `new` method initializes the application, loading previous state if available.

- **Update Method:**
  - The `update` method defines the layout and behavior of the graphical user interface (GUI). It is called each time the UI needs repainting.

- **Save Method:**
  - The `save` method is called by the framework to save the application state before shutdown.

## 5. Simulation Functions <a name="simulation-functions"></a>

This section includes functions responsible for advancing the simulation in time.

- **`offset_momentum` Function:**
  - Updates the velocity of the central body (Sun) to ensure overall momentum conservation.

- **`advance` Function:**
  - Advances the simulation by iterating over time steps and updating the positions and velocities of celestial bodies based on gravitational interactions.

- **`energy` Function:**
  - Calculates the total energy of the system, considering kinetic and potential energy.

## 6. Utility Functions <a name="utility-functions"></a>

This section includes utility functions for reading and writing simulation data.

- **`read_data` Function:**
  - Reads simulation data from a JSON file, deserializes it, and updates the `planets` vector.

- **`write_data` Function:**
  - Writes simulation data to a JSON file after converting units to the original scale.

- **`shift_mut_ref` Function:**
  - Pops a mutable reference off the head of a slice, mutating the slice to no longer contain the mutable reference.
