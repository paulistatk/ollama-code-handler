This project is a Rust program that uses the Ollama API to convert files from one programming language to another. It has the following capabilities:

Get a list of supported tags from the API.
Generate a converted file from a given prompt.
Run the converted file and check its output.
Walk through a directory and process all files that are found.
For each file found, it reads the contents of the file, sends them to the API, and saves the generated output to a file with a .md extension.
The main function of the program is to walk through a directory, process each file found, and generate a converted file for each one. The generated file is then saved to a file with a .md extension.

The program uses the following libraries:

- ignore for walking through the directory and finding files.
- tokio for asynchronous programming.
- reqwest for making HTTP requests to the API.
- serde_json for parsing JSON responses from the API.
