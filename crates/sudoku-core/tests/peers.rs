use sudoku_core::peers::{is_peer, PEERS, box_of, col_of, row_of, HOUSE_CELLS};

#[test]
fn every_cell_has_twenty_peers() {
    for cell in 0..81 {
        let mut count = 0;
        for peer in PEERS[cell] {
            if is_peer(cell, peer as usize) {
                count += 1;
            }
        }
        assert_eq!(count, 20, "cell {cell}");
    }
}

#[test]
fn peer_relation_is_symmetric() {
    for a in 0..81 {
        for b in 0..81 {
            assert_eq!(is_peer(a, b), is_peer(b, a), "({a}, {b})");
        }
    }
}

#[test]
fn houses_cover_every_cell_once_per_type() {
    let mut seen = [0u8; 81];
    for house in 0..27 {
        for cell in HOUSE_CELLS[house] {
            seen[cell as usize] += 1;
        }
    }
    for cell in 0..81 {
        assert_eq!(seen[cell], 3, "cell {cell}");
    }
}

#[test]
fn box_index_matches_box_cells() {
    for cell in 0..81 {
        let house = 18 + box_of(cell);
        assert!(HOUSE_CELLS[house].contains(&(cell as u8)));
    }
}

#[test]
fn row_and_col_helpers_agree() {
    for cell in 0..81 {
        let row = row_of(cell);
        let col = col_of(cell);
        assert!(HOUSE_CELLS[row].contains(&(cell as u8)));
        assert!(HOUSE_CELLS[9 + col].contains(&(cell as u8)));
    }
}