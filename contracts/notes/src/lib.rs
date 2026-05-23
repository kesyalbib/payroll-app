#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Address, Env,
    Symbol, Vec, String,
};

// =========================
// STRUCT DATA
// =========================

#[contracttype]
#[derive(Clone, Debug)]
pub struct Employee {
    pub id: u64,
    pub worker: Address,
    pub name: String,
    pub hourly_rate: i128, // contoh: 10 USDC per jam
    pub total_hours: u64,
    pub balance: i128,
}

// =========================
// STORAGE KEY
// =========================

const EMPLOYEE_DATA: Symbol = symbol_short!("EMPLOYEE");

// =========================
// CONTRACT
// =========================

#[contract]
pub struct PayrollContract;

#[contractimpl]
impl PayrollContract {

    // =========================
    // GET ALL EMPLOYEES
    // =========================
    pub fn get_employees(env: Env) -> Vec<Employee> {
        env.storage()
            .instance()
            .get(&EMPLOYEE_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // =========================
    // ADD EMPLOYEE
    // =========================
    pub fn add_employee(
        env: Env,
        worker: Address,
        name: String,
        hourly_rate: i128,
    ) -> String {

        let mut employees: Vec<Employee> = env.storage()
            .instance()
            .get(&EMPLOYEE_DATA)
            .unwrap_or(Vec::new(&env));

        let employee = Employee {
            id: env.prng().gen::<u64>(),
            worker,
            name,
            hourly_rate,
            total_hours: 0,
            balance: 0,
        };

        employees.push_back(employee);

        env.storage().instance().set(&EMPLOYEE_DATA, &employees);

        String::from_str(&env, "Employee berhasil ditambahkan")
    }

    // =========================
    // RECORD WORK HOURS
    // =========================
    pub fn record_work(
        env: Env,
        employee_id: u64,
        hours: u64,
    ) -> String {

        let mut employees: Vec<Employee> = env.storage()
            .instance()
            .get(&EMPLOYEE_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..employees.len() {

            let mut employee = employees.get(i).unwrap();

            if employee.id == employee_id {

                // tambah jam kerja
                employee.total_hours += hours;

                // hitung salary otomatis
                let payment =
                    employee.hourly_rate * hours as i128;

                // tambah balance
                employee.balance += payment;

                // update data
                employees.set(i, employee);

                // simpan kembali
                env.storage()
                    .instance()
                    .set(&EMPLOYEE_DATA, &employees);

                return String::from_str(
                    &env,
                    "Jam kerja berhasil dicatat"
                );
            }
        }

        String::from_str(&env, "Employee tidak ditemukan")
    }

    // =========================
    // WITHDRAW SALARY
    // =========================
    pub fn withdraw_salary(
        env: Env,
        employee_id: u64,
    ) -> String {

        let mut employees: Vec<Employee> = env.storage()
            .instance()
            .get(&EMPLOYEE_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..employees.len() {

            let mut employee = employees.get(i).unwrap();

            if employee.id == employee_id {

                if employee.balance <= 0 {
                    return String::from_str(
                        &env,
                        "Saldo kosong"
                    );
                }

                // simulasi transfer otomatis
                // nanti bisa diintegrasikan dengan token USDC Stellar

                employee.balance = 0;

                employees.set(i, employee);

                env.storage()
                    .instance()
                    .set(&EMPLOYEE_DATA, &employees);

                return String::from_str(
                    &env,
                    "Gaji berhasil ditarik"
                );
            }
        }

        String::from_str(&env, "Employee tidak ditemukan")
    }

    // =========================
    // DELETE EMPLOYEE
    // =========================
    pub fn delete_employee(
        env: Env,
        employee_id: u64,
    ) -> String {

        let mut employees: Vec<Employee> = env.storage()
            .instance()
            .get(&EMPLOYEE_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..employees.len() {

            if employees.get(i).unwrap().id == employee_id {

                employees.remove(i);

                env.storage()
                    .instance()
                    .set(&EMPLOYEE_DATA, &employees);

                return String::from_str(
                    &env,
                    "Employee berhasil dihapus"
                );
            }
        }

        String::from_str(&env, "Employee tidak ditemukan")
    }
}

mod test;