use soroban_sdk::{contractimpl, Bytes, BytesN, Env, Symbol, Address, IntoVal};

#[derive(Clone)]
struct Scholarship {
    donor: Address,
    amount: i32,        
    criteria: Vec<(String, String)>, //  Example: [("GPA", ">3.5"), ("major", "Computer Science")]
}

#[derive(Clone)]
struct Application {
    student: Address,
    scholarship_id: Bytes,
    data: Vec<(String, String)>, // Example: [("GPA", "3.8"), ("major", "Computer Science")] 
}

pub struct ScholarshipContract;

#[contractimpl]
impl ScholarshipContract {
    pub fn create_scholarship(
        e: Env, 
        donor: Address, 
        amount: i32, 
        criteria: Vec<(String, String)>
    ) {
        let scholarship_id = get_next_scholarship_id(&e);
        e.data().set(scholarship_id.clone(), Scholarship { donor, amount, criteria });
 ...
    }

    pub fn apply(e: Env, student: Address, scholarship_id: Bytes, data: Vec<(String, String)>) {
        
        e.data().set(get_application_key(scholarship_id, student), Application { 
            student, 
            scholarship_id, 
            data
        });
    }

    
    pub fn process_applications(e: Env) {
        let scholarships = get_all_scholarships(&e); // Assume a helper exists

        for scholarship in scholarships {
            let applications = get_applications_for_scholarship(&e, &scholarship.scholarship_id);

            for application in applications {
                if is_eligible(&application, &scholarship) {
                    
                    invoke_token_contract(&e, application.student, scholarship.amount); 
                }
            }
        }
    }
}


fn get_all_scholarships(e: &Env) -> Vec<(Bytes, Scholarship)> { ... }

fn get_next_scholarship_id(e: &Env) -> Bytes { ... }

fn get_application_key(scholarship_id: Bytes, student: Address) -> Bytes { ... }

fn get_applications_for_scholarship(e: &Env, scholarship_id: Bytes) -> Vec<(Bytes, Application)> { ... } 

fn is_eligible(application: &Application, scholarship: &Scholarship) -> bool {
    
}

fn invoke_token_contract(e: &Env, to: &Address, amount: i32) {
    
}
