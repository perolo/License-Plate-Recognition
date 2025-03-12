import pytest
from main import process_image

@pytest.mark.parametrize("filename,write_intermediate,expected", [
    ("image1", False, "HR.26 BR.9044"),
    ("image2", False, "ICOVIDT9"),
    ("image3", False, "EBJY. 982]"),
    ("image4", False, "H982 FKL"),
    ("image_a", False, "CFM 882"),
    ("image_b", False, "CFM 882"),
    ("image_c", False, "CFM 882"),
    ("image_d", False, "CFM 882"),
    ("image_e", False, "CFM 882"),
    
])
def test_process_image(filename, write_intermediate, expected):
    print("Processing: " + filename)
    result = process_image(filename, write_intermediate)
    print("OCR Result: " + result)
    if result != expected:
        print("Expected "+ expected + "Found :" + result + ":")
    assert result == expected
