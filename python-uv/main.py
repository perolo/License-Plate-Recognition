import cv2
from matplotlib import pyplot as plt
import numpy as np
import imutils
import easyocr

def main():
    files = ["image1", "image2", "image3", "image4"]
    for f in files:
        print("Hello from license-plate-recognition! file: " + f)
        img = cv2.imread("../data/" + f + ".jpg")
        gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)
        plt.imshow(cv2.cvtColor(gray, cv2.COLOR_BGR2RGB))
        cv2.imwrite("../output/" + f + "_grey.jpg", gray)

        bfilter = cv2.bilateralFilter(gray, 11, 17, 17)
        edged = cv2.Canny(bfilter, 30, 200)
        plt.imshow(cv2.cvtColor(edged, cv2.COLOR_BGR2RGB))
        cv2.imwrite("../output/" + f + "_edged.jpg", edged)

        keypoints = cv2.findContours(edged.copy(), cv2.RETR_TREE, cv2.CHAIN_APPROX_SIMPLE)
        contours = imutils.grab_contours(keypoints)
        print(len(contours))
        contours = sorted(contours, key=cv2.contourArea, reverse=True)[:10]
        for contour in contours:
            print(len(contour))

        location = None
        for contour in contours:
            approx = cv2.approxPolyDP(contour, 10, True)
            if len(approx) == 4:
                location = approx
                print("Location found: " + str(location))
                break

        mask = np.zeros(gray.shape, np.uint8)
        new_image = cv2.drawContours(mask, [location], 0, 255, -1)
        cv2.imwrite("../output/" + f + "_mask.jpg", new_image)
        
        new_image = cv2.bitwise_and(img, img, mask=mask)        
        cv2.imwrite("../output/" + f + "_new_image.jpg", new_image)

        plt.imshow(cv2.cvtColor(new_image, cv2.COLOR_BGR2RGB))

        (x,y) = np.where(mask==255)
        (x1, y1) = (np.min(x), np.min(y))
        (x2, y2) = (np.max(x), np.max(y))
        cropped_image = gray[x1:x2+1, y1:y2+1]

        plt.imshow(cv2.cvtColor(cropped_image, cv2.COLOR_BGR2RGB))

        reader = easyocr.Reader(['en'])
        result = reader.readtext(cropped_image)
        result

        text = result[0][-2]
        font = cv2.FONT_HERSHEY_SIMPLEX
        res = cv2.putText(img, text=text, org=(approx[0][0][0], approx[1][0][1]+60), fontFace=font, fontScale=1, color=(0,255,0), thickness=2, lineType=cv2.LINE_AA)
        res = cv2.rectangle(img, tuple(approx[0][0]), tuple(approx[2][0]), (0,255,0), 3)
        plt.imshow(cv2.cvtColor(res, cv2.COLOR_BGR2RGB))

        cv2.imwrite("../output/" + f + "_res.jpg", res)

if __name__ == "__main__":
    main()
