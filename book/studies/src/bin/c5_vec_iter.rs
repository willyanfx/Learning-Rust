use std::cmp::Ordering;

#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, Debug)]
struct Contact {
    name: &'static str,
    phone_number: &'static str,
}

impl Ord for Contact {
    fn cmp(&self, other: &Contact) -> Ordering {
        (self.name).cmp(&(other.name))
    }
}

fn agenda() {
    let contact_1 = Contact {
        name: "Marcelo",
        phone_number: "+55(11) 9.1234.5678",
    };
    let contact_2 = Contact {
        name: "Will",
        phone_number: "+55(49) 9.8765.4321",
    };
    let contact_3 = Contact {
        name: "Raphaela",
        phone_number: "+55(49) 9.8765.4322",
    };
    let mut agenda = vec![];
    agenda.push(contact_1);
    agenda.push(contact_2);
    agenda.push(contact_3);

    // println!("\nAgenda has 2 contacts \n {:?}", agenda);

    // agenda.pop();
    // println!("\nAgenda has 1 contacts \n {:?}", agenda);

    // println!("First contact: {:?}", agenda[0]);

    // for contact in &agenda {
    //     println!("{:?}", contact);
    // }

    // let mut agenda_iter = agenda.iter();

    // loop {
    //     let item = agenda_iter.next();
    //     if let Some(contact) = item {
    //         println!("loop: {:?}", contact);
    //     } else {
    //         break;
    //     }
    // }

    let names = agenda
        .iter()
        .map(|contact| contact.name)
        .collect::<Vec<_>>();
    println!("Nomes: {:?}", names);
}

fn agenda2() {
    let c1 = Contact {
        name: "Kwame",
        phone_number: "+55(11) 9.1234.5678",
    };
    let c2 = Contact {
        name: "Gi",
        phone_number: "+55(11) 9.8765.4321",
    };
    let c3 = Contact {
        name: "Joey Wheeler",
        phone_number: "+55(11) 9.9999.8888",
    };
    let c4 = Contact {
        name: "Linka",
        phone_number: "+55(11) 9.4567.7654",
    };
    let c5 = Contact {
        name: "Ma-Ti",
        phone_number: "+55(11) 9.4567.7654",
    };

    let mut agenda = vec![c1, c2, c3, c4, c5];

    agenda.sort();

    println!("{:?}", agenda);
}

fn main() {
    // agenda();
    agenda2();
}
