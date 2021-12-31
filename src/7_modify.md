# Modify Commands

To be able to manipulate the list of creatures and colors redant creates a copy of the actual data files creatures and colors in `/home/user/` directory with the name of `.redant`. This is hidden directory. This directory is usually created only after running commands on this page.

### Creature

Add a creature to your local data: `redant creature add alien venom`

This will add two creatures to your local data: `alien` and `venom`. The color of the output of this command will tell you if add operation was successful as explained below.


### Color

Add a color to your local data: `redant color add newcolor1 newcolor2 newcolor3`

**Output:**

- Green: Added
- Red: Not added because already present

**Note:** You can add as many creatures or colors as you want in one command