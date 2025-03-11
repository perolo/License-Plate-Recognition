use opencv::core::{
    self, bitwise_and, find_non_zero, AlgorithmHint, Mat, Rect, Scalar, Vector, CV_8UC1,
};
use opencv::imgcodecs::{have_image_reader, imencode, imread, imwrite, IMREAD_COLOR};
use opencv::imgproc::{
    approx_poly_dp, bilateral_filter, bounding_rect, canny, contour_area, cvt_color, draw_contours,
    find_contours, CHAIN_APPROX_SIMPLE, COLOR_BGR2GRAY, RETR_TREE,
};
use opencv::prelude::*;

use tesseract::Tesseract;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //files = ["image1", "image2", "image3", "image4"]
    let files = ["image1", "image2", "image3", "image4"];
    //for f in files:
    for f in files {
        let res = ocr_licenseplate(f, false);
        if let Ok(result) = res {
            println!("OCR Result: {}", result);
        } else {
            println!("Error: {:?}", res);
            //Err(res)
        }
    }
    Ok(())
}

fn ocr_licenseplate(car_image: &str, write_intermediate: bool) -> Result<String, Box<dyn std::error::Error>> {
    //img = cv2.imread(f + ".jpg")
    //let fname = "../data/".to_owned() + f + ".jpg";
    let fname = format!("../data/{}.jpg", car_image);
    have_image_reader(&fname)?;
    if let Ok(img) = imread(&fname, IMREAD_COLOR) {
        let mut gray_img = Mat::default();
        let hint: AlgorithmHint = AlgorithmHint::ALGO_HINT_DEFAULT;
        cvt_color(&img, &mut gray_img, COLOR_BGR2GRAY, 0, hint)?;
        if write_intermediate {
            let gname = format!("../output/{}_gray.jpg", car_image);
            imwrite(&gname, &gray_img, &core::Vector::new())?;
        }
        let mut bfilter = Mat::default();
        bilateral_filter(
            &gray_img,
            &mut bfilter,
            11,
            17.0,
            17.0,
            core::BORDER_DEFAULT,
        )?;
        let mut edged = Mat::default();
        canny(&bfilter, &mut edged, 30.0, 200.0, 3, false)?;
        if write_intermediate { 
            let ename = format!("../output/{}_edged2.jpg", car_image);
        imwrite(&ename, &edged, &core::Vector::new())?;
        }
        let mut contours: core::Vector<core::Vector<core::Point>> = core::Vector::new();
        find_contours(
            &edged,
            &mut contours,
            //&mut hierarchy,
            RETR_TREE,
            CHAIN_APPROX_SIMPLE,
            core::Point::new(0, 0),
        )?;
        let mut contours_vec: Vec<core::Vector<core::Point>> = contours.to_vec();
        contours_vec.sort_by(|a, b| {
            let area_a = contour_area(a, false).unwrap_or(0.0);
            let area_b = contour_area(b, false).unwrap_or(0.0);
            area_b.partial_cmp(&area_a).unwrap() // Reverse order for descending sort
        });
        let top_contours: Vec<core::Vector<core::Point>> =
            contours_vec.into_iter().take(10).collect();
        let size = gray_img.size()?;
        let mut mask = Mat::new_size_with_default(size, CV_8UC1, Scalar::all(0.0))?;
        let mut location = Mat::default();
        for contour in &top_contours {
            let mut approx = Mat::default();
            approx_poly_dp(&contour, &mut approx, 10.0, true)?;
            if approx.rows() == 4 {
                location = approx;
                //println!("Location found!: {:?}", location);
                break;
            }
        }
        let wrapped_location: core::Vector<Mat> = core::Vector::from(vec![location.clone()]);
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
        if write_intermediate {
            let mname = format!("../output/{}_mask2.jpg", car_image);
        imwrite(&mname, &mask, &core::Vector::new())?;
        }
        bitwise_and(&img, &img, &mut new_image, &mask)?;
        if write_intermediate {

        let nname = format!("../output/{}_new_image2.jpg", car_image);
        imwrite(&nname, &new_image, &core::Vector::new())?;
        }
        let mut non_zero_points = Mat::default();
        find_non_zero(&mask, &mut non_zero_points)?;
        let bounding_box: Rect = bounding_rect(&non_zero_points)?;
        let cropped_image = Mat::roi(&gray_img, bounding_box)?;
        if write_intermediate {
            let cname = format!("../output/{}_cropped_image2.jpg", car_image);
        imwrite(&cname, &cropped_image, &core::Vector::new())?;
        }
        let mut buffer: Vector<u8> = Vector::new();
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
        };
        Ok(result)
    } else {
        Err("Error reading image".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("image1", "IHR 26.BR 9044,\n")]
    #[case("image2", ":COVID{9\n")]
    #[case("image3", "BJY -982|\n")]
    #[case("image4", "HS82 FKL\n")]
    fn test_ocr_licenseplate(#[case] image_name: &str, #[case] expected: &str) {
        let result = ocr_licenseplate(image_name, false);
        assert!(result.is_ok());
        let result_text = result.unwrap();
        assert_eq!(&result_text, expected);
    }

    
    #[test]
    fn test_ocr_licenseplate_invalid_image() {
        let result = ocr_licenseplate("non_existent_image", false);
        assert!(result.is_err());
    }
}
