use std::collections::VecDeque;

use csv::Reader;
use net_worth::actors::trade_mapper;
use serde_json::{Map, Value};
use net_worth::actors::*;
const CSV_PATH:&str = "./angus_equity.csv";

fn main(){
    init();
    let mut data = Reader::from_path(CSV_PATH).unwrap();
    let mut equity_histories: VecDeque<Value> = VecDeque::new();
    

    data.records().into_iter().for_each(|item|{
        if let Ok(sr) = item{
            let mut equity_bian_map: Map<String, Value> = Map::new();
            // println!("{}", sr.get(3).unwrap()); 
            equity_bian_map.insert(String::from("time"), Value::from(sr.get(0).unwrap())); 
            equity_bian_map.insert(String::from("name"), Value::from(3));
            equity_bian_map.insert(String::from("equity"), Value::from(sr.get(2).unwrap())); 
            equity_bian_map.insert(String::from("type"), Value::from("Futures"));
            equity_histories.push_back(Value::from(equity_bian_map));   
        }
    });


    // println!("数据{:?}",Vec::from(equity_histories.clone()));
    let res = trade_mapper::TradeMapper::insert_bian_equity(Vec::from(equity_histories.clone()));

    println!("插入数据是否成功{}, {:?}",res,Vec::from(equity_histories.clone()));

    


}

