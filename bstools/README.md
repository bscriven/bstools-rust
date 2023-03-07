# bstools

bstools is a command line utility that allows other command line utilities and scripts to be conveniently organized, located, and executed. Support is available to:
- Execute other executables
- Create easy to remember aliases for other commands
- Execute Python scripts
- Pass arguments to commands

Commands can be grouped into logical categories. For example, if you were to create a script called `generate_uuid.py` and place it in a directory called `dev` within the `python` directory of the bstools home directory, it would be possible to execute the script with the following command:

`bs dev generate_uuid.py`

If you were unable to remember the name of the script, you could execute the following command to see a listing of scripts available within the `dev` directory:

`bs dev`

Executing `bs` would display a listing of directories and commands available at the root level.

---

## Configuration

bstools will prompt you to create necessary environment variables.

### BS_HOME

The BS_HOME environment variable must contain a path to a directory where commands will be located. The following directories will be automatically created within the BS_HOME directory:
- commands
  - Stores aliases for existing commands.
- data
  - Stores data to be used by other command line utilities.
- executables
  - Stores executable files.
- python
  - Stores Python scripts.

### BS_PYTHON

The BS_PYTHON environment variable must contain a path to the Python executable to use when executing Python scripts.

---

## Examples

### Commands

The `commands` directory contains aliases for existing commands. For example, say the following file was created:

`[BS-HOME]/commands/network/ping_alias`

The `ping_alias` file could contain the following contents:

`ping -n %s -4 %s`

bstools will replace the `%s` tokens within the file with arguments supplied at the command line. The command could then be executed like so:

`bs network ping_alias 5 localhost`

bstools can also append arguments supplied at the command line to the command that it is executing. For example, the contents of the `ping_alias` file could be changed to the following:

`ping -n %s -4`

The `ping_alias` command could still be executed like so:

`bs network ping_alias 5 localhost`

### Executables

The `executables` directory stores other executable files to be executed by bstools. For example, say the following executable file existed at:

`[BS-HOME]/executables/tools/example.exe`

Say also that `example.exe` expected the following command line arguments:

`-v -s test1 test2`

bstools could execute `example.exe` with the following command:

`bs tools example.exe -v -s test1 test2`

### Python

The `python` directory stores Python scripts to be executed by bstools. For example, say the following Python script existed at:

`[BS-HOME]/python/dev/generate_uuid.py`

Say also that `generate_uuid.py` expected a command line argument that determined how many UUIDs to generate.

bstools could execute `generate_uuid.py` with the following command to generate 10 UUIDs:

`bs dev generate_uuid.py 10`

### Finding Commands

bstools will list the available directories and commands. Assuming all of the previous examples were available in the `[BS-HOME]` directory, executing `bs` would display the following options:
- dev
- network
- tools

Executing `bs dev` would display `generate_uuid.py` as an option.

Executing `bs network` would display `ping_alias` as an option.

Executing `bs tools` would display `example.exe` as an option.