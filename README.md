# ModeSeven

A simple work-in-progress car racing game written in Rust.

## Setup

### Cloning the Repository

```bash
# Public code only
git clone https://github.com/staninna/modeseven.git

# With school content (requires permission)
git clone --recurse-submodules https://github.com/staninna/modeseven

# If school directory is missing (submodule manual setup)
git submodule add --depth=1 https://github.com/staninna/modeseven-school.git school
# Note: depth=1 is used to minimize repository size due to binary files
```

### Running the Game

```bash
cargo run --release
```

<!-- Links:
http://www.extentofthejam.com/WheelsPage/

-->