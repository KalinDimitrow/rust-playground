

pub enum Keys {
    Key1,
    Key2,
    Key3,
    Key4,
}

pub fn convert(key : &Keys) ->u32 {
    match key {
        Keys::Key1 => 1,
        Keys::Key2 => 2,
        Keys::Key3 => 5,
        Keys::Key4 => 10,
    }
}

pub fn print(key : &Keys) {
    match key {
        Keys::Key1 => {print!("spmething");},
        _ =>{}
    };

}