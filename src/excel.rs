extern crate calamine;
extern crate postgres;
extern crate walkdir;

use self::calamine::{open_workbook, DataType, Reader, Xlsx};
use self::postgres::{Connection, TlsMode};
use self::walkdir::WalkDir;
use std::path::Path;

pub fn readfile() {
    //
    // CREATE THE DATABASE, DONE ONLY ONCE
    //
    print!("Create Table in Database");
    const SQLFILE: &str = "sql/create.sql";
    let dsn = "postgresql://antonecto:onlylocal@localhost/antondb";//went to github :-)
    let conn: Connection = Connection::connect(dsn, TlsMode::None).unwrap();
    let create = std::fs::read_to_string(SQLFILE).expect("Error reading the sql");
    conn.batch_execute(&create).expect("sql creation failed");
    println!("...done");

    for entry in WalkDir::new("/home/anton/Temp/testdaten")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        if f_name.ends_with("HA.xlsx") {
            println!("checke {}", entry.path().to_str().unwrap());
            upload_file(&entry.path(), &conn);
        }
    }
}

pub fn upload_file(path: &Path, conn: &Connection) {
    //
    // iterate over all sheets and push them to db
    //
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");
    let sheets = workbook.sheet_names().to_owned();
    for s in sheets {
        if let Some(Ok(range)) = workbook.worksheet_range(&s) {
            //
            // now we have a sheet
            //
            let sheetname = String::from(s);
            let (rows, cols) = range.get_size();
            let non_empty_cells: usize = range.used_cells().count();
            assert_eq!(
                non_empty_cells,
                range
                    .rows()
                    .flat_map(|r| r.iter().filter(|&c| c != &DataType::Empty))
                    .count()
            );
            if non_empty_cells == 0 {
                return;
            };

            let mut insert = String::with_capacity(100000);
            insert.push_str("insert into allexcel (filename, sheetname, rownumber ");
            for c in 0..cols {
                insert.push_str(&format!(",s{:03} ", c));
            }
            insert.push_str(") values ");
            for (row_idx, row) in range.rows().enumerate() {
                insert.push_str(if row_idx == 0 { "" } else { "," });
                insert.push_str(&format!(
                    "('{}','{}','{}'",
                    path.to_str().unwrap(),
                    sheetname,
                    row_idx
                ));
                for col in row.iter() {
                    use excel::postgres::types::ToSql;
                    use excel::postgres::types::TEXT;
                    let mut colstr: Vec<u8> = Vec::new();
                    format!("{}", col)
                        .to_string()
                        .to_sql(&TEXT, &mut colstr)
                        .unwrap();
                    let colstr = String::from_utf8(colstr).unwrap_or("".to_string());
                    let cell = format!(",'{}' ", colstr).to_string();
                    insert.push_str(&cell.trim());
                }
                insert.push_str(")\n");
            }
            println!("Prepare: \n\n {} \n\n", &insert);
            let stmt = match conn.prepare(&insert) {
                Ok(stmt) => stmt,
                Err(e) => {
                    println!("Preparing query failed: {:?}", e);
                    return;
                }
            };
            stmt.execute(&[]).expect("Inserting sheet failed!");
            println!(
                "...Blatt {} mit {} Zeilen und {} Spalten verarbeitet ",
                sheetname, rows, cols
            );
        }
    }
}
