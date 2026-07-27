mod camera;

fn main() -> Result<(), String> {
    println!("");
    println!("== ここからmain処理開始 ==");
    println!("");

    let c = camera::Camera::new(1920, 1080)?;

    println!("Camera initialized.");

    camera::Camera::print_info(&c);
    c.capture("test_file.jpg")?;

    println!("");
    println!("== ここでmain処理終了 ==");
    println!("");
    Ok(())
}
