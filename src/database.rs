// //extern crate time;
// //extern crate chrono;
// #[allow(dead_code)]
// extern crate postgres_array;
// extern crate postgres;

// use self::postgres_array::Array;
// use self::postgres::{Connection, TlsMode};
// use self::postgres::types::FromSql;
// use self::postgres::Result as PgResult;
// //use self::chrono::prelude::*;
// //use self::time::Timespec;

// fn get_single_value<T>(conn: &Connection, query: &str) -> PgResult<T> where T: FromSql
// {
//     println!("Executing query: {}", query);
//     let stmt = conn.prepare(query)?;
//     let rows = stmt.query(&[])?;
//     let row = rows.iter().next().unwrap();
//     row.get_opt(0).unwrap()
// }

// fn connect() -> Connection {
//     let dsn = "postgresql://antonecto:onlylocal@localhost/antondb";
//     match Connection::connect(dsn, TlsMode::None){
//         Ok(conn) => conn,
//         Err(e) => {
//             println!("Connection error: {:?}", e);
//             panic!("no connection");
//         }
//     }
// }
// #[allow(unused_variables)]
// pub fn execute() {

//     let conn = connect();

//     let contents = std::fs::read_to_string("sql/create.sql").expect("Something went wrong reading the sql");
//     conn.batch_execute(&contents).expect("sql creation failed");

//     conn.execute("create table if not exists blog (
//         id serial primary key,
//         title \
//                   varchar(255),
//         body text)",
//                  &[])
//         .expect("Table creation failed");
//     let stmt = match conn.prepare("insert into blog (title, body) values ($1, $2)") {
//         Ok(stmt) => stmt,
//         Err(e) => {
//             println!("Preparing query failed: {:?}", e);
//             return;
//         }
//     };
//     for i in 1..5 {
//         let title = format!("Blogpost number {}", i);
//         let text = format!("Content of the blogpost #{}", i);
//         stmt.execute(&[&title, &text]).expect("Inserting blogposts failed");
//     }
//     let stmt = match conn.prepare("select id, title, body from blog where id < $1") {
//         Ok(stmt) => stmt,
//         Err(e) => {
//             println!("Preparing query failed: {:?}", e);
//             return;
//         }
//     };
//     let max_id: i32 = 3;
//     let rows = stmt.query(&[&max_id]).expect("Selecting blogposts failed");
//     for row in rows.iter() {
//         let id: i32 = row.get("id");
//         let title: String = row.get("title");
//         println!("ID={}, title={}", id, title);
//     }
//     println!("{:?}", get_single_value::<bool>(&conn, "select 1=1"));
//     println!("{:?}", get_single_value::<i32>(&conn, "select 1=1"));

//     type IntArray = Array<Option<i32>>;
//     let arr = get_single_value::<IntArray>(&conn, "select '{4, 5, 6}'::int[]");
//     // println!("{:?}",
//     //          arr.map(|arr| {
//     //              arr.iter()
//     //                  .filter_map(|x| *x)
//     //                  .collect::<Vec<_>>()
//     //          }));

//     //let json = get_single_value::<Json>(&conn, "select '{\"foo\": \"bar\", \"answer\": 42}'::json");
//     //println!("{:?}", json);

//     // let range = get_single_value::<Range<i32>>(&conn, "select '[10, 20)'::int4range");
//     // println!("{:?}", range);
//     // let ts_range =
//     // get_single_value::<Range<Timespec>>(&conn, "select '[2015-01-01, 2015-12-31]'::tsrange");
//     // println!("{:?}", ts_range);

//     // sql_macro();
// }
