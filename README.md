# WFRP-success-calculator
An API to rappidly calculate the success level of a warhammer fantasy rolplay d100 throw

### install rust

Follow the instructions (here)[https://www.rust-lang.org/tools/install]

### How to run

Execute the command `cargo run` to run the project, then do the calls to the url `http://localhost:3000`

To execute with hot reload run `cargo watch -x run` and proceede with the same URL

### TODO:
- [ ] Conenct to postgres DB
- [ ] Define the stats structure
- [ ] Create the Dockerfile image 
- [ ] Find were and how to deploy the image in a free way because I'm cheap
- [ ] API
    - [ ] Characters
        - [ ] create
        - [ ] modify
        - [ ] update
        - [ ] delete
    - [ ] Calculate roll
        - [ ] Throw (receives a character and a characteristic, the server generates a random number between 1 and 100, then calculates the success level and if is a hit calculates the place were it hit and if is crit or no)
        - [ ] Calculate-trhow (receives a character, a characteristic, and a d100 roll result the server generates a random number between 1 and 100, then calculates the success level and if is a hit calculates the place were it hit and if is crit or no)
