# Colornames.org Visualizer
A simple LUT-style visualization generator for the colornames.org database.
Comes packaged with results and the CSV generated at 10/20/2022 10:28.

Check out the website [here](https://colornames.org). I am not affiliated with the website, I just like the concept, and am trying to assist.

# Usage
Go to https://colornames.org/download/ and download colornames.zip. Extract the text file and run the program with `<exec> <args>`. [database] is the extracted text file. Here are the args you can give:

`help`: Shows the help screen.

`gen [database]`: Generates a bmp file with the specified colornames.org database.

`diff [first] [second] [lut_bg|<any>|<none>] [print_diff|<any>|<none>] [only_diff|<any>|<none>]`: Differentiates two colornames.org CSV databases with the second database (supposedly) newer than the first.\
Green pixels appear for an added color, red for a removed color, white for colors in both databases, black/LUT val for colors in neither.\
You can supply a third argument (`black_bg`, `lut_bg`) that defines whether an LUT should be placed for colors in neither database, or a black background. Defaults to an LUT background.\
A fourth argument (`print_diff`, `no_print`) can be provided to define if new/removed values should be printed. Defaults to no printing.\
Another fifth argument (`only_diff`, `show_both`) can be specified to define if only changed values should be shown (values in both databases are set to black).

`detail [database] [color]`: Gets the details of a specified color (location on LUT, name of color). Inputs a hex code with nothing prepended or appended.

`random [count|<none>]`: Outputs a specified number of random colors. Defaults to 1 color.

`random_detail [database] [count|<none>]`: Gets the details of the specified number of random colors. Defaults to 1 color.
