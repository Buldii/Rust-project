// mod app;
// mod tui;
// mod exif;

use std::path::Path;
use little_exif::exif_tag::ExifTag;
use little_exif::metadata::Metadata;
use little_exif::u8conversion::U8conversion;
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}




fn main() {

    let mut exif_array: Vec<(String,String)> = Vec::new();
    // let image_path = Path::new("C:/Users/Kuba/Downloads/LA_tester.png");
    //
    // // unwrap() spowoduje panic, jeżeli pliku nie będzie lub metadane będą uszkodzone
    // let mut metadata = Metadata::new_from_path(&image_path)
    //     .expect("Nie udało się wczytać metadanych");
    //
    // metadata.write_to_file(&image_path)
    //     .expect("Nie udało się zapisać metadanych");

    // println!("Ustawiono opis obrazu");

    // let mut v = Vec::new();
    let png_path = Path::new("C:/Users/Kuba/Downloads/Canon_PowerShot_S40.jpg/Canon_PowerShot_S40.jpg");
    let mut metadata = Metadata::new_from_path(png_path).unwrap();

    // metadata.set_tag(
    //     ExifTag::ImageDescription(("Twoj komentarz do zdjecia".to_string()))
    // );

    // metadata.write_to_file(png_path);

    //                                                                     String::new()
    // let image_description_by_tag = metadata.get_tag(&ExifTag::ImageDescription(String::new())).next().unwrap();

    // let endian = metadata.get_endian();
    // println!("Image description: {:?}", image_description_by_tag);
    // let image_description_string = String::from_u8_vec(
    //     &image_description_by_tag.value_as_u8_vec(&metadata.get_endian()),
    //     &endian
    // );
    // println!("{}", image_description_string);
    for tag in &metadata
    {
        // println!("{:?}", tag);
        // println!("{}", tag.to_string());

        // fn main() {
        //     let x = 21;
        //     let y = 2.5;
        //     println!("{}", type_of(&y));
        //     println!("{}", type_of(x));
        // }
        match tag {
            //
            ExifTag::ModifyDate(date) => {
                //println!("Modify date {:?}", date);
                exif_array.push(("Modify date".parse().unwrap(), date.to_string()));
                // println!("Modify date: {:?}", type_of(date));
            }
        
            ExifTag::ImageDescription(description) => {
                // println!("Image description: {:?}", description);
                exif_array.push(("Image description".to_string() , description.to_string()))
            }
        
            ExifTag::ExifVersion(version) => {
                exif_array.push((
                    "Exif version".to_string(),
                    String::from_utf8(version.to_vec()).unwrap(),
                    
                ));
                println!("{}", type_of(version));
            }
        
            ExifTag::UserComment(comment) => {
                exif_array.push((
                    "User comment".to_string(),
                    String::from_utf8(comment.to_vec()).unwrap(),
                    ))
            }
        
            ExifTag::Artist(_) => {}
            ExifTag::WhitePoint(_) => {}
            ExifTag::PrimaryChromaticities(_) => {}
            ExifTag::ColorMap(_) => {}
            ExifTag::YCbCrCoefficients(_) => {}
            ExifTag::YCbCrSubSampling(_) => {}
            ExifTag::YCbCrPositioning(_) => {}
            ExifTag::ReferenceBlackWhite(_) => {}
            ExifTag::Copyright(_) => {}
            _ => {}
        }
    }

    println!("{:?}", exif_array)
}
