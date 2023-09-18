trait Person {
    fn name(&self) -> String;
}

trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

trait ComSciStudent: Student + Programmer {}

struct StudentIml {}

impl Student for StudentIml {
    fn university(&self) -> String {
        String::from("StudentIml university")
    }
}

impl Person for StudentIml {
    fn name(&self) -> String {
        String::from("StudentIml name")
    }
}

impl Programmer for StudentIml {
    fn fav_language(&self) -> String {
        String::from("rust")
    }
}

impl ComSciStudent for StudentIml {}

fn programmer_report(p: &dyn Programmer) -> String {
    return p.fav_language();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let st = StudentIml {};
        println!("{}", programmer_report(&st));
    }
}
