import os

source = r"""
use util::load;

use std::io;

fn main() -> io::Result<()> {{
    let _contents = load("src/day{0}/input.txt")?;

    Ok(())
}}

# [cfg(test)]
mod tests {{

    # [test]
    fn test() {{

    }}
}}
"""

for day in range(1, 26):
    folder = os.path.join("src", "day{}".format(day))
    main = source.format(day)
    main_path = os.path.join(folder, "main.rs")
    input_path = os.path.join(folder, "input.txt")

    os.makedirs(folder)
    with open(main_path, 'w') as f:
        f.write(main)
    with open(input_path, 'w') as f:
        f.write('')

    print("[[bin]]")
    print('name = "day{}"'.format(day))
    print('path = "src/day{}/main.rs"\n'.format(day))
