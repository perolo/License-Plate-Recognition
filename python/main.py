import cv2
from matplotlib import pyplot as plt
import numpy as np
import imutils
import easyocr

def process_image(filename: str, write_intermediate: bool = False):
    text : str = "OCR Failed!"
    img = cv2.imread("../data/" + filename + ".jpg")
    gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)
    if write_intermediate:
        plt.imshow(cv2.cvtColor(gray, cv2.COLOR_BGR2RGB))
        cv2.imwrite("../output/" + filename + "_grey.jpg", gray)

    bfilter = cv2.bilateralFilter(gray, 11, 17, 17)
    edged = cv2.Canny(bfilter, 30, 200)
    if write_intermediate:
        plt.imshow(cv2.cvtColor(edged, cv2.COLOR_BGR2RGB))
        cv2.imwrite("../output/" + filename + "_edged.jpg", edged)

    keypoints = cv2.findContours(edged.copy(), cv2.RETR_TREE, cv2.CHAIN_APPROX_SIMPLE)
    contours = imutils.grab_contours(keypoints)
    contours = sorted(contours, key=cv2.contourArea, reverse=True)[:10]

    location = None
    for contour in contours:
        approx = cv2.approxPolyDP(contour, 10, True)
        if len(approx) == 4:
            location = approx
            break

    mask = np.zeros(gray.shape, np.uint8)
    new_image = cv2.drawContours(mask, [location], 0, 255, -1)
    if write_intermediate:
        cv2.imwrite("../output/" + filename + "_mask.jpg", new_image)

    new_image = cv2.bitwise_and(img, img, mask=mask)        
    if write_intermediate:
        cv2.imwrite("../output/" + filename + "_new_image.jpg", new_image)
        plt.imshow(cv2.cvtColor(new_image, cv2.COLOR_BGR2RGB))

    (x,y) = np.where(mask==255)
    (x1, y1) = (np.min(x), np.min(y))
    (x2, y2) = (np.max(x), np.max(y))
    cropped_image = gray[x1:x2+1, y1:y2+1]

    if write_intermediate:
        plt.imshow(cv2.cvtColor(cropped_image, cv2.COLOR_BGR2RGB))

    reader = easyocr.Reader(['en'])
    result = reader.readtext(cropped_image)
    text = result[0][-2]
    return text


def main():
    #files = ["image1", "image2", "image3", "image4", "image_a", "image_b", "image_c", "image_d", "image_e"]
    files = ["image1", "image2", "image3", "image4"]
    write_intermediate : bool = False
    for f in files:
        txt = process_image(f, write_intermediate)
        print("OCR Result: " + txt)


if __name__ == "__main__":
    main()
