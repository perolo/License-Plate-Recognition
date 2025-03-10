# License Plate Detection Project

This project is a simple license plate detection system implemented in Python using OpenCV, EasyOCR, and Matplotlib libraries.

## Introduction

The goal of this project is to refresh python, learn how to use uv and to oxidize to rust.

Planned tasks:
* [x] clone project
* [x] Build Python + replicate results
  * [x] Using Jupiter
  * [x] Using separate main.py
* [ ] Create nix environment with Python and uv
  * Not able to create stable environment - intermittent problems with building the environment with "nix develop"
  * Not able to use uv as package manager, debugging fails
* [ ] Create nix environment with rust
  * Not able to create stable environment - Not able to build opencv under NixOs
* [x] Create a separate rust docker builder with opencv src/Dockerfile
  * [x] First translation into rust
  * [ ] Rewrite all into Rust
* [x] Create a separate python + uv docker builder with opencv python/Dockerfile
  * [x] Compare pip and uv
* [ ] Performance comparison Rust/Python

## Prerequisites

### Building using Docker

```bash
$ docker run -it -v ./:/src rustcv  bash
$ cd src
$ cargo build

```

### Installing Python in Nix

```bash
nix develop
```
* [ ] Failing to write flake.nix

### Installing libraries

```bash
uv sync
```

## Usage

1. Clone the repository to your local machine:
   ```bash
   git clone https://github.com/your-username/license-plate-detection.git
   ```

2. Navigate to the project directory:
   ```bash
   cd license-plate-detection
   ```

3. Run the main Python script:
   ```bash
   python main.py
   ```

4. The script will process the input image, detect license plates, and display the result with the extracted plate numbers.

## Performance

### Python

```bash
$ docker build -t python python/.
$ docker run -it -v ./:/src python  bash
$ time python3 main.py
OCR Result: HR.26 BR.9044
Neither CUDA nor MPS are available - defaulting to CPU. Note: This module is much faster with a GPU.
OCR Result: ICOVIDT9
Neither CUDA nor MPS are available - defaulting to CPU. Note: This module is much faster with a GPU.
OCR Result: EBJY. 982]
Neither CUDA nor MPS are available - defaulting to CPU. Note: This module is much faster with a GPU.
OCR Result: H982 FKL

real	0m8.363s
user	0m8.657s
sys	  0m1.022s

```


### Rust 

```bash
$ docker build -t rustcv rust/.
$ docker run -it -v ./:/src rustcv  bash
$ time cargo run
  OCR Result: THR 26.BR 9044,
  OCR Result: *COVIDI9
  OCR Result: BUY -982|
  OCR Result: HS82 FKL

real	0m0.458s
user	0m0.739s
sys 	0m0.064s
```



## Workflow

1. **Image Input**: The system takes an input image containing vehicles with license plates.

2. **License Plate Detection**: OpenCV is used to detect the location of license plates within the image using image processing techniques such as edge detection and contour analysis.

3. **License Plate Recognition**: EasyOCR is employed to recognize and extract the text from the detected license plates.

4. **Result Visualization**: Matplotlib is used to visualize the original image with the detected license plates and the extracted license plate numbers.

## File Structure

- `license_plate_detection.py`: Main Python script for the license plate detection system.
- `input_image.jpg`: Sample input image for testing the system.

## Acknowledgements

- [OpenCV](https://opencv.org/): Open Source Computer Vision Library
- [EasyOCR](https://github.com/JaidedAI/EasyOCR): Ready-to-Use OCR with 40+ languages supported
- [Matplotlib](https://matplotlib.org/): Python plotting library

## License

This project is licensed under the MIT - see the [LICENSE](LICENSE) file for details.

## Contributing

Feel free to contribute to this project by forking the repository and submitting a pull request. Any suggestions and improvements are welcome.
