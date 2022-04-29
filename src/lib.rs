use std::fs;

#[derive(Debug)]
pub struct CsvParser {
    header: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl CsvParser {
    // This currently assumes there are no commas in cells (for now).
    // It will also strip " from the start and end of cells (for now).
    pub fn new_old_version(csv_path: &String) -> Self {
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

    pub fn new(csv_path: &String) -> Self {
        let file_contents = fs::read_to_string(&csv_path).expect("Failed to read CSV.");
        let mut lines = file_contents.lines();
        let header: Vec<String>;
        let mut rows: Vec<Vec<String>> = Vec::new();

        // Start with the header of the CSV
        if let Some(first_line) = lines.next() {
            header = parse_line(first_line);
        } else {
            panic!("CSV has no data.");
        }

        for row in lines {
            rows.push(parse_line(row));
        }

        Self {
            header: header.clone(),
            rows: rows.clone()
        }
    }
}

fn parse_line(line: &str) -> Vec<String> {
    let mut row: Vec<String> = Vec::new();
    let mut in_cell = true;
    let mut in_quoted_cell = false;

    // If the very first cell is quote-delimited, switch in_cell to false
    // This will guarantee that the first " will be ignored.
    if let Some(first_char) = line.chars().next() {
        if first_char == '"' {
            // Very first cell is quote-delimited
            in_cell = false;
        }
    }

    let mut cell = String::new();
    
    for char in line.chars() {
        if char == '\"' {
            if in_cell {
                // Found the end of a quote-delimited cell
                // Note that this means we cannot support quotes inside fields
                in_cell = false;
                in_quoted_cell = false;

                row.push(cell.clone());
                cell = String::new();
            } else {
                // We are now in a quote-delimited cell
                in_cell = true;
                in_quoted_cell = true;
            }
        } else if char == '\n' {
            // Must be the end of the line
            return row;
        } else if char == ',' {
            if in_quoted_cell {
                cell.push(char);
            } else {
                // End of non-quoted cell
                in_cell = false;
                row.push(cell.clone());
                cell = String::new();
            }
        } else {
            cell.push(char);
        }
    }

    // Reached end of line. Push current cell contents.
    row.push(cell.clone());

    row
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
