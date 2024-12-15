

fn main() {
    
    // read the input from the file and store it in a string
    let input = std::fs::read_to_string("data/input.txt").unwrap();

    // determine rows and columns
    // this part assumes that the input is a rectangle and all lines have the same length
    // if this is not the case, it should be handled differently
    let rows = input.lines().count();
    let columns = input.lines().next().unwrap().chars().count();

    // create a 2D vector to store the map
    // Can be done with a 1D vector as well, but this is more readable
    // Also because we only care about 4 values (X,M,A,S) 
    // We can represent them as 2 bits each and that would be more efficient but i chose to keep it simple
    let mut map = vec![vec![char::MAX; columns]; rows];

    // fill the map with the input
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            map[i][j] = c;
        }
    }

    
    // count the number of XMAS in the map
    let xmas_count = count_xmases(&map, rows, columns);
    println!("Number of XMAS: {}", xmas_count);

    // count the number of X shaped MAS in the map
    let mas_count = count_x_mases(&map, rows, columns);
    println!("Number of X shaped MAS: {}", mas_count);

}

// Counts the number of XMAS in the map in every possible direction (horizontal, vertical, diagonal, reverse)
fn count_xmases(map: &Vec<Vec<char>>, rows: usize, columns: usize) -> usize {
    let mut xmas_count: usize = 0;

    let mut rsafe: bool;
    let mut lsafe: bool;
    let mut bsafe:bool;

    for i in 0..rows {
        for j in 0..columns {

            // Cheking if we have enough space to check for XMAS in right, left, bottom directions
            rsafe = j<columns-3;
            lsafe = j>2;
            bsafe = i<rows-3;

            // Checks for XMAS in right, right-bottom, left-bottom, bottom directions
            if map[i][j] == 'X' {
                if bsafe && map[i+1][j] == 'M' && map[i+2][j] == 'A' && map[i+3][j] == 'S' {
                    xmas_count += 1;
                }
                if rsafe && bsafe && map[i+1][j+1] == 'M' && map[i+2][j+2] == 'A' && map[i+3][j+3] == 'S' {
                    xmas_count += 1;
                }
                if lsafe && bsafe && map[i+1][j-1] == 'M' && map[i+2][j-2] == 'A' && map[i+3][j-3] == 'S' {
                    xmas_count += 1;
                }
                if rsafe && map[i][j+1] == 'M' && map[i][j+2] == 'A' && map[i][j+3] == 'S' {
                    xmas_count += 1;
                }
            }

            // Checks for SMAX in right, right-bottom, left-bottom, bottom directions
            if map[i][j] == 'S' {
                if bsafe && map[i+1][j] == 'A' && map[i+2][j] == 'M' && map[i+3][j] == 'X' {
                    xmas_count += 1;
                }
                if rsafe && bsafe && map[i+1][j+1] == 'A' && map[i+2][j+2] == 'M' && map[i+3][j+3] == 'X' {
                    xmas_count += 1;
                }
                if lsafe && bsafe && map[i+1][j-1] == 'A' && map[i+2][j-2] == 'M' && map[i+3][j-3] == 'X' {
                    xmas_count += 1;
                }
                if rsafe && map[i][j+1] == 'A' && map[i][j+2] == 'M' && map[i][j+3] == 'X' {
                    xmas_count += 1;
                }
            }
            
        }
    }
    xmas_count
}

// Counts the number of MAS that are in a X shape in the map like this:
//   M  M
//    A
//  S  S
fn count_x_mases(map: &Vec<Vec<char>>, rows: usize, columns: usize) -> usize {
    let mut mas_count: usize = 0;

    // Loop through the maps inner cells and check every A cell for the X shape
    for i in 1..rows-1 {
        for j in 1..columns-1 {
            if map[i][j] == 'A' {

                // Construct two strings for the two diagonals
                let diag1 = map[i-1][j-1].to_string() + &map[i][j].to_string() + &map[i+1][j+1].to_string();
                let diag2 = map[i-1][j+1].to_string() + &map[i][j].to_string() + &map[i+1][j-1].to_string();

                // Check if the diagonals are equal to MAS or SAM
                if (diag1 == "MAS" || diag1 == "SAM") && (diag2 == "MAS" || diag2 == "SAM") {
                    mas_count += 1;
                }
            }
        }
    }
    mas_count
}