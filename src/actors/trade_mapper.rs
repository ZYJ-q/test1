pub struct TradeMapper;
// use super::http_data::TradeRe;
use crate::actors::database::get_connect;
// use log::info;
use mysql::*;
use mysql::prelude::*;
use serde_json::Value;
use super::db_data::{Trader, Equity};


impl TradeMapper {
  // 插入数据
  pub fn insert_net_worth(net_worths:Vec<Value>) -> bool {
    // 连接数据库
    let mut conn = get_connect();
    // let query_id = conn.exec_first(, params)

    let flag = conn.exec_batch(
      r"INSERT IGNORE INTO net_worth (name, time, net_worth, prod_id)
      VALUES (:name, :time, :net_worth, :prod_id)",
      net_worths.iter().map(|p| params! {
        "name" => &p["name"],
        "time" => &p["time"],
        "net_worth" => &p["net_worth"],
        "prod_id" => &p["prod_id"]
      })
    );

    match flag {
      Ok(_c) => {
        println!("insert success!");
        return true;
      },
      Err(e) => {
        eprintln!("error:{}", e);
        return false;
      }
    }
  }


  pub fn get_traders() -> Result<Vec<Trader>> {
    // 连接数据库
    let mut conn = get_connect();
    let res = conn.query_map(
      r"select * from test_traders",
      |(tra_id, tra_venue, ori_balance, tra_currency, api_key, secret_key, other_keys, r#type, name, alarm, threshold)| {
        Trader{ tra_id, tra_venue, ori_balance, tra_currency, api_key, secret_key, other_keys, r#type, name, alarm, threshold }
      } 
    ).unwrap();
    return Ok(res);
  }

  pub fn get_equity() -> Result<Vec<Equity>> {
    // 连接数据库
    let mut conn = get_connect();
    let res = conn.query_map(
      r"select * from total_bian_equity where id = 54422",
      |(id, name, time, equity, r#type)| {
        Equity{ id, name, time, equity, r#type}
      } 
    ).unwrap();
    return Ok(res);
  }

  pub fn insert_bian_equity(equitys:Vec<Value>) -> bool {
    // 连接数据库
    let mut conn = get_connect();
    // let query_id = conn.exec_first(, params)

    let flag = conn.exec_batch(
      r"INSERT IGNORE INTO bian_15m_equity (name, equity, time, type)
      VALUES (:name, :equity, :time, :type)",
      equitys.iter().map(|p| params! {
        "name" => &p["name"],
        "equity" => &p["equity"],
        "time" => &p["time"],
        "type" => &p["type"]
      })
    );

    match flag {
      Ok(_c) => {
        println!("insert success!");
        return true;
      },
      Err(e) => {
        eprintln!("error:{}", e);
        return false;
      }
    }
  }


  pub fn insert_bybit_equity(equitys:Vec<Value>) -> bool {
    // 连接数据库
    let mut conn = get_connect();
    // let query_id = conn.exec_first(, params)

    let flag = conn.exec_batch(
      r"INSERT IGNORE INTO bybit_equitys (name, time, equity)
      VALUES (:name, :time, :equity)",
      equitys.iter().map(|p| params! {
        "name" => &p["name"],
        "time" => &p["time"],
        "equity" => &p["equity"]
      })
    );

    match flag {
      Ok(_c) => {
        println!("insert success!");
        return true;
      },
      Err(e) => {
        eprintln!("error:{}", e);
        return false;
      }
    }
  }
}










