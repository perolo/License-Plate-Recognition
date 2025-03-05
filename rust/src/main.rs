use opencv::core;
use opencv::imgcodecs::{have_image_reader, imread, imwrite, IMREAD_COLOR};
use opencv::imgproc::{bilateral_filter, canny, cvt_color, COLOR_BGR2GRAY};
use opencv::prelude::*;

//use opencv::types::VectorOfMat;

//fn main() -> Result<(), Box<dyn std::error::Error>> {
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    //files = ["image1", "image2", "image3", "image4"]
    let files = ["image1", "image2", "image3", "image4"];
    //let files = ["image1"];
    //for f in files:
    for f in files {
        //print("Hello from license-plate-recognition! file: " + f)
        println!("Hello from license-plate-recognition! file: {}", &f);
        //img = cv2.imread(f + ".jpg")
        //let fname = "../data/".to_owned() + f + ".jpg";
        let fname = format!("../data/{}.jpg", f);

        //have_image_reader
        have_image_reader(&fname)?;
        if let Ok(img) = imread(&fname, IMREAD_COLOR) {
            println!("Image loaded successfully!");

            // Create a new Mat object to store the grayscale image
            let mut gray_img = Mat::default();

            // Convert the image to grayscale using cvtColor
            cvt_color(&img, &mut gray_img, COLOR_BGR2GRAY, 0)?;

            // Save the grayscale image
            //plt.imshow(cv2.cvtColor(gray, cv2.COLOR_BGR2RGB))
            //cv2.imwrite(f + "_grey.jpg", gray)
            let oname = format!("../output/{}_gray.jpg", f);
            imwrite(&oname, &gray_img, &core::Vector::new())?;

            //bfilter = cv2.bilateralFilter(gray, 11, 17, 17)
            // Create an output Mat for the filtered image
            let mut bfilter = Mat::default();

            // Apply bilateral filtering
            bilateral_filter(
                &gray_img,
                &mut bfilter,
                11,
                17.0,
                17.0,
                core::BORDER_DEFAULT,
            )?;

            //edged = cv2.Canny(bfilter, 30, 200)
            // Create an output Mat for the edges
            let mut edged = Mat::default();

            // Apply Canny edge detection
            canny(&bfilter, &mut edged, 30.0, 200.0, 3, false)?;

            //plt.imshow(cv2.cvtColor(edged, cv2.COLOR_BGR2RGB))
            //cv2.imwrite(f + "_edged.jpg", edged)
            let ename = format!("../output/{}_edged.jpg", f);
            imwrite(&ename, &edged, &core::Vector::new())?;

            //keypoints = cv2.findContours(edged.copy(), cv2.RETR_TREE, cv2.CHAIN_APPROX_SIMPLE)
            //contours = imutils.grab_contours(keypoints)

            // Vector to store contours
            /*
                        let mut contours = opencv::types::VectorOfMat::new();
                        let mut hierarchy = Mat::default();

                        // Find contours
                        find_contours(
                            &edged,
                            &mut contours,
                            &mut hierarchy,
                            imgproc::RETR_TREE,
                            imgproc::CHAIN_APPROX_SIMPLE,
                            core::Point::new(0, 0),
                        )?;

                        // Convert `VectorOfMat` to `Vec<Mat>` for sorting
                        let mut contours_vec: Vec<Mat> = contours.to_vec();

                        //contours = sorted(contours, key=cv2.contourArea, reverse=True)[:10]
                        // Sort contours by area in descending order and keep the top 10
                        contours_vec.sort_by(|a, b| {
                            let area_a = imgproc::contour_area(a, false).unwrap_or(0.0);
                            let area_b = imgproc::contour_area(b, false).unwrap_or(0.0);
                            area_b.partial_cmp(&area_a).unwrap() // Reverse order for descending sort
                        });

                        let top_contours: Vec<Mat> = contours_vec.into_iter().take(10).collect();
            */
            //mask = np.zeros(gray.shape, np.uint8)
            //new_image = cv2.drawContours(mask, [location], 0, 255, -1)
            //new_image = cv2.bitwise_and(img, img, mask=mask)
            //cv2.imwrite(f + "_new_image.jpg", new_image)

            //location = None
            //let mut location: Option<Mat> = None;

            //for contour in contours:
            //    approx = cv2.approxPolyDP(contour, 10, True)
            //    if len(approx) == 4:
            //        location = approx
            //        break
            // Find the contour that approximates a quadrilateral
            /*
            for contour in &contours {
                let mut approx = Mat::default();
                approx_poly_dp(contour, &mut approx, 10.0, true)?;
                if approx.rows() == 4 {
                    location = Some(approx);
                    break;
                }
            }

            if let Some(location) = location {
                // Create a black mask with the same size as `gray`
                let mut mask = Mat::zeros(gray.rows(), gray.cols(), core::CV_8U)?.to_mat()?;

                // Draw the contour on the mask
                match draw_contours(
                    &mut mask,
                    &opencv::types::VectorOfMat::from_iter(vec![location.clone()]),
                    -1,
                    core::Scalar::new(255.0, 255.0, 255.0, 255.0),
                    -1,
                    imgproc::LINE_8,
                    &Mat::default(),
                    0,
                    core::Point::new(0, 0),
                ) {
                    Ok(_) => {
                        println!("Image conv OK!");
                    }
                    Err(_) => {
                        println!("failed to contour image!");
                        return;
                    }
                }

                // Apply bitwise AND to keep only the selected region
                let mut new_image = Mat::default();
                match bitwise_and(&img, &img, &mut new_image, &mask) {
                    Ok(_) => {
                        println!("Image conv OK!");
                    }
                    Err(_) => {
                        println!("failed to bitwand image!");
                        return Ok();
                    }
                }

                // Save the new image
                let nname = format!("../output/{}_new.jpg", f);
                match imwrite(&nname, &new_image, &core::Vector::new()) {
                    Ok(_) => {
                        println!("Image conv OK!");
                    }
                    Err(err) => {
                        println!("failed to write image!");
                        return err;
                    }
                }
            }
            } else {
                println!("failed to load image!");
            }
                */

            /*




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

                    cv2.imwrite(f + "_res.jpg", res)
            */
        }
    }
    Ok(())
}
