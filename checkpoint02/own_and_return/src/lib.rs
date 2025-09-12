pub struct Film {
    pub name: String,
}

pub fn read_film_name(flm: &Film) -> String {
    return flm.name.clone();
}

pub fn take_film_name(flm: Film) -> String {
    return flm.name;
}
