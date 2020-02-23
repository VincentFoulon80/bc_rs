mod bop_core;
mod structs;
use std::thread;
use std::sync::mpsc::channel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // like a test
    println!("получение тегов...");
    let tags = bop_core::get_tags::get_tags().await;
    println!("{:#?}", tags);
    println!("получение данных с тега 5...");
    let data: structs::struct_json_discover::Root = bop_core::get_album_data::get_tag_data(tags[5].clone(), 1).await;
    println!("{:#?}", data);
    println!("получение первого альбома: {}", data.items[0].tralbum_url.as_str());
    let album_page: Result<String, reqwest::Error> = bop_core::bop_http_tools::http_request(data.items[0].tralbum_url.as_str()).await;
    match album_page {
        Ok(value) => {
            let album_json = bop_core::get_album_data::get_album_data(value.as_str());
            match album_json {
                Some(album_value) => {
                    let album_json_fixed = bop_core::get_album_data::fix_json(album_value);
                    println!("{}", album_json_fixed);
                    let data: structs::struct_json_album::Root = serde_json::from_str(album_json_fixed.as_str()).unwrap();
                    println!("{:#?}", data);
                    println!("попытка проиграть первый трек...");

                    let device = rodio::default_output_device().unwrap();
                    let track = bop_core::playback::get_track_from_url(data.trackinfo[0].file.mp3128.as_str()).await;
                    let sink = bop_core::playback::create_sink(track, device, 0);
                    sink.play();
                    loop {
                        thread::sleep_ms(1000);
                        println!("{}", sink.volume());
                        sink.set_volume(sink.volume() - 0.1);
                    }
                }
                None => println!("не получилось"),
            }
        }
        Err(_) => {
            panic!("пиздец");
        }
    }
    Ok(())
}