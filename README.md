# Reporter - a tool for quickly assembling PDFs 
Simple tool written in rust to automatically create PDF files with images
# How to use: 
- compile the executable 
- copy the executable to the desired directory
- provide a file with exercises ordered from first to last, a folder with images in the PNG format, with a numeric filename corresponding to the exercise (1.png for the first exercise)
- copy the preamble pkg.tex file from the extras folder to the desired directory
- run the executable providing the title, author, tex preamble and the exercise file 
The tool should provide a pdf file 

 # Current limitations:
- can only have one image per exercise (to be fixed)
- only works on Linux (and probably MacOS) (may fix)

# Build Instruction (ubuntu)
```
#Install the dependencies:
sudo apt update -y && sudo apt install -y texlive latexmk rustc cargo git
#Clone and cd into the directory 
git clone https://github.com/RawHav0kk/reporter && cd ./reporter
#Run the compiler 
cargo build --release
#File will be in target/release
```
# Help page 
```
./reporter --help
Usage: reporter --author <AUTHOR> --title <TITLE> --preamblefile <PREAMBLEFILE> --exercisefile <EXERCISEFILE>

Options:
  -a, --author <AUTHOR>
  -t, --title <TITLE>
  -p, --preamblefile <PREAMBLEFILE>
  -e, --exercisefile <EXERCISEFILE>
  -h, --help                         Print help
  -V, --version                      Print version
  ```
