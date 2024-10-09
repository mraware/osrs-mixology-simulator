# Brute-ish force

### Constraints
Current constraints set on the brute-ish force
- MAL is always the bottom row of the inventory (this makes the simulation take 1/10th the time to run)
- A mini-sim is run on every layout, this speeds up performance by reducing the number of total sims
- In a larger sim is run if the mini-sim performs well, if the larger sim beats the current top 10, it gets placed int he top 10
- Less mini-sims increases the chance of a good layout gets unlucky and is skipped, the larger sim prevents bad layouts from getting lucky and making it to the top

### Requirements
Rust and cargo: https://rustup.rs/


### Usage
To run `cargo run`

Setup configuration in `src/main.rs`


# Simulation comparison
### Requirements
Python 3: https://www.python.org/downloads/

### Usage
To run `python simulate.py`

Configuration in `simulate.py`


# Credits
Mitchram - creating the original python script to simulate, I wouldn't have made this if they didn't do all the hard work