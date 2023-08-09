use std::collections::VecDeque;

use csv::Reader;
use net_worth::actors::trade_mapper;
use serde_json::{Map, Value};
use net_worth::actors::*;
use chrono::{DateTime, NaiveDateTime, Utc, Local};
const CSV_PATH:&str = "./zd01_equity.csv";

fn main(){
    init();
    let mut data = Reader::from_path(CSV_PATH).unwrap();
    let mut equity_histories: VecDeque<Value> = VecDeque::new();
    // let data = trade_mapper::TradeMapper::get_equity().unwrap();

    // println!("获取到的权益数据{:?}", data );
    // let len = data.len();

    // for i in 0..len{
    //     let mut equity_bian_map: Map<String, Value> = Map::new();
    //     let name = data[i * 3].name;
        
    //     if name == 12{
    //         let equity = &data[i * 3].equity;
    //         let new_equity = &equity[1..equity.len()-1];
    //         // println!("处理之后权益{}", new_equity);
    //         let equitys:f64 = new_equity.parse().unwrap();
    //         let new_equitys = equitys - 4535.7;
    //         // println!("权益1111{}", new_equitys);
    //     let time = &data[i * 3].time;
    //     let new_time = &time[1..time.len()-1];

    //     // let new_times = format!("{}/{}/{} {}:{}:{}", ts[0], new_t_m, new_t_d, new_t_h, new_t_ms, new_t_s);

    //     // let tims = new_time.timestamp_millis();
    //     // let t = NaiveDateTime::parse_from_str(&new_time, "%-Y/%-m/%-d %-H:%-M:%-S").unwrap();
    //     // println!("时间1111111111{}", t);
    //         let r#type = &data[i].r#type;
    //         let new_type = &r#type[1..r#type.len()-1];
    //         // let new_time = format!("{}", t);
    //         // println!("时间{}", new_time);
            
    //         println!("权益{}", new_equitys);

            
    //         equity_bian_map.insert(String::from("name"), Value::from(name));
    //         equity_bian_map.insert(String::from("equity"), Value::from(new_equitys));
    //         equity_bian_map.insert(String::from("time"), Value::from(new_time));
    //         equity_bian_map.insert(String::from("type"), Value::from(new_type));
    //         equity_histories.push_back(Value::from(equity_bian_map));     
            
            

    //     } else {
    //         let equity = &data[i * 3].equity;
    //         // println!("权益{}", equity);
    //         let new_equity = &equity[1..equity.len()-1];
    //         // println!("处理之后权益{}", new_equity);
    //         let equitys:f64 = new_equity.parse().unwrap();
    //     let time = &data[i * 3].time;
    //     let new_time = &time[1..time.len()-1];
    //     println!("打印时间{}", new_time);
    //     // let t = NaiveDateTime::parse_from_str(&time[1..time.len()-1], "%Y/%m/%d %H:%M").unwrap();
    //         let r#type = &data[i * 3].r#type;
    //         let new_type = &r#type[1..r#type.len()-1];
    //         // let new_time = format!("{}", t);
    //         // println!("时间{}", new_time);
    //         // let t = NaiveDateTime::parse_from_str(&new_time, "%Y/%m/%d %H:%M").unwrap();
    //         // let date_time = format!("{}", t.format("%Y/%m/%d %H:%M:%S"));
    //     // println!("时间1111111111{}", t);
    //     equity_bian_map.insert(String::from("name"), Value::from(name));
    //         equity_bian_map.insert(String::from("equity"), Value::from(equitys));
    //         equity_bian_map.insert(String::from("time"), Value::from(new_time));
    //         equity_bian_map.insert(String::from("type"), Value::from(new_type));
    //         equity_histories.push_back(Value::from(equity_bian_map));
            
    //     }
        

    // }
    

    data.records().into_iter().for_each(|item|{
        if let Ok(sr) = item{
            let mut equity_bian_map: Map<String, Value> = Map::new();
            // println!("{}", sr.get(3).unwrap()); 
            let time = sr.get(1).unwrap();
            let t = NaiveDateTime::parse_from_str(time, "%Y/%m/%d %H:%M").unwrap();
            let date_time = format!("{}", t.format("%Y/%m/%d %H:%M:%S"));
            let equity = sr.get(3).unwrap();
            let new_equitys:f64 = equity.parse().unwrap();
            equity_bian_map.insert(String::from("time"), Value::from(date_time)); 
            equity_bian_map.insert(String::from("name"), Value::from(14));
            equity_bian_map.insert(String::from("equity"), Value::from(new_equitys)); 
            equity_bian_map.insert(String::from("type"), Value::from("Futures"));
            equity_histories.push_back(Value::from(equity_bian_map));   
        }
    });


    println!("数据{:?}",Vec::from(equity_histories.clone()));
    let res = trade_mapper::TradeMapper::insert_bian_equity(Vec::from(equity_histories.clone()));

    println!("插入数据是否成功{}",res);

    


}

