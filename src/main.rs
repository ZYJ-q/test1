use std::collections::VecDeque;

use csv::Reader;
use net_worth::actors::trade_mapper;
use serde_json::{Map, Value};
use net_worth::actors::*;
use chrono::{DateTime, NaiveDateTime, Utc, Local};
const CSV_PATH:&str = "./xh02_equity.csv";

fn main(){
    init();
    // let mut data = Reader::from_path(CSV_PATH).unwrap();
    let mut equity_histories: VecDeque<Value> = VecDeque::new();
    let data = trade_mapper::TradeMapper::get_equity().unwrap();

    // println!("获取到的权益数据{:?}", data );

    for i in data{
        let mut equity_bian_map: Map<String, Value> = Map::new();
        let name = i.name;
        
        if name == 12{
            let new_equity: f64 = i.equity.as_str().parse().unwrap();
            let equity = new_equity - 4535.7;
            println!("权益{}", equity);
            let time = i.time.as_str();
            let t = NaiveDateTime::parse_from_str(&time, "%Y/%m/%d %H:%M:%S").unwrap();
            let r#type = i.r#type.as_str();
            let new_time = format!("{}", t);

            println!("时间{}", t);

            
            equity_bian_map.insert(String::from("name"), Value::from(name));
            equity_bian_map.insert(String::from("equity"), Value::from(equity));
            equity_bian_map.insert(String::from("time"), Value::from(new_time));
            equity_bian_map.insert(String::from("type"), Value::from(r#type));
            

        } else {
            let equity = i.equity;
            println!("权益{}", equity);
        let time = i.time.as_str();
        let t = NaiveDateTime::parse_from_str(&time, "%Y/%m/%d %H:%M:%S").unwrap();
            let r#type = i.r#type.as_str();
            let new_time = format!("{}", t);
        equity_bian_map.insert(String::from("name"), Value::from(name));
            equity_bian_map.insert(String::from("equity"), Value::from(equity));
            equity_bian_map.insert(String::from("time"), Value::from(new_time));
            equity_bian_map.insert(String::from("type"), Value::from(r#type));
            
        }
        

    }
    

    // data.records().into_iter().for_each(|item|{
    //     if let Ok(sr) = item{
    //         let mut equity_bian_map: Map<String, Value> = Map::new();
    //         // println!("{}", sr.get(3).unwrap()); 
    //         equity_bian_map.insert(String::from("time"), Value::from(sr.get(0).unwrap())); 
    //         equity_bian_map.insert(String::from("name"), Value::from(8));
    //         equity_bian_map.insert(String::from("equity"), Value::from(sr.get(2).unwrap())); 
    //         equity_bian_map.insert(String::from("type"), Value::from("Futures"));
    //         equity_histories.push_back(Value::from(equity_bian_map));   
    //     }
    // });


    // println!("数据{:?}",Vec::from(equity_histories.clone()));
    // let res = trade_mapper::TradeMapper::insert_bian_equity(Vec::from(equity_histories.clone()));

    // println!("插入数据是否成功{}, {:?}",res,Vec::from(equity_histories.clone()));

    


}

