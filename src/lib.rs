use std::fs;

#[derive(Debug)]
pub struct CsvParser {
    header: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl CsvParser {
    // This currently assumes there are no commas in cells (for now).
    // It will also strip " from the start and end of cells (for now).
    pub fn new(csv_path: &String) -> Self {
        let file_contents = fs::read_to_string(&csv_path).expect("Failed to read CSV.");
        let mut lines = file_contents.lines();
        let mut header: Vec<String> = Vec::new();
        let mut rows: Vec<Vec<String>> = Vec::new();

        // Start with the header of the CSV
        if let Some(first_line) = lines.next() {
            let header_split = first_line.split(",");

            for cell in header_split {
                header.push(String::from(cell.trim()));
            }
        } else {
            panic!("CSV has no data.");
        }

        for row in lines {
            let row_split = row.split(",");
            let mut row_vec: Vec<String> = Vec::new();

            for cell in row_split {
                row_vec.push(String::from(cell.trim()));
            }

            rows.push(row_vec);
        }

        Self {
            header: header.clone(),
            rows: rows.clone()
        }
    }
}

pub enum CellType {
    Text,
    Number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let csv_parser = CsvParser::new(&String::from("test.csv"));
        let expected = vec![
            String::from("Column 1"),
            String::from("Column 2"),
            String::from("Column 3"),
            String::from("Column 4"),
            String::from("Column 5")
        ];

        println!("{:?}", csv_parser);

        assert_eq!(csv_parser.header, expected);
        assert_eq!(csv_parser.rows.len(), 5);
    }
}
