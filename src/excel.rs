extern crate calamine;

use self::calamine::{open_workbook, Xlsx, Reader, DataType, Range};


pub fn get_range(path:&str, sheet:&str) -> Option<Range<DataType>> {
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");
    if let Some(Ok(range)) = workbook.worksheet_range(sheet) {
        Some(range)
    }else{
        None
    }
}

pub fn readfile(){

    println!("Excel Reader");
    let path = "/home/anton/Temp/testdaten/tameld.xlsx";
    let sheetname = "RawData";

    // Read whole worksheet data and provide some statistics
    if let Some(range) = get_range(path,sheetname){
        let (rows,cols) = range.get_size();
        let total_cells = rows * cols;
        let non_empty_cells: usize = range.used_cells().count();
        println!("Found {} cells in {}, including {} non empty cells",
             sheetname, total_cells, non_empty_cells);
        assert_eq!(non_empty_cells, range.rows()
                   .flat_map(|r| r.iter().filter(|&c| c != &DataType::Empty)).count());

        let mut zeilenindex:usize = 0;
        let mut spaltenindex_max:usize = 0;
        for row in range.rows() {
            let mut spaltenindex:usize = 0;
            for col in row.iter() {
                let _value = format!("{}",col).trim().to_string();
                //println!("{:03} {:02} {}",zeilenindex, spaltenindex, value);
                spaltenindex += 1;
                spaltenindex_max = if spaltenindex>spaltenindex_max {spaltenindex} else {spaltenindex_max};
            }
            zeilenindex += 1;
            //println!("row={:?}, row[0]={:?}", row, row[0]);
        }
        println!("Maximale Spaltenanzahl {} in {} Zeilen",spaltenindex_max,zeilenindex-1);
    }
}
