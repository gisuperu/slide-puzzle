use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{read, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, SetBackgroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    // Result,
};
use rand::Rng;

fn mat_swap_pop_back<T>(v: &mut Vec<Vec<T>>, i1: usize, j1: usize, i2: usize, j2: usize) {
    let n = v[i1].len();
    v[i1].swap(j1, n - 1);
    if i1 == i2 && j2 == n - 1 {
        return;
    }
    let mut e1 = v[i1].pop().unwrap();
    std::mem::swap(&mut v[i2][j2], &mut e1);
    v[i1].push(e1);
    v[i1].swap(j1, n - 1);
}

fn clear_check(answer : [[i8; 4]; 4], board : Vec<Vec<i8>>) -> bool{
    let mut correct = true;
    for i in 0..4 {
        for j in 0..4 {
            if answer[i][j] != board[i][j]{
                correct = false;
                return correct;
            }
        }
    }
    return correct;
}

fn main() -> std::io::Result<()>{
    let answer : [[i8; 4]; 4] = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, -1]];
    let mut board : Vec<Vec<i8>> = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12], vec![13, 14, 15, -1]];
    let mut cursor  = (3, 3);
    println!("board = {:?}", board);
    // mat_swap_pop_back(&mut board, 0, 0, 3, 3);

    //乱数生成
    let mut rng = rand::thread_rng();
    for _k in 0..1000{
        let r: i32 = rng.gen_range(0..4);
        match r {
            0 => { // Left
                if cursor.1 > 0 {
                    mat_swap_pop_back(&mut board, cursor.0, cursor.1, cursor.0, cursor.1 -1);
                    cursor.1 -= 1;
                }
            }
            1 => { // Up
                
                if cursor.0 > 0 {
                    mat_swap_pop_back(&mut board, cursor.0, cursor.1, cursor.0 -1, cursor.1);
                    cursor.0 -= 1;
                }
            }
            2 => { // Right
                
                if cursor.1 < 3 {
                    mat_swap_pop_back(&mut board, cursor.0, cursor.1, cursor.0, cursor.1 +1);
                    cursor.1 += 1;
                }
            }
            3 => { // Down
                if cursor.0 < 3 {
                    mat_swap_pop_back(&mut board, cursor.0, cursor.1, cursor.0 +1, cursor.1);
                    cursor.0 += 1;
                }
            }
            _ => unreachable!(),
        }
    }
    
    // 端末をRAWモードに変更
    // カーソルの非表示，AlternateScreenに切り替え
    enable_raw_mode()?;
    execute!(std::io::stderr(), Hide, EnterAlternateScreen)?;
    loop {
        execute!(std::io::stderr(), MoveTo(0, 0),)?;
        for i in 0..4 {
            for j in 0..4 {
                if i == cursor.0 && j == cursor.1 {
                    execute!(std::io::stderr(), SetBackgroundColor(Color::Grey))?;
                } else {
                    execute!(std::io::stderr(), SetBackgroundColor(Color::DarkGreen))?;
                }
                match board[i][j]{
                    1  => execute!(std::io::stderr(), Print('1'))?,
                    2  => execute!(std::io::stderr(), Print('2'))?,
                    3  => execute!(std::io::stderr(), Print('3'))?,
                    4  => execute!(std::io::stderr(), Print('4'))?,
                    5  => execute!(std::io::stderr(), Print('5'))?,
                    6  => execute!(std::io::stderr(), Print('6'))?,
                    7  => execute!(std::io::stderr(), Print('7'))?,
                    8  => execute!(std::io::stderr(), Print('8'))?,
                    9  => execute!(std::io::stderr(), Print('9'))?,
                    10 => execute!(std::io::stderr(), Print('A'))?,
                    11 => execute!(std::io::stderr(), Print('B'))?,
                    12 => execute!(std::io::stderr(), Print('C'))?,
                    13 => execute!(std::io::stderr(), Print('D'))?,
                    14 => execute!(std::io::stderr(), Print('E'))?,
                    15 => execute!(std::io::stderr(), Print('F'))?,
                    _  => execute!(std::io::stderr(), Print(' '))?,
                }
                
                // if board[i][j] > 0 && board[i][j] < 16 {
                //     execute!(std::io::stderr(), Print(board[i][j]))?;
                // } else {
                //     execute!(std::io::stderr(), Print(' '))?;
                // }
            }
            execute!(std::io::stderr(), Print("\r\n"))?;
        }
        match read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => break,
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                ..
            }) => {
                if cursor.1 > 0 {

                    mat_swap_pop_back(&mut board, cursor.0, cursor.1, cursor.0, cursor.1 -1);
                    cursor.1 -= 1;
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Up, ..
            }) => {
                if cursor.0 > 0 {
                    mat_swap_pop_back(&mut board, cursor.0, cursor.1, cursor.0 -1, cursor.1);
                    cursor.0 -= 1;
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                ..
            }) => {
                if cursor.1 < 3 {
                    mat_swap_pop_back(&mut board, cursor.0, cursor.1, cursor.0, cursor.1 +1);
                    cursor.1 += 1;
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            }) => {
                if cursor.0 < 3 {
                    mat_swap_pop_back(&mut board, cursor.0, cursor.1, cursor.0 +1, cursor.1);
                    cursor.0 += 1;
                }
            }
            _ => continue,
        }
        if clear_check(answer, board.clone()) {
            break;
        }
    }
    execute!(std::io::stderr(), Show, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    return Ok(());
}
