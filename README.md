# License Plate Detection Project

This project is a simple license plate detection system implemented in Python using OpenCV, EasyOCR, and Matplotlib libraries.

## Introduction

The goal of this project is to refresh python, learn how to use uv and to oxidize to rust.

## Prerequisites

### Installing Python

```bash
nix develop
```

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
