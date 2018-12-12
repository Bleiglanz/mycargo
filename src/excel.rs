extern crate postgres;
extern crate calamine;

use self::calamine::{open_workbook, Xlsx, Reader, DataType, Range};
use self::postgres::{Connection,TlsMode};
//use self::postgres::types::FromSql;
//use self::postgres::Result as PgResult;

pub fn get_range(path:&str, sheet:&str) -> Option<Range<DataType>> {
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");
    if let Some(Ok(range)) = workbook.worksheet_range(sheet) {
        Some(range)
    }else{
        None
    }
}

pub fn readfile(){

    const PATH:&str = "/home/anton/Temp/testdaten/tameld.xlsx";
    const SHEETNAME:&str = "RawData";
    const SQLFILE:&str = "sql/create.sql";
    let pval = format!("{}",PATH);
    let sval = format!("{}",SHEETNAME);

    println!("Create Table in Database");
    let dsn = "postgresql://antonecto:onlylocal@localhost/antondb";
    let conn = match Connection::connect(dsn, TlsMode::None) {
        Ok(conn) => conn,
        Err(e) => { println!("Connection error: {:?}", e); return; }
    };

    let create = std::fs::read_to_string(SQLFILE).expect("Err reading the sql");
    conn.batch_execute(&create).expect("sql creation failed");

    println!("Excel Reader");

    // Read whole worksheet data and provide some statistics
    if let Some(range) = get_range(&PATH,&SHEETNAME){
        let (rows,cols) = range.get_size();
        let total_cells = rows * cols;
        let non_empty_cells: usize = range.used_cells().count();
        println!("Zellen: {}",total_cells);
        assert_eq!(non_empty_cells, range.rows()
                   .flat_map(|r| r.iter().filter(|&c| c != &DataType::Empty)).count());

        let mut insert = "insert into allexcel (filename, sheetname, rownumber ".to_string();
        for c in 0..cols {
            insert.push_str(&format!(",s{:03} ",c));
        }
        insert.push_str(") values (unnest($1::text[]), unnest($2::text[]), unnest($3::bigint[])");
        for c in 0..cols {
            insert.push_str(&format!(", unnest(${}::text[])",c+4));
        }
        insert.push_str(")");
        println!("{}",insert);
        let stmt = match conn.prepare(&insert) {
            Ok(stmt) => stmt,
            Err(e) => { println!("Preparing query failed: {:?}", e); return; }
        };
        println!("Statement bereit, lies Daten"); //Todo: unten

        let val_filename  = vec![&PATH;rows];
        let val_sheetname = vec![&SHEETNAME;rows];
        let mut val_rownumber = vec![0;rows];
        let mut val_sxxx = vec![vec!["";rows];cols];
        for (row_idx,row) in range.rows().enumerate() {
            val_rownumber[row_idx]=row_idx;
            for (col_idx,col) in row.iter().enumerate() {
                
                val_sxxx[col_idx][row_idx]=&(format!("{}",col).trim().to_string());

                //                println!("Zeile {}",row_idx);
//                stmt.execute(&[&pval, &sval, &(row_idx as i64), &(_col_idx as i64), &value]).expect("db insert failed!");
            }
        }
    }
}
