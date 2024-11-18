# modeseven

A simple wip of a car racing game in rust.

## How to clone

```bash
# Public code only
git clone https://github.com/staninna/modeseven.git

# With school content (needs permission)
git clone --recurse-submodules https://github.com/staninna/modeseven
# If there is no school directory run (idk why it doesn't clone automatically cuz .gitmodules is defined)
git submodule add --depth=1 https://github.com/staninna/modeseven-school.git school
#                     ^ depth=1 is used cuz binary blobs in git is a pain
```