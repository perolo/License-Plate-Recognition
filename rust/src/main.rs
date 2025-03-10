use opencv::core;
//use opencv::core::Point;
use opencv::core::{bitwise_and,  Mat, Rect, Scalar, CV_8UC1};
use opencv::imgcodecs::{have_image_reader, imencode, imread, imwrite, IMREAD_COLOR};
use opencv::imgproc::{
    approx_poly_dp, bilateral_filter, canny, contour_area, cvt_color, draw_contours, find_contours,
    CHAIN_APPROX_SIMPLE, COLOR_BGR2GRAY, RETR_TREE,
};
use opencv::prelude::*;

use opencv::core::find_non_zero;
use opencv::imgproc::bounding_rect;

//use pyo3::prelude::*;
//use pyo3::types::PyBytes;
use tesseract::Tesseract;
use opencv::core::Vector;
//use opencv::imgproc::rectangle;
//use opencv::imgproc::{FONT_HERSHEY_SIMPLEX, LINE_AA};

//use opencv::imgproc::put_text;

//use opencv::types::VectorOfMat;

//use opencv::types::VectorOfVectorOfPoint;

//use opencv::types::VectorOfMat;

//fn main() -> Result<(), Box<dyn std::error::Error>> {
fn main() -> Result<(), Box<dyn std::error::Error>> {
    //println!("Hello, world!");
    //files = ["image1", "image2", "image3", "image4"]
    let files = ["image1", "image2", "image3", "image4"];
    //let files = ["image1"];
    //for f in files:
    for f in files {
        //print("Hello from license-plate-recognition! file: " + f)
        //println!("Hello from license-plate-recognition! file: {}", &f);
        //img = cv2.imread(f + ".jpg")
        //let fname = "../data/".to_owned() + f + ".jpg";
        let fname = format!("../data/{}.jpg", f);

        //have_image_reader
        have_image_reader(&fname)?;
        if let Ok(img) = imread(&fname, IMREAD_COLOR) {
            //println!("Image loaded successfully!");

            // Create a new Mat object to store the grayscale image
            let mut gray_img = Mat::default();

            // Convert the image to grayscale using cvtColor
            cvt_color(&img, &mut gray_img, COLOR_BGR2GRAY, 0)?;

            // Save the grayscale image
            //plt.imshow(cv2.cvtColor(gray, cv2.COLOR_BGR2RGB))
            //cv2.imwrite(f + "_grey.jpg", gray)
            let oname = format!("../output/{}_gray2.jpg", f);
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
            let ename = format!("../output/{}_edged2.jpg", f);
            imwrite(&ename, &edged, &core::Vector::new())?;

            //keypoints = cv2.findContours(edged.copy(), cv2.RETR_TREE, cv2.CHAIN_APPROX_SIMPLE)
            //contours = imutils.grab_contours(keypoints)

            // Vector to store contours

            //let mut contours = opencv::types::VectorOfMat::new();
            //let mut contours: Vec<Mat> = vec![];
            //let mut contours = Mat::default();
            //let mut contours: Vec<Vec<Point>> = vec![vec![]];
            //let mut contours = VectorOfVectorOfPoint::new();
            let mut contours: core::Vector<core::Vector<core::Point>> = core::Vector::new();
            //std::vector<std::vectorcv::Point >
            //let mut hierarchy = Mat::default();

            // Find contours
            find_contours(
                &edged,
                &mut contours,
                //&mut hierarchy,
                RETR_TREE,
                CHAIN_APPROX_SIMPLE,
                core::Point::new(0, 0),
            )?;
            /*
            println!("Contours found: {}", contours.len());
            let mut i = 0;
            for countour in &contours {
                println!("Countour: {}", countour.len());
                i += 1;
                if i > 10 {
                    break;
                }
            }*/

            // Convert `VectorOfMat` to `Vec<Mat>` for sorting
            let mut contours_vec: Vec<core::Vector<core::Point>> = contours.to_vec();

            //contours = sorted(contours, key=cv2.contourArea, reverse=True)[:10]
            // Sort contours by area in descending order and keep the top 10
            contours_vec.sort_by(|a, b| {
                let area_a = contour_area(a, false).unwrap_or(0.0);
                let area_b = contour_area(b, false).unwrap_or(0.0);
                area_b.partial_cmp(&area_a).unwrap() // Reverse order for descending sort
            });

            let top_contours: Vec<core::Vector<core::Point>> =
                contours_vec.into_iter().take(10).collect();

            //println!("top Contours found: {}", top_contours.len());
/*
            let mut i = 0;
            for countour in &top_contours {
                println!("Countour: {}", countour.len());
                i += 1;
                if i > 10 {
                    break;
                }
            }
*/
            //mask = np.zeros(gray.shape, np.uint8)
            let size = gray_img.size()?;
            let mut mask = Mat::new_size_with_default(size, CV_8UC1, Scalar::all(0.0))?;

            //location = None
            //let mut location: Option<Mat> = None;

            //new_image = cv2.drawContours(mask, [location], 0, 255, -1)
            //let location = VectorOfVectorOfPoint::new(); // Replace with actual location VectorOfPoint
            //let mut location: core::Vector<core::Point> = core::Vector::new();
            let mut location = Mat::default();

            //for contour in contours:
            //    approx = cv2.approxPolyDP(contour, 10, True)
            //    if len(approx) == 4:
            //        location = approx
            //        break
            // Find the contour that approximates a quadrilateral

            for contour in &top_contours {
                let mut approx = Mat::default();
                approx_poly_dp(&contour, &mut approx, 10.0, true)?;
                if approx.rows() == 4 {
                    location = approx;
                    //println!("Location found!: {:?}", location);
                    break;
                }
            }

            //new_image = cv2.drawContours(mask, [location], 0, 255, -1)
            //let mut wrapped_location: Vec<Mat> = core::Vector::new().into();
            //wrapped_location.push(location.clone());
            //let mut wrapped_location = Mat::default();
            //wrapped_location.push(&location)?;
            //let mut wrapped_location = VectorOfMat::new();
            //wrapped_location.push(location);
            //let mut wrapped_location = [[location.clone()]];
            let wrapped_location: core::Vector<Mat> =
                core::Vector::from(vec![location.clone()]);

            //println!("wrapped_location : {:?}", wrapped_location);

            let mut new_image = Mat::default();
            draw_contours(
                &mut mask,
                &wrapped_location,
                -1,
                Scalar::all(255.0),
                -1,
                opencv::imgproc::LINE_8,
                &Mat::default(),
                0,
                opencv::core::Point::new(0, 0),
            )?;
            let mname = format!("../output/{}_mask2.jpg", f);
            imwrite(&mname, &mask, &core::Vector::new())?;

            //new_image = cv2.bitwise_and(img, img, mask=mask)
            bitwise_and(&img, &img, &mut new_image, &mask)?;
            //cv2.imwrite(f + "_new_image.jpg", new_image)
            let nname = format!("../output/{}_new_image2.jpg", f);
            imwrite(&nname, &new_image, &core::Vector::new())?;

            //(x,y) = np.where(mask==255)
            //(x1, y1) = (np.min(x), np.min(y))
            //(x2, y2) = (np.max(x), np.max(y))
            //cropped_image = gray[x1:x2+1, y1:y2+1]

            // Find coordinates where mask == 255
            let mut non_zero_points = Mat::default();
            find_non_zero(&mask, &mut non_zero_points)?;

            // Convert non_zero_points to a vector of Point
            //let points: Vec<Vec<Point>> = non_zero_points.to_vec_2d()?;

            // Find the bounding rectangle
            let bounding_box: Rect = bounding_rect(&non_zero_points)?;

            // Crop the grayscale image using the bounding box
            let cropped_image = Mat::roi(&gray_img, bounding_box)?;

            let cname = format!("../output/{}_cropped_image2.jpg", f);
            imwrite(&cname, &cropped_image, &core::Vector::new())?;
            //println!("Cropped !: {:?}", location);
            /*
                        // Find coordinates where mask == 255
                        let (x, y): (Vec<usize>, Vec<usize>) = mask
                            .indexed_iter()
                            .filter(|&((_, _), &value)| value == 255)
                            .map(|((i, j), _)| (i, j))
                            .unzip();

                        // Find the bounding box coordinates
                        let x_min = *x.iter().min().unwrap();
                        let y_min = *y.iter().min().unwrap();
                        let x_max = *x.iter().max().unwrap();
                        let y_max = *y.iter().max().unwrap();

                        // Crop the grayscale image
                        //let cropped_image = gray.view(s![x1..=x2, y1..=y2]).to_owned();
                        let rect = Rect::new(
                            x_min,
                            y_min,
                            (x_max - x_min + 1) as i32,
                            (y_max - y_min + 1) as i32,
                        );
                        let cropped_image = Mat::roi(&gray_img, rect)?;

                        let mut non_zero_points = Mat::default();
                        find_non_zero(&mask, &mut non_zero_points)?;

                        let (x_min, y_min, x_max, y_max) = if non_zero_points.rows() > 0 {
                            //let mut min_point = Point::new(0, 0);
                            let mut min_point:f64 = 0.0;
                            //let mut max_point = Point::new(0, 0);
                            let mut max_point:f64 = 0.0;
                            min_max_loc(
                                &non_zero_points,
                                Some(&mut min_point),
                                Some(&mut max_point),
                                None,
                                None,
                                /* mask */
                            )?;
                            (min_point.x, min_point.y, max_point.x, max_point.y)
                        } else {
                            (0, 0, 0, 0)
                        };

                        //min_max_loc(&non_zero_points, Some(&mut min_point), Some(&mut max_point), None, None, /* mask */)?;


                        let rect = Rect::new(
                            x_min,
                            y_min,
                            (x_max - x_min + 1) as i32,
                            (y_max - y_min + 1) as i32,
                        );
                        let cropped_image = Mat::roi(&gray, rect)?;
            */
            //reader = easyocr.Reader(['en'])
            //result = reader.readtext(cropped_image)
            //result

            // Initialize Tesseract
            let mut buffer : Vector<u8> = Vector::new();
            imencode(
                ".png",
                &cropped_image,
                &mut buffer,
                &opencv::core::Vector::new(),
            )?;

            let result = {
                let tess = Tesseract::new(None, Some("eng"))?;
                let buf = buffer.as_slice();
                tess.set_image_from_mem(&buf)?.get_text()?
                //tess.get_text()?
            };

            println!("OCR Result: {}", result);

            /*
            plt.imshow(cv2.cvtColor(new_image, cv2.COLOR_BGR2RGB))

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

            //text = result[0][-2]
            //font = cv2.FONT_HERSHEY_SIMPLEX
            //res = cv2.putText(img, text=text, org=(approx[0][0][0], approx[1][0][1]+60), fontFace=font, fontScale=1, color=(0,255,0), thickness=2, lineType=cv2.LINE_AA)
            //res = cv2.rectangle(img, tuple(approx[0][0]), tuple(approx[2][0]), (0,255,0), 3)
/*
            //let text = &result[0]; // Get the first string in the result
            let text = &result; // Get the first string in the result

            // Define the font and other parameters
            let font = FONT_HERSHEY_SIMPLEX;
            let font_scale = 1.0;
            let color = Scalar::new(0.0, 255.0, 0.0, 0.0); // Green color
            let thickness = 2;
            let line_type = LINE_AA;
        
            // Draw text on the image
            let text_position = Point::new(location.at_2d(0,0).x, locationat_2d(1,0).y + 60);
            put_text(
                &mut img,
                text,
                text_position,
                font,
                font_scale,
                color,
                thickness,
                line_type,
                false,
            )?;
        
            // Draw a rectangle on the image
            let top_left = location[0][0];
            let bottom_right = location[2][0];
            let rec: Rect = Rect::new(top_left.x, top_left.y, bottom_right.x - top_left.x, bottom_right.y - top_left.y);
            rectangle(
                &mut img,
                rec,
                color,
                3, // Thickness
                line_type,
                0, // Shift
            )?;

            let rname = format!("../output/{}_res_image2.jpg", f);
            imwrite(&rname, &cropped_image, &core::Vector::new())?;
*/
        }
    }
    Ok(())
}
