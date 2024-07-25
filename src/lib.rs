use near_sdk::{env, log, near, serde_json, PromiseError, Gas, GasWeight, CryptoHash};

const RESUMPTION_TOKEN_REGISTER: u64 = 0;

#[near(contract_state)]
pub struct Contract {
    greeting: String,
    resumption_token: CryptoHash
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            greeting: "Hello".to_string(),
            resumption_token: [0; 32],
        }
    }
}

#[near]
impl Contract {
    pub fn set_greeting(&mut self, greeting: String) {
        self.greeting = greeting;
    }

    pub fn get_greeting(&self) -> String {
        self.greeting.clone()
    }

    pub fn update_greeting(&mut self) {
        let yield_promise = env::promise_yield_create(
            "set_greeting",
            &Vec::new(),
            Gas::from_tgas(5),
            GasWeight::default(),
            RESUMPTION_TOKEN_REGISTER,
        );
        let token = if let Some(data) = env::read_register(RESUMPTION_TOKEN_REGISTER)
        {
            if let Ok(token) = CryptoHash::try_from(data) {
                token
            } else {
                env::panic_str("Wrong register length")
            }
        } else {
            env::panic_str("Register is empty")
        };
        self.resumption_token = token;
        env::promise_return(yield_promise);
    }

    pub fn respond(&mut self, new_message: String) {
        // TODO: Check that caller is allowed to respond
        env::promise_yield_resume(&self.resumption_token, &serde_json::to_vec(&new_message).unwrap());
    }
}
