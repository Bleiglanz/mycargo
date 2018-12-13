extern crate postgres;
extern crate calamine;
extern crate walkdir;

use self::walkdir::WalkDir;
use self::calamine::{open_workbook, Xlsx, Reader, DataType, Range};
use self::postgres::{Connection,TlsMode};
use std::path::Path;

fn get_ranges(path:&Path) -> Vec<(String,Range<DataType>)> {
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");
    let sheets = workbook.sheet_names().to_owned();
    let mut result:Vec<(String,Range<DataType>)> = Vec::new();
    for s in sheets {
        if let Some(Ok(range)) = workbook.worksheet_range(&s) {
             result.push((String::from(s),range));
        }
    }
    result
}

pub fn readfile(){

    //
    // CREATE THE DATABASE, DONE ONLY ONCE
    //

    print!("Create Table in Database");
    const SQLFILE:&str = "sql/create.sql";
    let dsn = "postgresql://antonecto:onlylocal@localhost/antondb";
    let conn:Connection = Connection::connect(dsn, TlsMode::None).unwrap();
    let create = std::fs::read_to_string(SQLFILE).expect("Error reading the sql");
    conn.batch_execute(&create).expect("sql creation failed");
    println!("...done");

    for entry in WalkDir::new("/home/anton/Temp/testdaten")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok()) {
            let f_name = entry.file_name().to_string_lossy();
            if f_name.ends_with("sdaten.xlsx") {
                println!("checke {}",entry.path().to_str().unwrap());
                upload_file(&entry.path(),&conn);
            }
    }
}


pub fn upload_file(path:&Path,conn:&Connection){

    for (sheetname, range) in get_ranges(path){
        let (rows,cols) = range.get_size();
        let total_cells = rows * cols;
        let non_empty_cells: usize = range.used_cells().count();
        assert_eq!(non_empty_cells, range.rows().flat_map(|r| r.iter().filter(|&c| c != &DataType::Empty)).count());
        if non_empty_cells == 0 { return; };
        let mut insert = String::with_capacity(100000);
        insert.push_str("insert into allexcel (filename, sheetname, rownumber ");
        for c in 0..cols {
            insert.push_str(&format!(",s{:03} ",c));
        }
        insert.push_str(") values ");
        for (row_idx,row) in range.rows().enumerate() {
            insert.push_str(if row_idx==0 {""} else {","} );
            insert.push_str(&format!("('{}','{}','{}'",path.to_str().unwrap(),sheetname,row_idx));
            for (col_idx,col) in row.iter().enumerate() {
                let colstr = format!("{}",col).replace(r"\",r"\\").replace(r"'",r"\'");
                let cell = format!(",'{}' ",colstr).to_string();
                insert.push_str(&cell.trim());
            }
            insert.push_str(")\n");
        }
        let stmt = match conn.prepare(&insert) {
            Ok(stmt) => stmt,
            Err(e) => { println!("Preparing query failed: {:?}", e); println!("{}",insert); return; }
        };
        stmt.execute(&[]).expect("Inserting sheet failed");
        println!("Blatt {} eingefügt ",sheetname);
    }
}
