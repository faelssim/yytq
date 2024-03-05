use chrono::prelude::*;
use colored::*;
use std::time::{Duration, UNIX_EPOCH};

use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Temperature {
  temp: f64,
  temp_min: f64,
  temp_max: f64,
  feels_like: f64,
  humidity: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Coord {
  lon: f64,
  lat: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sys {
  sunrise: i32,
  sunset: i32,
}


/// 定义接口返回的数据结构
#[derive(Deserialize, Serialize, Debug)]
pub struct Weather {
  main: Temperature,
  sys: Sys,
  coord: Coord,
}

impl Weather {
  pub async fn get(city: &String) -> Result<Self, ExitFailure> {
    println!("{}", "---------正在获取天气，请稍后---------".bright_green());
    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&APPID=a94ea17df56a4deb95859a82f07fcac0", city);
    let url = Url::parse(url.as_str())?;
    let response: Weather = reqwest::get(url).await?.json::<Weather>().await?;
    Ok(response)
  }
}

pub fn formate_timestamp(timestamp: i32, format: &str) -> String {
  let time = UNIX_EPOCH + Duration::from_secs(timestamp as u64);
  let datetime = DateTime::<Local>::from(time);
  datetime.format(format).to_string()
}

pub fn print_response(resp: &Weather) {
  println!(
    "  当前温度：{}℃ \n  当前最低温：{}℃ \n  当前最高温：{}℃ \n  体感温度：{}℃ \n  湿度：{}% \n  今日日出时间：{} \n  今日日落时间：{} \n  所在经度：{} \n  所在纬度：{}",
    resp.main.temp.to_string().bright_red(),
    resp.main.temp_min,
    resp.main.temp_max,
    resp.main.feels_like,
    resp.main.humidity,
    formate_timestamp(resp.sys.sunrise, "%H:%M:%S"),
    formate_timestamp(resp.sys.sunset, "%H:%M:%S"),
    resp.coord.lon,
    resp.coord.lat
  );
}


/// 时间戳的单测
#[test]
fn test_timestamp_to_time() {
  assert_eq!(
    formate_timestamp(1643467428, "%H:%M:%S"),
    "22:43:48".to_string()
  );
  assert_eq!(
    formate_timestamp(1643467428, "%Y-%m-%d %H:%M:%S"),
    "2022-01-29 22:43:48".to_string()
  );
}